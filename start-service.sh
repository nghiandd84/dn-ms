# Start postgres database
sudo service postgresql start

# Start redis server . Password: Redis@123
sudo service redis-server start

#Kafka start . Password: kafka/Kafka@123
sudo service zookeeper start
sleep 5s
sudo service kafka start
sleep 5s
# Create topic if not exists
#/home/kafka/kafka/bin/kafka-topics.sh --create --topic notification_topic --bootstrap-server localhost:9092 --if-not-exists
# Send message to topic
#/home/kafka/kafka/bin/kafka-console-producer.sh --topic notification_topic --bootstrap-server localhost:9092 
# {"event_type":"depositSuccess", "user_id": "3158787f-7b76-4b04-b79d-4d8fac17d841", "platform": "Platform1"}
#/home/kafka/kafka/bin/kafka-topics.sh  --describe notification_topic --bootstrap-server localhost:9092 
