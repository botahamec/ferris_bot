pub mod admin;
pub mod emoji;
pub mod general;

pub use admin::ADMIN_GROUP;
pub use emoji::EMOJI_GROUP;
pub use general::GENERAL_GROUP;

#[macro_export]
macro_rules! cmd_ctx_msg {
	($name: ident, $($line: stmt)*) => {
		#[command]
		fn $name(ctx: &mut Context, msg: &Message) -> CommandResult {
			$($line)*
			Ok(())
		}
	};
}

#[macro_export]
macro_rules! cmd_ctx_msg_args {
	($name: ident, $($line: stmt)*) => {
		#[command]
		fn $name(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
			$($line)*
			Ok(())
		}
	};
}