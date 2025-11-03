#!/bin/bash

# Add permission 
# sudo chmod +x start.sh

echo "Start DN-MS"

echo "Argument 1 is $1"

APP_DIRECTORY=./target/debug
LOG_DIRECTORY=./target/logs
CURRENT_DATE=$(date +%Y%m%d)

export AUTH_PORT=5101
export BAKERY_PORT=5201
export EMAIL_TEMPLATE_PORT=5301
export NOTIFICATION_PORT=5401

export NOTIFICATION_APP_PORT=4001

echo "Kill current instances"

# Kill Auth port
for i in {1..3}; do 
    fuser -k -15 510$i/tcp 
done

# Kill Bakery port
for i in {1..3}; do 
    fuser -k -15 520$i/tcp 
done


# Kill Email Template port
for i in {1..3}; do 
    fuser -k -15 530$i/tcp 
done

# Kill Notification port
for i in {1..3}; do 
    fuser -k -15 540$i/tcp 
done

# Kill Notification App port
for i in {1..3}; do 
    fuser -k -15 400$i/tcp 
done

echo "Sucess kill all instances"

rm -v $LOG_DIRECTORY/*

    
if [ "$1" == "kill" ]; then
    exit 0  # Exit after killing processes
fi


echo "------------ Start Auth API ------------"
for i in {1..3}; do
    PORT=510$i
    echo "--- Auth on port $PORT ---"
    # Execute the program
    AUTH_PORT=510$i $APP_DIRECTORY/api-auth > $LOG_DIRECTORY/api-auth-${CURRENT_DATE}-instance-$i.log &
done


echo "------------ Start Bakery API ------------"
for i in {1..3}; do
    PORT=520$i
    echo "--- Bakery on port $PORT ---"
    # Execute the program
    BAKERY_PORT=520$i $APP_DIRECTORY/api-bakery > $LOG_DIRECTORY/api-bakery-${CURRENT_DATE}-instance-$i.log &
done


echo "------------ Start Email Template API ------------"
for i in {1..3}; do
    PORT=530$i
    echo "--- Email Template on port $PORT ---"
    # Execute the program
    EMAIL_TEMPLATE_PORT=530$i $APP_DIRECTORY/api-email-template > $LOG_DIRECTORY/api-email-template-${CURRENT_DATE}-instance-$i.log &
done

echo "------------ Start Notification API ------------"
for i in {1..3}; do
    PORT=540$i
    echo "--- Notification API on port $PORT ---"
    # Execute the program
    NOTIFICATION_PORT=540$i $APP_DIRECTORY/api-notification > $LOG_DIRECTORY/api-notification-${CURRENT_DATE}-instance-$i.log &
done

echo "------------ Start Notification App ------------"
for i in {1..3}; do
    PORT=400$i
    echo "--- Notification App on port $PORT ---"
    # Execute the program
    NOTIFICATION_APP_PORT=400$i INSTANCE_ID=$i $APP_DIRECTORY/app-notification > $LOG_DIRECTORY/app-notification-${CURRENT_DATE}-instance-$i.log &
done

#wait

# Build Auth-web to release
# dx bundle --package auth-web --release
# Run Auth-web from release
# IP=0.0.0.0 PORT=8080 ./target/dx/auth-web/release/web/auth-web
# IP=0.0.0.0 PORT=8080 RUST_BACKTRACE=1 ./target/dx/auth-web/release/web/auth-web

echo "All done"
exit 0 