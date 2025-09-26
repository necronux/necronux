// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedProvisioningStrategy, NormalizedTool, NormalizedToolProvisioner,
        NormalizedToolProvisionerFallbacks, NormalizedToolProvisioningBinding,
        NormalizedToolProvisioningCommands,
    },
    models::v0_4::{
        ValidatedProvisioningStrategy, ValidatedTool, ValidatedToolProvisioner,
        ValidatedToolProvisionerFallbacks, ValidatedToolProvisioningBinding,
        ValidatedToolProvisioningCommands,
    },
    validators,
};

impl TryFrom<NormalizedTool> for ValidatedTool {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedTool) -> Result<Self, Self::Error> {
        let parent_object = "Tool";
        Ok(Self {
            name: validators::ensure_str_is_not_empty(s.name, "name", parent_object)?,
            executable: validators::ensure_str_is_not_empty(
                s.executable,
                "executable",
                parent_object,
            )?,
            provisioning_strategy: validators::ensure_vec_is_not_empty(
                s.provisioning_strategy,
                "provisioningStrategy",
                parent_object,
            )?
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
        let parent_object = "ToolProvisionerFallbacks";
        Ok(Self {
            fallbacks: validators::ensure_vec_is_not_empty(
                s.fallbacks,
                "fallbacks",
                parent_object,
            )?
            .into_iter()
            .map(|f| f.try_into())
            .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<NormalizedToolProvisioner> for ValidatedToolProvisioner {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedToolProvisioner) -> Result<Self, Self::Error> {
        let parent_object = "ToolProvisioner";
        Ok(Self {
            name: validators::ensure_str_is_not_empty(s.name, "name", parent_object)?,
            executable: validators::ensure_str_is_not_empty(
                s.executable,
                "executable",
                parent_object,
            )?,
        })
    }
}

impl TryFrom<NormalizedToolProvisioningCommands> for ValidatedToolProvisioningCommands {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedToolProvisioningCommands) -> Result<Self, Self::Error> {
        let parent_object = "ToolProvisioningCommands";
        Ok(Self {
            install_command_prefix_args: validators::ensure_some_str_is_not_empty(
                s.install_command_prefix_args,
                "installCommandPrefixArgs",
                parent_object,
            )?,
            install_command: validators::ensure_str_is_not_empty(
                s.install_command,
                "installCommand",
                parent_object,
            )?,
            verify_command_prefix_args: validators::ensure_some_str_is_not_empty(
                s.verify_command_prefix_args,
                "verifyCommandPrefixArgs",
                parent_object,
            )?,
            verify_command: validators::ensure_str_is_not_empty(
                s.verify_command,
                "verifyCommand",
                parent_object,
            )?,
            uninstall_command_prefix_args: validators::ensure_some_str_is_not_empty(
                s.uninstall_command_prefix_args,
                "uninstallCommandPrefixArgs",
                parent_object,
            )?,
            uninstall_command: validators::ensure_str_is_not_empty(
                s.uninstall_command,
                "uninstallCommand",
                parent_object,
            )?,
        })
    }
}
