# Install Consul
```
# Install dependencies
sudo apt-get update && sudo apt-get install -y gnupg software-properties-common curl

# Add the HashiCorp GPG key
curl -fsSL https://apt.releases.hashicorp.com/gpg | sudo gpg --dearmor -o /usr/share/keyrings/hashicorp-archive-keyring.gpg

# Add the official repository
echo "deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/hashicorp.list

# Update and install
sudo apt-get update && sudo apt-get install consul
```

# Start Consul in "Dev Mode" (Quickest Start)
```
consul agent -dev
```

# Configure as a "Production-Style" Server
Create the Config File
Create a file at /etc/consul.d/server.hcl:
```
# Basic Server Config
server = true
bootstrap_expect = 1  # For local testing, we only expect 1 server
data_dir = "/opt/consul"
log_level = "INFO"

# Networking
bind_addr = "0.0.0.0" # Listen on all interfaces
client_addr = "0.0.0.0"

# Enable the Web UI
ui_config {
  enabled = true
}

# Connect (Service Mesh) - useful for your Rust services later
connect {
  enabled = true
}
```

Setup Permissions and Start
```
# Ensure the data directory exists
sudo mkdir -p /opt/consul
sudo chown -R consul:consul /opt/consul

# Start the agent with the config
sudo consul agent -config-dir=/etc/consul.d
```

Config WSL as service

# Step 1. Enable systemd in WSL

Open your WSL terminal and run:
```
sudo nano /etc/wsl.conf
```
Add this line
```
[boot]
systemd=true
```
# Step 2. Create a Dedicated Consul User
```
sudo groupadd --system consul
sudo useradd -s /sbin/nologin --system -g consul consul
```

# Step 3: Set up Directories & Configuration
1. Create directories:
```
sudo mkdir -p /etc/consul.d
sudo mkdir -p /var/lib/consul
sudo chown -R consul:consul /etc/consul.d /var/lib/consul
```
2. Create the configuration file:
```
sudo nano /etc/consul.d/consul.hcl
```

# Step 4: Create the systemd Service File
Create the service unit file:
```
sudo nano /etc/systemd/system/consul.service
```
Paste the following content:
```
[Unit]
Description="HashiCorp Consul - A network service mesh"
Documentation=https://www.consul.io/
Requires=network-online.target
After=network-online.target

[Service]
User=consul
Group=consul
ExecStart=/usr/bin/consul agent -config-dir=/etc/consul.d/
ExecReload=/bin/kill --signal HUP $MAINPID
KillMode=process
Restart=on-failure
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
```

# Step 5: Start and Enable
```
# Reload the systemd manager to see the new service
sudo systemctl daemon-reload

# Enable it to start on boot
sudo systemctl enable consul

# Start it now
sudo systemctl start consul
```
Verify it's running
```
sudo systemctl status consul
```