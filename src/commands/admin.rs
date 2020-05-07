use serenity::{
	framework::standard::{
		macros::{command, group},
		CommandResult,
	},
	model::channel::Message,
	prelude::Context,
};

#[group]
#[owners_only]
#[commands(everyone)]
struct Admin;

#[command]
fn everyone(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, "@everyone")?;
	Ok(())
}
