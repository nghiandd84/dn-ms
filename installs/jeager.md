1. Download
# Set the version (e.g., v1.75.0 - check for the current latest)
JAEGER_VERSION="1.75.0" 
JAEGER_TAR="jaeger-${JAEGER_VERSION}-linux-amd64.tar.gz"

# Download the file
wget https://github.com/jaegertracing/jaeger/releases/download/v${JAEGER_VERSION}/${JAEGER_TAR}

# Extract the archive
tar -xvzf ${JAEGER_TAR}

# Move the executable to a standard binary path
sudo mv jaeger-${JAEGER_VERSION}-linux-amd64/jaeger-all-in-one /usr/local/bin/jaeger-all-in-one

# Make sure it's executable
sudo chmod +x /usr/local/bin/jaeger-all-in-one

2. Create the Systemd Service File

# Create file
sudo nano /etc/systemd/system/jaeger-all-in-one.service
```
[Unit]
Description=Jaeger All-in-One Tracing Service
After=network.target

[Service]
# The user/group to run as (optional, for security)
User=nghiandd
Group=nghiandd

# The executable command and arguments
# --collector.otlp.enabled=true enables the OTLP receiver (Port 4317 gRPC, 4318 HTTP)
# The Jaeger UI is available at port 16686
ExecStart=/usr/local/bin/jaeger-all-in-one --collector.otlp.enabled=true 

# Restart the service if it fails
Restart=always

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

3. Enable and Start the Service 
# Reload the systemd manager configuration:
sudo systemctl daemon-reload

# Enable the service (to start on boot):
sudo systemctl enable jaeger-all-in-one
```
Created symlink /etc/systemd/system/multi-user.target.wants/jaeger-all-in-one.service → /etc/systemd/system/jaeger-all-in-one.service.
```

# Start the service now:
sudo systemctl start jaeger-all-in-one

# Check the status and logs:
sudo systemctl status jaeger-all-in-one
sudo journalctl -u jaeger-all-in-one -f

4. Verification

http://localhost:16686 (Jaeger Web UI)
localhost:4317

