#!/bin/bash

# Add permission 
# sudo chmod +x start.sh

echo "Start DN-MS"

echo "Argument 1 is $1"

APP_DIRECTORY=./target/debug
LOG_DIRECTORY=./target/logs
export BAKERY_PORT=5201
export AUTH_PORT=5101

echo "Kill current instances"

    # Kill port
    for i in {1..3}; do 
        fuser -k 520$i/tcp 
    done

    # Kill port
    for i in {1..3}; do 
        fuser -k 510$i/tcp 
    done

    echo "Sucess kill all instances"

    
if [ "$1" == "kill" ]; then
    exit 0  # Exit after killing processes
fi

echo "------------ Start Bakery app ------------"
for i in {1..3}; do
    PORT=520$i
    echo "--- Bakery on port $PORT ---"
    # Execute the program
    BAKERY_PORT=520$i $APP_DIRECTORY/api-bakery > $LOG_DIRECTORY/api-bakery-instance-$i.log &
done

echo "------------ Start Auth app ------------"
for i in {1..3}; do
    PORT=510$i
    echo "--- Auth on port $PORT ---"
    # Execute the program
    AUTH_PORT=510$i $APP_DIRECTORY/api-auth > $LOG_DIRECTORY/api-auth-instance-$i.log &
done

#wait

echo "All done"
exit 0 