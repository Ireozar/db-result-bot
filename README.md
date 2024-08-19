# db-result-bot
A Discord bot to get the winner of a DuelingBook replay link submitted.

## Installation
- build the binary
```git clone https://github.com/Ireozar/db-result-bot.git
cd db-result-bot
cargo build --release

- install geckodriver from your favorite source
- install firefox and add a new profile with:
```firefox -ProfileManager
- log into the profile's duelingbook and let it remember your login
- start geckodriver using:
```./gecho.sh $PATH_TO_FIREFOX_PROFILE`
(with the path set to your custom profile's path, typically something like `~/.mozilla/firefox/$PROFILE`)
- start the binary with the env variable DISCORD_TOKEN set
