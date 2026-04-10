# Step 1: Install otelcol-contrib
```
# Define version (using 0.96.0 as a stable 2026-era baseline)
OTEL_VERSION="0.96.0"

# Download the DEB package
wget https://github.com/open-telemetry/opentelemetry-collector-releases/releases/download/v${OTEL_VERSION}/otelcol-contrib_${OTEL_VERSION}_linux_amd64.deb

# Install it
sudo dpkg -i otelcol-contrib_${OTEL_VERSION}_linux_amd64.deb
```

# Step 2: Configure Integration with Jaeger
Open the config file
```
sudo nano /etc/otelcol-contrib/config.yaml
```

Replace the file content with this bridge configuration:
```
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4319
      http:
        endpoint: 0.0.0.0:4320

processors:
  batch:
  memory_limiter:
    check_interval: 1s
    limit_mib: 512

exporters:
  # This points to your Jaeger v2 instance
  otlp/jaeger:
    endpoint: "localhost:4317" 
    tls:
      insecure: true

  # Useful for debugging: prints spans to your console logs
  debug:
    verbosity: normal

service:
  pipelines:
    traces:
      receivers: [otlp]
      processors: [memory_limiter, batch]
      exporters: [otlp/jaeger, debug]
```

# Step 3: Start and Enable the Collector
```
# Enable the service
sudo systemctl enable otelcol-contrib

# Start the service
sudo systemctl start otelcol-contrib

# Verify it is healthy
sudo systemctl status otelcol-contrib
```

# Step 4: Verify the Data Flow
To confirm everything is connected:
Check the Collector logs: sudo journalctl -u otelcol-contrib -f
Check the Jaeger UI: http://localhost:16686