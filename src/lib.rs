
use serenity::{prelude::*, async_trait};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, StandardFramework, CommandResult};
use serenity::model::gateway::{Ready, GatewayIntents};
use serenity::model::channel::Message;

use shuttle_secrets::SecretStore;
use anyhow::anyhow;
use tracing::info;

use reqwest;

#[group]
#[commands(view)]
struct General;

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[command]
async fn view(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let function = args.single_quoted::<String>().unwrap_or_default();
    let url = "https://raw.githubusercontent.com/HigherOrderCO/Wikind/master/";
    let final_url = format!("{}{}.kind2", url, function.replace(".", "/"));
    let response = reqwest::get(&final_url).await;
    
    match response {
        Ok(res) => {
            if res.status().is_success() {
                let text = res.text().await.unwrap_or_default();

                // Message Output:
                let code_response = format!("Reponse: \n```rust\n{} ```", text.trim());

                msg.channel_id.say(&ctx.http, code_response).await?;
            } else {
                msg.channel_id.say(&ctx.http, "Function Not Found.").await?;
            }
        }
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Not a kind2 function, refer to https://github.com/HigherOrderCO/Wikind").await?;
        }
    }

    Ok(())
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Discord token and GuildId (if necessary in ../Secrets.toml
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let framework = StandardFramework::new().configure(|c| c.prefix("/")).group(&GENERAL_GROUP);
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    Ok(client)
}
