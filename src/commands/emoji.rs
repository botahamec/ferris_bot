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
#[commands(bird, man, onion)]
#[description = "Sends out an emoji in the chat"]
#[default_command(bird)]
#[prefixes("em", "emoji")]
struct Emoji;

cmd_ctx_msg!{bird,
	msg.channel_id.say(&ctx.http, "🐦")?;
}

cmd_ctx_msg!{man,
	msg.channel_id.say(&ctx.http, "🕴️")?;
}

cmd_ctx_msg!{onion,
	msg.channel_id.say(&ctx.http, "🧅")?;
}