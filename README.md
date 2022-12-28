
# SOS Médecins Oise available slots

*Update 2022-12-28*

I've found that all slots for the day are dumped at 6am, 9am and 11am

This is a small RiiR project about finding an available slot to book a medical doctor.
My region area is flagged as a "medical desert" and finding an available medical doctor (especially when you have kids) is a pain because public hospital urgency service is currently under heavy load mainly because of our shitty gov (France) and COVID.

## Run
I run the compiled bin in a crontab every minute and when slots are available, it sends me a Telegram message to my personal group chat until a kill it.
2 environment variables are required:
- `BOT_TOKEN` for Telegram
- `CHAT_ID` the unique ID of group conversation on Telegram

Some stdout logs are provided.

## TODO
This is a feature list I want to dev:
- [ ] Snooze notification until SOS API respond "no available slots"
- [ ] i18n all messages (currently in French)
- [ ] List available slots in the Telegram message
- [ ] Make the bot more smart with command to subscribe/unsubscribe for notification (lot of work and I would use [Teloxide](https://github.com/teloxide/teloxide))
