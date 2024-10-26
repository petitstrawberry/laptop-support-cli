use clap::Args;
use clap::Subcommand;
use laptop_support::laptop::Laptop;

use crate::command::CommandTrait;

#[derive(Subcommand, Debug)]
pub enum MouseSubcommand {
    #[command(about = "Enable mouse pass-through")]
    Enable,
    #[command(about = "Disable mouse pass-through")]
    Disable,
    #[command(about = "Get current mouse pass-through status")]
    Status,
}

impl MouseSubcommand {
    fn run(&self, laptop: &Laptop) {
        match self {
            MouseSubcommand::Enable => {
                laptop.mouse.enable_passthrough().unwrap();
            }
            MouseSubcommand::Disable => {
                laptop.mouse.disable_passthrough().unwrap();
            }
            MouseSubcommand::Status => {
                let is_enabled = laptop.mouse.is_passthrough_enabled().unwrap();

                println!("Mouse pass-through: {}", is_enabled);
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct Mouse {
    #[clap(subcommand)]
    subcmd: MouseSubcommand,
}

impl CommandTrait for Mouse {
    fn execute(&self, laptop: &Laptop) {
        self.subcmd.run(laptop);
    }
}