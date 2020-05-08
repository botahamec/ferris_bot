use serenity::{
	framework::standard::{
        Args, CheckResult, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, StandardFramework,
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
#[individual_command_tip = "?help [command] gives info about the command"]
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
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.reply(&ctx, "Pong?")?;
	Ok(())
}

#[command]
#[description = "Repeats what 'ya said"]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	msg.reply(&ctx, args.rest())?;
	Ok(())
}
