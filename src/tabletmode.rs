use clap::Args;
use clap::Subcommand;
use laptop_support::laptop::Laptop;

use crate::command::CommandTrait;

#[derive(Subcommand, Debug)]
pub enum TabletModeSubcommand {
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

impl TabletModeSubcommand {
    fn run(&self, laptop: &Laptop) {
        match self {
            TabletModeSubcommand::Enable => {
                laptop.tabletmode.enable().unwrap();
            }
            TabletModeSubcommand::Disable => {
                laptop.tabletmode.disable().unwrap();
            }
            TabletModeSubcommand::Auto => {
                laptop.tabletmode.enable_auto_detect().unwrap();
            }
            TabletModeSubcommand::NoAuto => {
                laptop.tabletmode.disable_auto_detect().unwrap();
            }
            TabletModeSubcommand::Status => {
                let is_tabletmode = laptop.tabletmode.is_enabled().unwrap();
                let is_auto = laptop.tabletmode.is_auto_detect_enabled().unwrap();

                println!("Tablet mode: {}", is_tabletmode);
                println!("Auto switch: {}", is_auto);
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct TabletMode {
    #[clap(subcommand)]
    subcmd: TabletModeSubcommand,
}

impl CommandTrait for TabletMode {
    fn execute(&self, laptop: &Laptop) {
        self.subcmd.run(laptop);
    }
}
