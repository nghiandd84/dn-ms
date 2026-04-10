# Step 1: Download Jaeger v2

```
# Create directory
sudo mkdir -p /opt/jaeger

# Download the latest Linux binary (v2.17.0 as of early 2026)
wget https://github.com/jaegertracing/jaeger/releases/download/v2.17.0/jaeger-2.17.0-linux-amd64.tar.gz

# Extract and move the binary
tar -xzf jaeger-2.17.0-linux-amd64.tar.gz
sudo mv jaeger-2.17.0-linux-amd64/jaeger /usr/bin/jaeger
```

# Step 2: Create the systemd Service
Create file
```
sudo nano /etc/systemd/system/jaeger.service
```

Paste the following configuration:
```
[Unit]
Description=Jaeger v2 All-in-One Tracing
Documentation=https://www.jaegertracing.io/
Wants=network-online.target
After=network-online.target

[Service]
User=root
# In Jaeger v2, we use --set to configure internal components
ExecStart=/usr/bin/jaeger --set=extensions.jaeger_storage.backends.primary_store.memory.max_traces=50000
Restart=on-failure
# Removed obsolete Syslog settings; systemd now handles this via journald automatically
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

# Step 3: Start and Verify
```
sudo systemctl daemon-reload
sudo systemctl enable jaeger
sudo systemctl start jaeger

# Verify it is healthy
sudo systemctl status jaeger
```