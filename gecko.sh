#/bin/bash

firefox --marionette --profile ~/.mozilla/firefox/j1yb8rro.Replayer/ --headless &
sleep 1s
geckodriver --connect-existing --marionette-port 2828 &
echo "Running..."
