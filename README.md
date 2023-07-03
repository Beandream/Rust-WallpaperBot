# Rust-WallpaperBot
 WallpaperBot, a discord bot rewritten in Rust

Used to automatically moderate a discord channel meant for images for a wallpaper competition.
When a user posts a message, the bot detects if its an image.
If true it automatically removes all previous non-image messages.
This allows for discussions to be made on submitted images before the next user submits one, without cluttering the discord channel.

The bot uses slash commands:
    /hello - is a simple hello command for testing purposes.
    /clean - manually starts the script that cleans the discord channel from non-image messages. (requires manage_messages permissions)


## TODOS:
    (!IMPORTANT) only run the automatic image dectection moderation in the correct channel and not in every channel. (this should be customizable and not hardcoded)

    create an integer option for the /clean command on how many messages you want to remove. (currrently defaults to 100)

    profit
    
## IDEAS:
    send a confirm message when users post an image on if it is meant to be a submission or not. (to prevent users posting non-submission images to the channel)
        could probably be reaction based. or maybe there is cool new discord ui features for buttons, not sure.
    use ai to generate a responses on the submission images.
        it would have to analyze the image to figure out what it is meant to be to give valid feedback.
        would probably work poorly in practice but could be fun to implement.
    used embeds for more organized announcing of the competition themes and deadlines.
