// static/script.js
document.addEventListener('DOMContentLoaded', () => {
    const xVal = document.getElementById('xVal');
    const yVal = document.getElementById('yVal');
    const zVal = document.getElementById('zVal');
    const statusEl = document.getElementById('status');
    const permissionButton = document.getElementById('permissionButton');

    function handleMotion(event) {
        const acc = event.accelerationIncludingGravity;
        if (acc && acc.x !== null && acc.y !== null && acc.z !== null) {
            const data = {
                x: acc.x,
                y: acc.y,
                z: acc.z
            };

            // Update UI
            xVal.textContent = data.x.toFixed(2);
            yVal.textContent = data.y.toFixed(2);
            zVal.textContent = data.z.toFixed(2);

            // ✅ Sends { x, y, z } via POST /motion as JSON
            fetch('/motion', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(data),
            })
            .then(response => {
                if (!response.ok) {
                    statusEl.textContent = `Error sending data: ${response.statusText}`;
                    console.error('Error sending data:', response);
                } else {
                    statusEl.textContent = 'Data sent successfully!';
                }
            })
            .catch(error => {
                statusEl.textContent = `Fetch error: ${error}`;
                console.error('Fetch error:', error);
            });
        } else {
            statusEl.textContent = 'Accelerometer data not available or incomplete.';
        }
    }

    function requestMotionPermission() {
        // For iOS 13+
        if (typeof DeviceMotionEvent !== 'undefined' && typeof DeviceMotionEvent.requestPermission === 'function') {
            DeviceMotionEvent.requestPermission()
                .then(permissionState => {
                    if (permissionState === 'granted') {
                        statusEl.textContent = 'Permission granted. Listening for motion...';
                        window.addEventListener('devicemotion', handleMotion);
                    } else {
                        statusEl.textContent = 'Permission not granted.';
                    }
                    permissionButton.style.display = 'none'; // Hide button after attempt
                })
                .catch(error => {
                    statusEl.textContent = `Permission error: ${error}`;
                    console.error(error);
                    permissionButton.style.display = 'none';
                });
        } else {
            // For other browsers or older iOS
            statusEl.textContent = 'Listening for motion (no specific permission needed or available)...';
            window.addEventListener('devicemotion', handleMotion);
            permissionButton.style.display = 'none'; // Hide button as it's not applicable
        }
    }

    // ✅ JS reads devicemotion events from phone
    if (window.DeviceMotionEvent) {
        permissionButton.addEventListener('click', requestMotionPermission);
        statusEl.textContent = 'Click button to request motion permission.';
    } else {
        statusEl.textContent = 'DeviceMotionEvent not supported by this browser.';
        permissionButton.style.display = 'none';
    }
});