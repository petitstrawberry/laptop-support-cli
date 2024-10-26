use clap::Args;
use clap::Subcommand;
use laptop_support::laptop::Laptop;

use crate::command::CommandTrait;

#[derive(Subcommand, Debug)]
pub enum KeyboardSubcommand {
    #[command(about = "Enable keyboard pass-through")]
    Enable,
    #[command(about = "Disable keyboard pass-through")]
    Disable,
    #[command(about = "Get current keyboard pass-through status")]
    Status,
}

impl KeyboardSubcommand {
    fn run(&self, laptop: &Laptop) {
        match self {
            KeyboardSubcommand::Enable => {
                laptop.keyboard.enable_passthrough().unwrap();
            }
            KeyboardSubcommand::Disable => {
                laptop.keyboard.disable_passthrough().unwrap();
            }
            KeyboardSubcommand::Status => {
                let is_enabled = laptop.keyboard.is_passthrough_enabled().unwrap();

                println!("Keyboard pass-through: {}", is_enabled);
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct Keyboard {
    #[clap(subcommand)]
    subcmd: KeyboardSubcommand,
}

impl CommandTrait for Keyboard {
    fn execute(&self, laptop: &Laptop) {
        self.subcmd.run(laptop);
    }
}