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
#[only_in(guild)]
#[description = "Admins only"]
struct Admin;

#[command]
#[description = "Do you hate the people on your server? Use this command to prove it!"]
fn everyone(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, "@everyone")?;
	Ok(())
}
