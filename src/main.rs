mod command;
mod tabletmode;
mod keyboard;
mod mouse;

use clap::Parser;
use clap::Subcommand;
use command::CommandTrait;
use laptop_support::laptop::Laptop;
use tabletmode::TabletMode;

// Command line tool for laptop-support protocol
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Control tablet mode")]
    Tabletmode(TabletMode),
    #[command(about = "Control keyboard")]
    Keyboard(keyboard::Keyboard),
    #[command(about = "Control mouse")]
    Mouse(mouse::Mouse),
}

impl Commands {
    fn execute(&self, laptop: &Laptop) {
        match self {
            Commands::Tabletmode(cmd) => cmd.execute(laptop),
            Commands::Keyboard(cmd) => cmd.execute(laptop),
            Commands::Mouse(cmd) => cmd.execute(laptop),
        }
    }
}

fn main() {
    let args = Args::parse();
    let laptop = Laptop::new();

    args.subcmd.execute(&laptop);
}
