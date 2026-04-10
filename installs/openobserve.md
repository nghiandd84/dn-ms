# Step 1: Download the Binary
First, we'll download the Linux binary and place it in a standard location.

```
# Create a directory for the binary
sudo mkdir -p /usr/local/bin/openobserve

# Download the latest version (AMD64)
# Check link at https://openobserve.ai/downloads/
# Note: If you're on an ARM-based Windows machine, use the 'arm64' link instead.
curl -L -o openobserve.tar.gz https://downloads.openobserve.ai/releases/o2-enterprise/latest/openobserve-ee-linux-amd64.tar.gz

# Extract and move it
tar -xzf openobserve.tar.gz
sudo mv openobserve /usr/local/bin/openobserve-bin
chmod +x /usr/local/bin/openobserve-bin
```

# Step 2: Create a Configuration File
We need to store your credentials and data path in a safe spot.
```
sudo mkdir -p /etc/openobserve
sudo nano /etc/openobserve/openobserve.env
```

Paste the following into the file (change the password to something secure!):
```
ZO_ROOT_USER_EMAIL=admin@example.com
ZO_ROOT_USER_PASSWORD=Tes@789
ZO_DATA_DIR=/var/lib/openobserve
```

# Step 3: Create the Systemd Service

```
sudo nano /etc/systemd/system/openobserve.service
```

```
[Unit]
Description=OpenObserve Server
After=network.target

[Service]
Type=simple
LimitNOFILE=65535
EnvironmentFile=/etc/openobserve/openobserve.env
ExecStart=/usr/local/bin/openobserve-bin
Restart=on-failure
# Ensure the data directory exists and is writable
StateDirectory=openobserve

[Install]
WantedBy=multi-user.target
```

# Step 4: Start and Enable the Service
```
# Reload systemd to recognize the new service
sudo systemctl daemon-reload

# Start the service
sudo systemctl start openobserve

# Enable it to start automatically when WSL starts
sudo systemctl enable openobserve
```

Check status
```
sudo systemctl status openobserve
```

http://localhost:5080