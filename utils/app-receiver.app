#!/bin/sh

LOCAL_PORT=19991

echo "Listening on :$LOCAL_PORT for application name.."
LOCAL_APP_NAME=$(nc -l -p "$LOCAL_PORT" | tr -d ' ')
echo "Received application name : '$LOCAL_APP_NAME'"

pid=$(pidof "$LOCAL_APP_NAME")
if [ -n "$pid" ]; then
    echo "Killing running application with pid $pid"
    kill "$pid"
fi

LOCAL_APP_PATH="/mnt/ext1/applications/$LOCAL_APP_NAME"
echo "Application will be saved to '$LOCAL_APP_PATH'"

echo "Listening on :$LOCAL_PORT for application content.."
nc -l -p "$LOCAL_PORT" > "$LOCAL_APP_PATH"
echo "Application has been saved to '$LOCAL_APP_PATH'"
