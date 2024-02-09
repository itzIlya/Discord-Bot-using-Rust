#![allow(deprecated)]
#![allow(unused_imports)]
use std::ascii::escape_default;
use serenity::prelude::*;
use serenity::model::guild::Member;
use serenity::utils::MessageBuilder;

// additional uses:
 // We recommend migrating to poise, instead of using the standard command framework.
use std::collections::{HashMap, HashSet};
use std::env;
use std::env::args;
use std::fmt::Write;
use std::sync::Arc;
use lazy_static::lazy_static;

use serenity::async_trait;
use serenity::builder::EditChannel;
use serenity::framework::standard::buckets::{LimitedFor, RevertBucket};
use serenity::framework::standard::macros::{check, command, group, help, hook};
use serenity::framework::standard::{
    help_commands,
    Args,
    BucketBuilder,
    CommandGroup,
    CommandOptions,
    CommandResult,
    Configuration,
    DispatchError,
    HelpOptions,
    Reason,
    StandardFramework,
};
use serenity::gateway::ShardManager;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::id::UserId;
use serenity::model::permissions::Permissions;
use serenity::prelude::*;
use serenity::utils::{content_safe, ContentSafeOptions};
// end of additional uses
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}
// This is a multi threaded application so we are using RwLock:
// this allows multiple threads to read the list concurrently while acquiring a write lock for modifications.
lazy_static! {
    static ref BAD_WORDS: RwLock<Vec<String>> = {
        let mut initial_values = Vec::new();
        initial_values.extend(["shit", "fuck","ass"].iter().map(|&s| s.to_string()));
        RwLock::new(initial_values)
    };
    static ref ABOUT_TEXT: RwLock<String> = RwLock::new(String::from("Bot created by ilya"));
}


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        println!("New member joined : {}", new_member.user.name);
        // This is an unsafe approach. Would be better to handle the unwrap using matchin. Ik. I'm lazy af. 
        // Let's hope channel_id is always something meaningful #_#
        let channel_id= new_member.default_channel(&ctx.cache).unwrap().id;
        let welcome_message = MessageBuilder::new()
            .push("welcome ")
            .mention(&new_member)
            .push("!")
            .push("I'm the welcome bot. If you have any questions about the server or the channel or want to learn more about the commands, use ~help.")
            .build();
        
        // Not checking the result...
        // Don't really care if .say() works or not... 
        // It probably does, unless the bot isn't running. I guess we'll eventually find out.
        let _ = channel_id.say(&ctx.http,&welcome_message).await;
    }
    /* 
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content == "!ping" {

            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");

                    return;
                },
            };

            let response = MessageBuilder::new()
            .push("User ")
            .push_bold_safe(&msg.author.name)
            .push(" used the 'ping' command in the ")
            .mention(&channel)
            .push(" channel")
            .build();

            if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                println!("Error sending message: {why:?}");
            }
        }
    }
   */
}


#[group]
//#[commands(about, am_i_admin, say, commands, ping, latency, some_long_command, upper_command)]
#[commands(about,update_about,commands,remove_member)]
struct General;


#[help]
// This replaces the information that a user can pass a command-name as argument to gain specific
// information about it.
#[individual_command_tip = "Hello! こんにちは！Hola! Bonjour! 您好! 안녕하세요~\n\n\
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]

// Anything below is just some search related crap. Don't touch. Although, shit probably won't fall apart if you do.
// Define the maximum Levenshtein-distance between a searched command-name and commands. If the
// distance is lower than or equal the set distance, it will be displayed as a suggestion.
// Setting the distance to 0 will disable suggestions.
#[max_levenshtein_distance(3)]
// When you use sub-groups, Serenity will use the `indention_prefix` to indicate how deeply an item
// is indented. The default value is "-", it will be changed to "+".
#[indention_prefix = "+"]
// On another note, you can set up the help-menu-filter-behaviour.
// Here are all possible settings shown on all possible options.
// First case is if a user lacks permissions for a command, we can hide the command.
#[lacking_permissions = "Hide"]
// If the user is nothing but lacking a certain role, we just display it.
#[lacking_role = "Nothing"]
// The last `enum`-variant is `Strike`, which ~~strikes~~ a command.
#[wrong_channel = "Strike"]
// Serenity will automatically analyse and generate a hint/tip explaining the possible cases of
// ~~strikethrough-commands~~, but only if `strikethrough_commands_tip_in_{dm, guild}` aren't
// specified. If you pass in a value, it will be displayed instead.

async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}


#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Got command '{}' by user '{}'", command_name, msg.author.name);

    // Increment the number of times this command has been run once. If the command's name does not
    // exist in the counter, add a default value of 0.
    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command '{command_name}'"),
        Err(why) => println!("Command '{command_name}' returned error {why:?}"),
    }
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{unknown_command_name}'");
}

#[hook]
async fn normal_message(ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
    if bad_language(String::from(&msg.content)).await {
        let response = MessageBuilder::new()
            .mention(&msg.author)
            .push_bold(", watch your language please!")
            .build();
        let _ = msg.reply(&ctx.http, response).await;
    }
}
async fn bad_language(input_str : String) -> bool{
    let words =  BAD_WORDS.read().await ;

    for word in words.iter() {
        if input_str.contains(word) {
            return true;
        }
    }
    false
}

#[hook]
async fn delay_action(ctx: &Context, msg: &Message) {
    // You may want to handle a Discord rate limit if this fails.
    let _ = msg.react(ctx, '⏱').await;
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _command_name: &str) {
    if let DispatchError::Ratelimited(info) = error {
        // We notify them only once.
        if info.is_first_try {
            let _ = msg
                .channel_id
                .say(&ctx.http, &format!("Try this again in {} seconds.", info.as_secs()))
                .await;
        }
    }
}


#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new(&token);
    // Set gateway intents, which decides what events the bot will be notified about
    /*let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_PRESENCES
        |GatewayIntents::GUILD_MEMBERS;
     */
    let intents = GatewayIntents::all(); // bot has the intent to view and be notified about everything
    // FETCHING OWNER AND BOT ID:
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error)
        .bucket("emoji", BucketBuilder::default().delay(5)).await
        .bucket("complicated",
                BucketBuilder::default().limit(2).time_span(30).delay(5)
                    // The target each bucket will apply to.
                    .limit_for(LimitedFor::Channel)
                    // The maximum amount of command invocations that can be delayed per target.
                    // Setting this to 0 (default) will never await/delay commands and cancel the invocation.
                    .await_ratelimits(1)
                    // A function to call when a rate limit leads to a delay.
                    .delay_action(delay_action)
        ).await
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);
        //.group(&EMOJI_GROUP)
        //.group(&MATH_GROUP)
        //.group(&OWNER_GROUP);


    framework.configure(
        Configuration::new().with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("~") // this means that the commands should start with a ~ prefix
            // In this case, if "," would be first, a message would never be delimited at ", ",
            // forcing you to trim your arguments if you want to avoid whitespaces at the start of
            // each.
            .delimiters(vec![", ", ","])
            // Sets the bot's owners. These will be used for commands that are owners only.
            .owners(owners),
    );
    // old client definition
    /*let mut client =
        Client::builder(&token, intents).
            event_handler(Handler).await
            .expect("Err creating client");
     */

    // new client definition
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<CommandCounter>(HashMap::default())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    // Finally, start a single shard, and start listening to events.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

// in here we can start defining our custom commands


// Commands can be created via the attribute `#[command]` macro.
#[command]
// Options are passed via subsequent attributes.
// Make this command use the "complicated" bucket.
#[bucket = "complicated"]
async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.read().await;
    let counter = data.get::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");

    for (name, amount) in counter {
        writeln!(contents, "- {name}: {amount}")?;
    }

    msg.channel_id.say(&ctx.http, &contents).await?; // To hell with the return value aye?

    Ok(()) // we are not checking for any error (LET'S HOPE THIS SHIT ALWAYS WORKS)
}

// here is a simple command to test bot functionality
#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, ABOUT_TEXT.read().await.clone()).await?;

    Ok(())
}

#[command]
async fn update_about(ctx: &Context, msg: &Message,args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            let _ = msg.reply(&ctx.http, "Command only available in server channels.").await?;
            return Ok(());
        }
    };

    if let Ok(member) = msg.member(&ctx).await {
        if let Ok(perms) = member.permissions(&ctx.cache) {
            if !perms.contains(Permissions::all()) {
                let _ = msg.reply(&ctx.http, "You don't have permission to edit bot features").await?;
                return Ok(());
            }
        }
    }

    let mut guard = ABOUT_TEXT.write().await;
    *guard = String::from(format!("Bot made by itzIlya.\n{}",args.rest()));
    let _ = msg.reply(&ctx.http, "successfully updated about").await?;
    Ok(())
}


#[command]
async fn remove_member(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    
    
    let mut cloned_args = args.clone();

    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            let _ = msg.reply(&ctx.http, "Command only available in server channels.").await?;
            return Ok(());
        }
    };

    if let Ok(member) = msg.member(&ctx).await {
        if let Ok(perms) = member.permissions(&ctx.cache) {
            if !perms.contains(Permissions::KICK_MEMBERS) {
                let _ = msg.reply(&ctx.http, "You don't have permission to kick members.").await?;
                return Ok(());
            }
        }
    }
/* 
    // in case you want to extract the user_id 
    let mut cloned_args = args.clone();
    let user_id = match cloned_args.single::<>() {
        Ok(user_id) => user_id,
        Err(_) => {
            let _ = msg.reply(&ctx.http, "Please provide a valid user ID to kick.").await?;
            return Ok(());
        }
    };
*/
    let username = match cloned_args.single_quoted::<String>() {
        Ok(username) => username,
        Err(_) => {
            let _ = msg.reply(&ctx.http, "Please provide a valid username to kick.");
            return Ok(());
        }
    };

    
    // everything else is the reason for kick
    let reason = cloned_args.rest();
/* 
    this code is here in case you want to use userID to kick members instead of usernames
    let member = match msg.guild_id.unwrap().member(&ctx.http, user_id).await {
        Ok(member) => member,
        Err(_) => {
            let _ = msg.reply(&ctx.http, "User not found or an error occurred.").await?;
            return Ok(());
        }
    };
*/

    // Get member from the username
    let member = match guild_id.members(&ctx.http, None, None).await {
        Ok(members) => {
            // I have little clue about what the following lines do. It works so I wont touch it.
            // This is probably in syntax closest to  "  return i for i in iterable  " in python
            members.iter()
                .find(|member| member.user.name == username) // shitty unreadable syntax
                .cloned()
        },
        Err(_) => {
            msg.reply(&ctx.http, "An error occurred while fetching the user.").await?;
            return Ok(());
        }
    };
    
    // Check if member was found
    let member = match member {
        Some(member) => member,
        None => {
            msg.reply(&ctx.http, "User not found.").await?;
            return Ok(());
        }
    };

    // Kick the member
    if let Err(why) = member.kick_with_reason(&ctx.http, &format!("Kicked by {}", msg.author.name)).await {
        println!("Error kicking user: {:?}", why);
        let _ = msg.reply(&ctx.http, "Failed to kick the user.").await?;
    } else {
        println!("reached here22");
        let _ = msg.reply(&ctx.http, format!("User {} has been kicked for reason: {}", username, reason)).await?;
    }

    Ok(())
}