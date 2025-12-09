1. Download
# Update package list
sudo apt-get update

# Install wget if you don't have it
sudo apt-get install -y wget

# Download the latest CONTRIB version (Example uses v0.130.1 - replace with the latest)
COLLECTOR_VERSION=0.130.1 
wget https://github.com/open-telemetry/opentelemetry-collector-releases/releases/download/v${COLLECTOR_VERSION}/otelcol-contrib_${COLLECTOR_VERSION}_linux_amd64.deb
# https://github.com/open-telemetry/opentelemetry-collector-releases/releases/download/v0.130.1/otelcol-contrib_0.130.1_linux_amd64.deb

2. Install 
# Install the package
sudo dpkg -i otelcol-contrib_${COLLECTOR_VERSION}_linux_amd64.deb
```
Selecting previously unselected package otelcol-contrib.
(Reading database ... 77579 files and directories currently installed.)
Preparing to unpack otelcol-contrib_0.130.1_linux_amd64.deb ...
Unpacking otelcol-contrib (0.130.1) ...
Setting up otelcol-contrib (0.130.1) ..
Created symlink /etc/systemd/system/multi-user.target.wants/otelcol-contrib.service → /lib/systemd/system/otelcol-contrib.service.
```

# Fix any potential dependency issues
sudo apt-get install -f

3. Configure the Collector
# Open the configuration file for editing (e.g., using nano)
sudo nano /etc/otelcol-contrib/config.yaml


# Start service
sudo systemctl start otelcol-contrib

# Enable auto start
sudo systemctl enable otelcol-contrib

# Check status
sudo systemctl status otelcol-contrib

# View logs
sudo journalctl -u otelcol-contrib -f

# Restart
sudo systemctl restart otelcol-contrib