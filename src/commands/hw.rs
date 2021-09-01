use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

const HELP_MESSAGE: &str = "
Hello World!

https://www.youtube.com/watch?v=bPmVhyHBRAM.

<#881344414511927306>
— amp 
";


#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, HELP_MESSAGE).await?;

    Ok(())
}