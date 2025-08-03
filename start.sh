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
export EMAIL_TEMPLATE_PORT=3201

echo "Kill current instances"

# Kill Auth port
for i in {1..3}; do 
    fuser -k 510$i/tcp 
done

# Kill Bakery port
for i in {1..3}; do 
    fuser -k 520$i/tcp 
done


# Kill Email Template port
for i in {1..3}; do 
    fuser -k 530$i/tcp 
done

echo "Sucess kill all instances"

rm -v $LOG_DIRECTORY/*

    
if [ "$1" == "kill" ]; then
    exit 0  # Exit after killing processes
fi


echo "------------ Start Auth app ------------"
for i in {1..3}; do
    PORT=510$i
    echo "--- Auth on port $PORT ---"
    # Execute the program
    AUTH_PORT=510$i $APP_DIRECTORY/api-auth > $LOG_DIRECTORY/api-auth-${CURRENT_DATE}-instance-$i.log &
done


echo "------------ Start Bakery app ------------"
for i in {1..3}; do
    PORT=520$i
    echo "--- Bakery on port $PORT ---"
    # Execute the program
    BAKERY_PORT=520$i $APP_DIRECTORY/api-bakery > $LOG_DIRECTORY/api-bakery-${CURRENT_DATE}-instance-$i.log &
done


echo "------------ Start Email Template app ------------"
for i in {1..3}; do
    PORT=530$i
    echo "--- Email Template on port $PORT ---"
    # Execute the program
    EMAIL_TEMPLATE_PORT=530$i $APP_DIRECTORY/api-email-template > $LOG_DIRECTORY/api-email-template-${CURRENT_DATE}-instance-$i.log &
done

#wait

echo "All done"
exit 0 