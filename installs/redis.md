# Step 1: Install Redis
```
# Add the official Redis GPG key and repository
curl -fsSL https://packages.redis.io/gpg | sudo gpg --dearmor -o /usr/share/keyrings/redis-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/redis-archive-keyring.gpg] https://packages.redis.io/deb $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/redis.list

# Update and install
sudo apt-get update
sudo apt-get install redis-server
```

# Step 2: Configure for systemd
Open file
```
sudo nano /etc/redis/redis.conf
```

Find the supervised directive (use Ctrl+W to search). Change it from no to systemd:
```
supervised systemd
```

(Optional) If you want to access Redis from Windows tools (like Redis Insight), find the bind line and change it to:
```
bind 0.0.0.0
```

Warning: If you bind to 0.0.0.0, set a password by uncommenting # requirepass foobared and changing "foobared" to something secure.

# Step 3: Start and Enable the Service
```
# Start the service
sudo systemctl start redis-server

# Enable it to start whenever WSL starts
sudo systemctl enable redis-server

# Verify it is healthy
sudo systemctl status redis-server
```
