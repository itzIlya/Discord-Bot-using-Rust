# discordReadme

# Language sensitive discord bot

This is a relatively simple multithreaded discord bot program written entirely in rust using [Serenity](https://github.com/serenity-rs/serenity/tree/current) and [Tokio](https://github.com/tokio-rs/tokio). 

## Functionality

### Language checking

    The bot checks every message that is sent to all the channels within a server for bad language. Using profanity in a message will cause the bot to reply to that message with a reminder to not use bad language in the channel. 

### Welcome message

    Each time a new member joins the server, depending on how they were invited, the bot will send out a welcome message to that member in one of the channels in the server.

### Commands

    The bot supports the following commands:

- `**~commands`  :** Shows the number of times members have attempted to use each command.
- **`~help [optional:command_name]`**: Provides a guide for using the bot and sending out commands.
- `**~about**` : Upon calling this command, the bot will send a custom message set by the admin. The admin can customize this message and put information about the server and channels, server guidelines or additional information.
- `**~update_about [new_about]**` : Admins can call this command to change the text that is shown when members call the ~about command. Only members with the role named  “admin” can call this command.
- `**~remove_member [Username] [optional:reason]**` : Admins can call this command to call for immediate removal of a member from the server and possibly provide a reason for their action. The member with the specified username will be kicked by the bot. Only members with the role named “admin” can call this command.
- `**~get_image [url]` *(coming soon)*** : Members can provide the bot with a URL for an image and receive the file for that image in the channel.

## Setting up the bot

To set up the bot, you first need to install all the dependencies for the bot, mainly Tokio and Serenity. You will then have to use a developer account to create a new discord application and get a Token. The Token should be specified as an environment variable named **DISCORD_TOKEN**. 

The bot will need certain permissions for the server. To grant those permissions first go to the bot dashboard on discord. Go to Privileged Gateway Intents and activate all the intents for the application. You can also check all the permissions if you wish to. Next, go to OAuth2 tab and click on the “bot” option. In the bot permissions tab, check “Administrator”.  Finally copy and paste the link at the bottom of the page in your browser and add the bot to your server with role of your choosing.

## About the bot

    The bot is a multithreaded program that uses the Tokio framework to accomplish its multithreaded needs. Things that can be edited during runtime such as the about text for the server need to use locks to prevent multiple threads from accessing them at the same time (Lines 68-75 in src/main.rs). Accessing the shared variables also requires requesting for locks. The bot can handle multiple requests at the same time from a single server, However, it can not run multiple servers/shards at once. This feature may be added in the future as it does not require much change to structure of the code except for the main function and client initialization. Feel free to make these changes yourself if you need to. A guide to handling multiple shards is in the [following example of the serenity documentation](https://github.com/serenity-rs/serenity/blob/current/examples/e02_transparent_guild_sharding/src/main.rs).

## Purpose

The purpose of this bot was to learn to work with multiple threads in rust and play around with the discord API using Serenity.