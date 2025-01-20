use crate::commands::lvl_1::{app::AppCommand, infra::InfraCommand, system::SystemCommand};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum NecronuxCommand {
    #[command()]
    /// Setup infrastructure, resources and their configurations
    Infra(InfraCommand),

    #[command()]
    /// Setup system tweaks and execute system utilities
    System(SystemCommand),

    #[command()]
    /// Setup application or batch of applications with their configurations
    App(AppCommand),
}
