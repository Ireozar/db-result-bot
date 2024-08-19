#/bin/bash

firefox --marionette --profile $1 --headless &
sleep 1s
geckodriver --connect-existing --marionette-port 2828 &
echo "Running..."
