# Step 1: Install Java (Prerequisite)
```
sudo apt update
sudo apt install default-jdk -y
```

# Step 2: Download and Extract Kafka
We will install it in /opt/kafka.

```
# Download the latest binary (Check kafka.apache.org for newer versions if needed)
wget https://archive.apache.org/dist/kafka/3.7.0/kafka_2.13-3.7.0.tgz

# Extract and move to /opt
tar -xzf kafka_2.13-3.7.0.tgz
sudo mv kafka_2.13-3.7.0 /opt/kafka
```

# Step 3: Configure Kafka with KRaft
KRaft allows Kafka to run without a separate ZooKeeper service.

Generate a Cluster ID:
```
KAFKA_CLUSTER_ID=$(/opt/kafka/bin/kafka-storage.sh random-uuid)
```

Format the Storage Directory:
```
/opt/kafka/bin/kafka-storage.sh format -t $KAFKA_CLUSTER_ID -c /opt/kafka/config/kraft/server.properties
```

# Step 4: Create the systemd Service
To keep Kafka running in the background

Create the file:
```
sudo nano /etc/systemd/system/kafka.service
```

Paste the following:
```
[Unit]
Description=Apache Kafka Server
Documentation=http://kafka.apache.org/documentation.html
Requires=network.target
After=network.target

[Service]
Type=simple
User=root
ExecStart=/opt/kafka/bin/kafka-server-start.sh /opt/kafka/config/kraft/server.properties
ExecStop=/opt/kafka/bin/kafka-server-stop.sh
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

# Step 5: Start and Enable
```
sudo systemctl daemon-reload
sudo systemctl start kafka
sudo systemctl enable kafka
```

Verify 
```
sudo systemctl status kafka
```