# 1. Install PostgreSQL
```
sudo apt update
sudo apt install postgresql postgresql-contrib
```

# 2. Start and Enable the Service
```
# Start the Postgres service
sudo systemctl start postgresql

# Enable it to start whenever WSL starts
sudo systemctl enable postgresql

# Verify it is running
sudo systemctl status postgresql
```

# 3. Set a Password for the 'postgres' User
```
sudo -u postgres psql
```

```
ALTER USER postgres PASSWORD 'password123';
```