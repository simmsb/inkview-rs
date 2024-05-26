#!/bin/sh

LOCAL_PATH=$1
REMOTE_APP_NAME=$2
REMOTE_IP=$3
REMOTE_PORT=19991

echo "Sending '$LOCAL_PATH' to '$REMOTE_IP:$REMOTE_PORT/$REMOTE_APP_NAME'"

echo "Sending application name.."
echo "$REMOTE_APP_NAME" | nc "$REMOTE_IP" "$REMOTE_PORT"

# The ereader needs a bit of time to re-launch 'nc'
sleep 3

echo "Sending application content.."
nc "$REMOTE_IP" "$REMOTE_PORT" < "$LOCAL_PATH"
