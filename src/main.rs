use clap::Parser;
use clap::Subcommand;
use laptop_support::laptop::Laptop;

// Command line tool for laptop-support protocol
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[clap(subcommand)]
    subcmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Control tablet mode")]
    Tabletmode {
        #[clap(subcommand)]
        action: TabletModeAction,
    },
}

#[derive(Subcommand, Debug)]
enum TabletModeAction {
    #[command(about = "Enable tablet mode")]
    Enable,
    #[command(about = "Disable tablet mode")]
    Disable,
    #[command(about = "Enable auto switch to tablet mode")]
    Auto,
    #[command(about = "Disable auto switch to tablet mode")]
    NoAuto,
    #[command(about = "Get current tablet mode, auto switch status")]
    Status,
}

fn main() {
    let args = Args::parse();
    let laptop = Laptop::new();

    match args.subcmd {
        Command::Tabletmode { action } => match action {
            TabletModeAction::Enable => {
                laptop.tabletmode.enable().unwrap();
            }
            TabletModeAction::Disable => {
                laptop.tabletmode.disable().unwrap();
            }
            TabletModeAction::Auto => {
                laptop.tabletmode.enable_auto_detect().unwrap();
            }
            TabletModeAction::NoAuto => {
                laptop.tabletmode.disable_auto_detect().unwrap();
            }
            TabletModeAction::Status => {
                let is_tabletmode = laptop.tabletmode.is_enabled().unwrap();
                let is_auto = laptop.tabletmode.is_auto_detect_enabled().unwrap();

                println!("Tablet mode: {}", is_tabletmode);
                println!("Auto switch: {}", is_auto);
            }
        },
    }
}
