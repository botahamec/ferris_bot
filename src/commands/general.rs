use serenity::{
	framework::standard::{
		macros::{command, group},
		Args, CommandResult,
	},
	model::channel::Message,
	prelude::Context,
};

use crate::cmd_ctx_msg;
use crate::cmd_ctx_msg_args;

#[group]
#[commands(ping, say)]
struct General;

cmd_ctx_msg!{ping,
	msg.reply(&ctx, "Pong?")?;
}

cmd_ctx_msg_args!{say,
	msg.channel_id.say(&ctx.http, args.rest())?;
}
