mod swimlane;

use crate::swimlane::swimlane_command;
use clap::command;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(swimlane_command())
        .get_matches();

    match matches.subcommand() {
        Some(("swimlane", sub_matches)) => {
            swimlane::process_swimlane_command(sub_matches);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
