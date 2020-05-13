use serenity::{
	framework::standard::{
        Args, CheckResult, CommandResult, CommandOptions, CommandGroup,
        HelpOptions, help_commands,
        macros::{command, group, help, check},
    },
	model::channel::Message,
	model::id::UserId,
	prelude::Context,
};

use std::collections::HashSet;

#[help]
#[command_not_found_text = "Command not found: `{}`"]
#[strikethrough_commands_tip_in_dm(" ")]
#[strikethrough_commands_tip_in_guild(" ")]
#[individual_command_tip = "?help (command) gives info about the command"]
#[lacking_permissions = "Nothing"]
#[lacking_role = "Nothing"]
#[lacking_ownership = "Strike"]
fn help(
	context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
	help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}


#[group]
#[commands(ping, say)]
#[description = "All of the general commands"]
struct General;

#[command]
#[description = "🏓"]
#[only_in(guild)]
#[required_permissions("MANAGE_EMOJIS")]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.reply(&ctx, "Pong?")?;
	Ok(())
}

#[command]
#[description = "Repeats what 'ya said"]
#[aliases("repeat", "echo")]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	msg.channel_id.say(&ctx, args.rest())?;
	Ok(())
}

#[command]
#[checks(Bot)]
fn talk_to_self(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.reply(&ctx, "Hello, myself!")?;
	Ok(())
}


#[check]
#[name = "Bot"]
fn bot_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
	if let Some(member) = msg.member(&ctx.cache) {
		let user = member.user.read();
		user.bot.into()
	} else {
		false.into()
	}
}