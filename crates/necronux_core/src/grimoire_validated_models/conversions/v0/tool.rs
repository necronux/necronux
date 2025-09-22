// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError,
    grimoire_normalized_models::models::v0::{
        NormalizedProvisioningStrategy, NormalizedTool, NormalizedToolProvisioner,
        NormalizedToolProvisionerFallbacks, NormalizedToolProvisioningBinding,
        NormalizedToolProvisioningCommands,
    },
    models::v0::{
        ValidatedProvisioningStrategy, ValidatedTool, ValidatedToolProvisioner,
        ValidatedToolProvisionerFallbacks, ValidatedToolProvisioningBinding,
        ValidatedToolProvisioningCommands,
    },
};

impl TryFrom<NormalizedTool> for ValidatedTool {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedTool) -> Result<Self, Self::Error> {
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

impl TryFrom<NormalizedProvisioningStrategy> for ValidatedProvisioningStrategy {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedProvisioningStrategy) -> Result<Self, Self::Error> {
        match s {
            NormalizedProvisioningStrategy::Binding(b) => {
                Ok(ValidatedProvisioningStrategy::Binding(b.try_into()?))
            }
            NormalizedProvisioningStrategy::Fallbacks(f) => {
                Ok(ValidatedProvisioningStrategy::Fallbacks(f.try_into()?))
            }
        }
    }
}

impl TryFrom<NormalizedToolProvisioningBinding> for ValidatedToolProvisioningBinding {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedToolProvisioningBinding) -> Result<Self, Self::Error> {
        Ok(Self {
            tool_provisioner: s.tool_provisioner.try_into()?,
            provisioning_commands: s.provisioning_commands.try_into()?,
        })
    }
}

impl TryFrom<NormalizedToolProvisionerFallbacks> for ValidatedToolProvisionerFallbacks {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedToolProvisionerFallbacks) -> Result<Self, Self::Error> {
        Ok(Self {
            fallbacks: s
                .fallbacks
                .into_iter()
                .map(|f| f.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<NormalizedToolProvisioner> for ValidatedToolProvisioner {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedToolProvisioner) -> Result<Self, Self::Error> {
        Ok(Self {
            name: s.name,
            executable: s.executable,
        })
    }
}

impl TryFrom<NormalizedToolProvisioningCommands> for ValidatedToolProvisioningCommands {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedToolProvisioningCommands) -> Result<Self, Self::Error> {
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
