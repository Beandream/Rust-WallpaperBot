use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;

fn message_has_image(msg :&Message) -> bool{
    let Some(attachment) = msg.attachments.get(0) else {return false};
    if attachment.content_type.as_ref().unwrap().contains("image") {
        return true;
    }
    return false;
}

async fn delete_msg(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    if let Err(e) = msg.delete(ctx).await {
        println!("Error deleting message: {:?}", e);
    }
    Ok(())
}


async fn clean_channel(ctx: &Context, channel_id: &ChannelId) -> Result<(), serenity::Error> {
    let messages = channel_id.messages(ctx, |retriever| retriever.limit(10)).await?;

    for message in messages {
        if !message_has_image(&message) {
            delete_msg(&ctx, &message).await?;
        }
    }
    Ok(())
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(433758414880112640);

        let _ = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {command.name("hello").description("Say Hello!") })
                .create_application_command(|command| {command.name("clean").description("Clean channel of non-submission messages.") })

        }).await.unwrap();

    }
    
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // println!("{:#?}", interaction.channel_id);

        if let Interaction::ApplicationCommand(command) = interaction {

            let _ = match command.data.name.as_str() {
                "hello" => {
                    respond_to_interaction(&ctx, &command, "hello".to_owned()).await.expect("Cannot repond to slash command")
                },
                "clean" => {
                    respond_to_interaction(&ctx, &command, "Cleaning the channel of non-submission messages.".to_owned()).await.expect("Cannot repond to slash command");
                    clean_channel(&ctx, &command.channel_id).await.expect("No worky");

                },
                command => unreachable!("Unknown command: {}", command),
            };
        }
    }
}

async fn respond_to_interaction(ctx: &Context, command: &ApplicationCommandInteraction, response_content: String) -> Result<(), serenity::Error> {

    command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(response_content))

    }).await?;
    
    Ok(())
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}