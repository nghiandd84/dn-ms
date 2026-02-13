#!/bin/bash

# Add permission 
# sudo chmod +x start.sh

echo "Start DN-MS"

echo "Argument 1 is $1"

APP_DIRECTORY=./target/debug
RUST_LOG_DIRECTORY=/home/nghiandd/Training/dn-ms/logs
CURRENT_DATE=$(date +%Y%m%d)

export AUTH_PORT=5101
export AUTH_NOTIFICATION_PORT=5111
export BAKERY_PORT=5201
export EMAIL_TEMPLATE_PORT=5301
export NOTIFICATION_PORT=5401
export PROFILE_PORT=5501
export TRANSLATION_PORT=5601
export EVENT=5701
export NOTIFICATION_APP_PORT=4001

echo "Kill current instances"

# Kill Auth port
for i in {1..2}; do 
    fuser -k -15 510$i/tcp 
done

# Kill Auth Notification port
for i in {1..2}; do 
    fuser -k -15 511$i/tcp 
done


# Kill Bakery port
for i in {1..2}; do 
    fuser -k -15 520$i/tcp 
done


# Kill Email Template port
for i in {1..2}; do 
    fuser -k -15 530$i/tcp 
done
fuser -k -15 5306/tcp 
fuser -k -15 5307/tcp 
fuser -k -15 5308/tcp 

# Kill Notification port
for i in {1..2}; do 
    fuser -k -15 540$i/tcp 
done


# Kill Profile port
for i in {1..2}; do 
    fuser -k -15 550$i/tcp 
done

# Kill Translation port
for i in {1..2}; do 
    fuser -k -15 560$i/tcp 
done

# Kill Event port
for i in {1..2}; do 
    fuser -k -15 570$i/tcp 
done


# Kill Inventory port
for i in {1..2}; do 
    fuser -k -15 580$i/tcp 
done

# Kill Notification App port
for i in {1..2}; do 
    fuser -k -15 400$i/tcp 
done

# Kill Auth-Web
fuser -k -15 8080/tcp 


# Kill Gateway App
for i in {0..2}; do 
    fuser -k -15 600$i/tcp 
done


echo "Sucess kill all instances"

rm -v -rf $RUST_LOG_DIRECTORY/*
    
if [ "$1" == "kill" ]; then
    echo "Kill all api or app"
    exit 0  # Exit after killing processes
fi


echo "------------ Start Auth API ------------"
for i in {1..2}; do
    PORT=510$i
    echo "--- Auth on port $PORT ---"
    # Execute the program
    AUTH_PORT=510$i $APP_DIRECTORY/api-auth  &
done
sleep 1s

echo "------------ Start Bakery API ------------"
for i in {1..2}; do
    PORT=520$i
    echo "--- Bakery on port $PORT ---"
    # Execute the program
    BAKERY_PORT=520$i $APP_DIRECTORY/api-bakery  &
done
sleep 1s

echo "------------ Start Email Template API ------------"
for i in {1..2}; do
    PORT=530$i
    echo "--- Email Template on port $PORT ---"
    # Execute the program
    EMAIL_TEMPLATE_PORT=530$i TENANT=TENANT_${i} $APP_DIRECTORY/api-email-template  &
done
# Tenant DEFAULT
    EMAIL_TEMPLATE_PORT=5306 TENANT=TENANT_1 $APP_DIRECTORY/api-email-template  &
    EMAIL_TEMPLATE_PORT=5307 TENANT=TENANT_2 $APP_DIRECTORY/api-email-template  &
    EMAIL_TEMPLATE_PORT=5308 TENANT=DEFAULT $APP_DIRECTORY/api-email-template  &

sleep 1s

echo "------------ Start Notification API ------------"
for i in {1..2}; do
    PORT=540$i
    echo "--- Notification API on port $PORT ---"
    # Execute the program
    NOTIFICATION_PORT=540$i $APP_DIRECTORY/api-notification  &
done
sleep 1s

# Notification must start after all APIs
echo "------------ Start Notification After all API ------------"

echo "------------ Start Notification App ------------"
for i in {1..2}; do
    PORT=400$i
    echo "--- Notification App on port $PORT ---"
    # Execute the program
    NOTIFICATION_APP_PORT=400$i INSTANCE_ID=$i $APP_DIRECTORY/app-notification  &
done
sleep 1s

echo "------------ Start Auth notification ------------"
for i in {1..2}; do
    PORT=511$i
    echo "--- Auth Notification on port $PORT ---"
    # Execute the program
    AUTH_NOTIFICATION_PORT=511$i $APP_DIRECTORY/auth-notification  &
done
sleep 1s


echo "------------ Start Profile API ------------"
for i in {1..2}; do
    PORT=550$i
    echo "--- Profile on port $PORT ---"
    # Execute the program
    PROFILE_PORT=550$i $APP_DIRECTORY/api-profile  &
done
sleep 1s

echo "------------ Start Translation API ------------"
for i in {1..2}; do
    PORT=560$i
    echo "--- Translation on port $PORT ---"
    # Execute the program
    TRANSLATION_PORT=560$i $APP_DIRECTORY/api-translation  &
done
sleep 1s

echo "------------ Start Event API ------------"
for i in {1..2}; do
    PORT=570$i
    echo "--- Event on port $PORT ---"
    # Execute the program
    EVENT_PORT=570$i $APP_DIRECTORY/api-event  &
done
sleep 1s


echo "------------ Start INVENTORY API ------------"
for i in {1..2}; do
    PORT=580$i
    echo "--- INVENTORY on port $PORT ---"
    # Execute the program
    INVENTORY_PORT=580$i $APP_DIRECTORY/api-inventory  &
done
sleep 1s


echo "------------ Start Auth-Server ------------"
IP=0.0.0.0 PORT=8080 RUST_LOG=debug RUST_BACKTRACE=1 ./target/dx/auth-web/debug/web/auth-web >> $RUST_LOG_DIRECTORY/auth-server.log &

echo "------------ Start Gateway App ------------"
echo "--- Gateway start on Portal 6000, 6001, 6002 ---"
# Execute the program
$APP_DIRECTORY/app-gateway  &


sleep 1s


#wait

# Build Auth-web to release
# IP=0.0.0.0 PORT=8080 RUST_LOG=debug RUST_BACKTRACE=1 ./target/dx/auth-web/debug/web/auth-web
# dx bundle --package auth-web --release
# Run Auth-web from release
# IP=0.0.0.0 PORT=8080 ./target/dx/auth-web/release/web/auth-web
# IP=0.0.0.0 PORT=8080 RUST_LOG=debug RUST_BACKTRACE=1 ./target/dx/auth-web/release/web/auth-web

echo "All done"
exit 0 


# Run migrate
# cargo run --bin migrations_auth -- -v -u postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms -s auth
# Rollack to last version
# cargo run --bin migrations_auth -- -v -u postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms -s auth down
# Check migrate status
# cargo run --bin migrations_auth -- -v -u postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms -s auth status