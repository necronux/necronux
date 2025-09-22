// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::v0::{
        NormalizedProvisioningStrategy, NormalizedTool, NormalizedToolProvisioner,
        NormalizedToolProvisionerFallbacks, NormalizedToolProvisioningBinding,
        NormalizedToolProvisioningCommands,
    },
    grimoire_schemas::schemas::v0::{
        ParsedProvisioningStrategy, ParsedTool, ParsedToolProvisioner,
        ParsedToolProvisionerFallbacks, ParsedToolProvisioningBinding,
        ParsedToolProvisioningCommands,
    },
};

impl TryFrom<ParsedTool> for NormalizedTool {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedTool) -> Result<Self, Self::Error> {
        Ok(Self {
            name: s.name,
            executable: s.executable,
            provisioning_strategy: s
                .provisioning_strategy
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<ParsedProvisioningStrategy> for NormalizedProvisioningStrategy {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedProvisioningStrategy) -> Result<Self, Self::Error> {
        match s {
            ParsedProvisioningStrategy::Binding(b) => {
                Ok(NormalizedProvisioningStrategy::Binding(b.try_into()?))
            }
            ParsedProvisioningStrategy::Fallbacks(f) => {
                Ok(NormalizedProvisioningStrategy::Fallbacks(f.try_into()?))
            }
        }
    }
}

impl TryFrom<ParsedToolProvisioningBinding> for NormalizedToolProvisioningBinding {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedToolProvisioningBinding) -> Result<Self, Self::Error> {
        Ok(Self {
            tool_provisioner: s.tool_provisioner.try_into()?,
            provisioning_commands: s.provisioning_commands.try_into()?,
        })
    }
}

impl TryFrom<ParsedToolProvisionerFallbacks> for NormalizedToolProvisionerFallbacks {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedToolProvisionerFallbacks) -> Result<Self, Self::Error> {
        Ok(Self {
            fallbacks: s
                .fallbacks
                .into_iter()
                .map(|f| f.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<ParsedToolProvisioner> for NormalizedToolProvisioner {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedToolProvisioner) -> Result<Self, Self::Error> {
        Ok(Self {
            name: s.name,
            executable: s.executable,
        })
    }
}

impl TryFrom<ParsedToolProvisioningCommands> for NormalizedToolProvisioningCommands {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedToolProvisioningCommands) -> Result<Self, Self::Error> {
        Ok(Self {
            install_command_prefix_args: s.install_command_prefix_args,
            install_command: s.install_command,
            verify_command_prefix_args: s.verify_command_prefix_args,
            verify_command: s.verify_command,
            uninstall_command_prefix_args: s.uninstall_command_prefix_args,
            uninstall_command: s.uninstall_command,
        })
    }
}
