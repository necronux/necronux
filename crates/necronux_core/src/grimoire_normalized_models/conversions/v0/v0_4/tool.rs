// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::{
        models::v0_4::{
            NormalizedProvisioningStrategy, NormalizedTool, NormalizedToolProvisioner,
            NormalizedToolProvisionerFallbacks, NormalizedToolProvisioningBinding,
            NormalizedToolProvisioningCommands,
        },
        normalizers,
    },
    grimoire_schemas::schemas::v0_4::{
        ParsedProvisioningStrategy, ParsedTool, ParsedToolProvisioner,
        ParsedToolProvisionerFallbacks, ParsedToolProvisioningBinding,
        ParsedToolProvisioningCommands,
    },
};

impl TryFrom<ParsedTool> for NormalizedTool {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedTool) -> Result<Self, Self::Error> {
        let parent_object = "Tool";
        Ok(Self {
            name: normalizers::ensure_req_field_is_not_missing(s.name, "name", &parent_object)?,
            executable: normalizers::ensure_req_field_is_not_missing(
                s.executable,
                "executable",
                &parent_object,
            )?,
            provisioning_strategy: normalizers::ensure_req_field_is_not_missing(
                s.provisioning_strategy,
                "provisioningStrategy",
                &parent_object,
            )?
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
        let parent_object = "ToolProvisioningBinding";
        Ok(Self {
            tool_provisioner: normalizers::ensure_req_field_is_not_missing(
                s.tool_provisioner,
                "toolProvisioner",
                &parent_object,
            )?
            .try_into()?,
            provisioning_commands: normalizers::ensure_req_field_is_not_missing(
                s.provisioning_commands,
                "provisioningCommands",
                &parent_object,
            )?
            .try_into()?,
        })
    }
}

impl TryFrom<ParsedToolProvisionerFallbacks> for NormalizedToolProvisionerFallbacks {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedToolProvisionerFallbacks) -> Result<Self, Self::Error> {
        let parent_object = "ToolProvisionerFallbacks";
        Ok(Self {
            fallbacks: normalizers::ensure_req_field_is_not_missing(
                s.fallbacks,
                "fallbacks",
                &parent_object,
            )?
            .into_iter()
            .map(|f| f.try_into())
            .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<ParsedToolProvisioner> for NormalizedToolProvisioner {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedToolProvisioner) -> Result<Self, Self::Error> {
        let parent_object = "ToolProvisioner";
        Ok(Self {
            name: normalizers::ensure_req_field_is_not_missing(s.name, "name", &parent_object)?,
            executable: normalizers::ensure_req_field_is_not_missing(
                s.executable,
                "executable",
                &parent_object,
            )?,
        })
    }
}

impl TryFrom<ParsedToolProvisioningCommands> for NormalizedToolProvisioningCommands {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedToolProvisioningCommands) -> Result<Self, Self::Error> {
        let parent_object = "ToolProvisioningCommands";
        Ok(Self {
            install_command_prefix_args: s.install_command_prefix_args,
            install_command: normalizers::ensure_req_field_is_not_missing(
                s.install_command,
                "installCommand",
                &parent_object,
            )?,
            verify_command_prefix_args: s.verify_command_prefix_args,
            verify_command: normalizers::ensure_req_field_is_not_missing(
                s.verify_command,
                "verifyCommand",
                &parent_object,
            )?,
            uninstall_command_prefix_args: s.uninstall_command_prefix_args,
            uninstall_command: normalizers::ensure_req_field_is_not_missing(
                s.uninstall_command,
                "uninstallCommand",
                &parent_object,
            )?,
        })
    }
}
