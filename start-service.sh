# Start postgres database
sudo service postgresql start

# Start redis server . Password: Redis!123
sudo service redis-server start

#Kafka start . Password: kafka/Kafka@123
sudo service zookeeper start
sleep 5s
sudo service kafka start
sleep 5s
sudo service consul start
sleep 5s
# Connect consul UI
# http://localhost:8500/ui
# Create topic if not exists
#/home/kafka/kafka/bin/kafka-topics.sh --create --topic notification_topic --bootstrap-server localhost:9092 --if-not-exists
# Send message to topic
#/home/kafka/kafka/bin/kafka-console-producer.sh --topic notification_topic --bootstrap-server localhost:9092 
# {"event_type":"depositSuccess", "user_id": "3158787f-7b76-4b04-b79d-4d8fac17d841", "platform": "Platform1"}
# {"message_type":"notification", "user_id": "3158787f-7b76-4b04-b79d-4d8fac17d841", "message": "My Message"}
#/home/kafka/kafka/bin/kafka-topics.sh  --describe notification_topic --bootstrap-server localhost:9092 
#/home/kafka/kafka/bin/kafka-topics.sh   --bootstrap-server localhost:9092  --topic notification_topic --delete
# Get IP Address of WSL 
# hostname -I
# Create proxy on window to connect to kafka
# netsh interface portproxy add v4tov4 listenport=9092 listenaddress=0.0.0.0 connectport=9092 connectaddress=172.25.43.223
# TODO implement service discovery https://medium.com/@patrickkoss/how-to-build-a-service-discovery-and-leader-election-with-zookeeper-in-rust-1fcffd9c889d