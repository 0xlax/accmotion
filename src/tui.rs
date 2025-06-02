use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Dataset, Chart, GraphType, Axis},
    Terminal,
};
use std::{
    error::Error,
    io,
    sync::mpsc,
    time::{Duration, Instant},
};
use ringbuffer::{RingBuffer, AllocRingBuffer};

// Data structure to hold our accelerometer readings
#[derive(Clone, Copy)]
pub struct AccelReading {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub timestamp: Instant,
}

// App state
pub struct App {
    rx: mpsc::Receiver<AccelReading>,
    readings: AllocRingBuffer<AccelReading>,
    should_quit: bool,
}

impl App {
    pub fn new(rx: mpsc::Receiver<AccelReading>) -> App {
        App {
            rx,
            readings: AllocRingBuffer::new(100), // Keep last 100 readings
            should_quit: false,
        }
    }

    pub fn update(&mut self) {
        // Receive all pending readings
        while let Ok(reading) = self.rx.try_recv() {
            self.readings.push(reading);
        }
    }
}

pub fn run_tui(rx: mpsc::Receiver<AccelReading>) -> Result<(), Box<dyn Error>> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new(rx);
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33); // ~30 fps

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    app.should_quit = true;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame<'_>, app: &App) {
    let size = f.size();
    
    // Create three sections for x, y, z gauges
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // X gauge
            Constraint::Length(3),  // Y gauge
            Constraint::Length(3),  // Z gauge
            Constraint::Min(10),    // Graph
        ])
        .split(size);

    // Helper function to normalize accelerometer values to 0-100 range
    let normalize = |v: f32| ((v + 20.0) / 40.0 * 100.0).clamp(0.0, 100.0);

    // Get the latest reading
    let latest = app.readings.iter().last().copied().unwrap_or(AccelReading {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        timestamp: Instant::now(),
    });

    // Create gauges for each axis
    let x_gauge = Gauge::default()
        .block(Block::default().title("X-Axis").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio((normalize(latest.x) / 100.0) as f64);

    let y_gauge = Gauge::default()
        .block(Block::default().title("Y-Axis").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio((normalize(latest.y) / 100.0) as f64);

    let z_gauge = Gauge::default()
        .block(Block::default().title("Z-Axis").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .ratio((normalize(latest.z) / 100.0) as f64);

    f.render_widget(x_gauge, chunks[0]);
    f.render_widget(y_gauge, chunks[1]);
    f.render_widget(z_gauge, chunks[2]);

    // Create chart data
    let start_time = app.readings.iter().next().map(|r| r.timestamp).unwrap_or_else(Instant::now);
    let x_data: Vec<(f64, f64)> = app.readings.iter()
        .map(|r| (r.timestamp.duration_since(start_time).as_secs_f64(), r.x as f64))
        .collect();
    let y_data: Vec<(f64, f64)> = app.readings.iter()
        .map(|r| (r.timestamp.duration_since(start_time).as_secs_f64(), r.y as f64))
        .collect();
    let z_data: Vec<(f64, f64)> = app.readings.iter()
        .map(|r| (r.timestamp.duration_since(start_time).as_secs_f64(), r.z as f64))
        .collect();

    let datasets = vec![
        Dataset::default()
            .name("X")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Red))
            .data(&x_data),
        Dataset::default()
            .name("Y")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Green))
            .data(&y_data),
        Dataset::default()
            .name("Z")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Blue))
            .data(&z_data),
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Motion History").borders(Borders::ALL))
        .x_axis(Axis::default().title("Time (s)"))
        .y_axis(Axis::default().title("Acceleration"));

    f.render_widget(chart, chunks[3]);
}
