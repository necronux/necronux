use crate::{
    controller::Cli,
    core::commands::lvl_1::infra::{InfraCommand, InfraSubCommands},
    core::handlers::infra::infra_run::handle_infra_run,
};
use clap::CommandFactory;
use color_eyre::eyre::{Ok, Result};
use log::{debug, info};

pub fn handle_infra(infra_command: &InfraCommand) -> Result<()> {
    debug!("Handling infra command subcommands");
    let command_handled = handle_infra_command(infra_command)?;
    if !command_handled {
        debug!("No infra subcommand or valid argument provided, displaying help message");
        let mut cli = Cli::command();
        if let Some(infra_command) = cli.find_subcommand_mut("infra") {
            infra_command.print_help()?;
        }
    }

    Ok(())
}

pub fn handle_infra_command(infra_command: &InfraCommand) -> Result<bool> {
    if let Some(subcommand) = &infra_command.infra_subcommand {
        match subcommand {
            InfraSubCommands::Run(args) => {
                info!("'infra run' command was provided");
                debug!("Handling infra run command");
                handle_infra_run(args)?;
                Ok(true)
            }
            InfraSubCommands::ListHosts => {
                info!("'list-hosts' flag was provided");
                debug!("Displaying list of hosts");
                println!("trinity-infinity,\ntrinity-heliolens");
                Ok(true)
            }
        }
    } else {
        Ok(false)
    }
}
