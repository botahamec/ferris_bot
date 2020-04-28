use serenity::{
	framework::standard::{
		macros::{command, group},
		CommandResult,
	},
	model::channel::Message,
	prelude::Context,
};

use crate::cmd_ctx_msg;

#[group]
#[owners_only]
#[commands(everyone)]
struct Admin;

cmd_ctx_msg!{everyone,
	msg.channel_id.say(&ctx.http, "@everyone")?;
}
