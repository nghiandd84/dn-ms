# 1. Download the zip file (using a common version, e.g., 1.18.0)
CONSUL_VERSION="1.20.0" # You can update this to the latest stable version
wget "https://releases.hashicorp.com/consul/${CONSUL_VERSION}/consul_${CONSUL_VERSION}_linux_amd64.zip"

# 2. Unzip the file
sudo apt-get install unzip
unzip "consul_${CONSUL_VERSION}_linux_amd64.zip"

# 3. Move the binary to a directory in your PATH (e.g., /usr/local/bin)
sudo mv consul /usr/local/bin/

# 4. Clean up the downloaded files
rm "consul_${CONSUL_VERSION}_linux_amd64.zip"

# Run the following command to ensure Consul is correctly installed and in your path:
consul --version

2. Configure Consul as a Service (Systemd)
# Create the Consul configuration directory
sudo mkdir -p /etc/consul.d

# Create the data directory
sudo mkdir -p /opt/consul/data

# Create a dedicated user for Consul for security best practices
sudo useradd --system --home /etc/consul.d --no-create-home --shell /bin/false consul

# Set ownership of the data directory to the consul user
sudo chown -R consul:consul /opt/consul

Step 4: Create the Configuration File

sudo nano /etc/consul.d/consul.hcl

Paste the following configuration:
```
# /etc/consul.d/consul.hcl

datacenter = "wsl-dc1"
data_dir = "/opt/consul/data"
client_addr = "0.0.0.0" # Allows access from Windows (localhost)
ui = true # Enables the web UI
server = true # Run the agent in server mode
bootstrap_expect = 1 # Start a single-node cluster
performance {
  raft_multiplier = 1 # Better responsiveness in a development environment
}
```

Step 5: Create the Systemd Service File

sudo nano /etc/systemd/system/consul.service

```
[Unit]
Description=Consul Agent
Requires=network-online.target
After=network-online.target

[Service]
User=consul
Group=consul
PIDFile=/run/consul.pid
ExecStart=/usr/local/bin/consul agent -config-dir=/etc/consul.d -dev
ExecReload=/bin/kill -HUP $MAINPID
KillSignal=SIGINT
TimeoutStopSec=5
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
```

Step 6: Reload and Start the Service

# Reload the systemd configuration
sudo systemctl daemon-reload

# Enable Consul to start on boot
sudo systemctl enable consul

# Start the Consul service
sudo systemctl start consul