# Start postgres database
sudo service postgresql start

# Start redis server . Password: Redis@123
sudo service redis-server start

#Kafka start . Password: kafka/Kafka@123
sudo service zookeeper start

sleep 10s

sudo service kafka start

