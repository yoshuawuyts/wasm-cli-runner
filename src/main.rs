use std::io;
use std::path::PathBuf;

use clap::{Command, Parser};

#[derive(clap::Parser)]
struct Arg {
    /// The path to a .wasm binary
    path: PathBuf,
}

fn main() -> io::Result<()> {
    let Arg { path } = Arg::parse();
    path.canonicalize()?;

    let commands = vec![make_command("bar"), make_command("foo")];
    let mut cli = Command::new("test")
        .subcommand_required(true)
        .arg_required_else_help(true);

    for cmd in commands {
        cli = cli.subcommand(cmd);
    }

    // Augment with derived subcommands
    let matches = cli.get_matches();
    Ok(())
}

fn make_command(name: &'static str) -> Command {
    Command::new(&name).about(&format!("the {name} command"))
}
