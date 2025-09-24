// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedProvisioningStrategy, NormalizedTool, NormalizedToolProvisioner,
        NormalizedToolProvisionerFallbacks, NormalizedToolProvisioningBinding,
        NormalizedToolProvisioningCommands,
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
        Ok(Self {
            name: s
                .name
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "name".to_string(),
                    parent_object_name: "Tool".to_string(),
                })?,
            executable: s.executable.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "executable".to_string(),
                    parent_object_name: "Tool".to_string(),
                }
            })?,
            provisioning_strategy: s
                .provisioning_strategy
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "provisioningStrategy".to_string(),
                    parent_object_name: "Tool".to_string(),
                })?
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
            tool_provisioner: s
                .tool_provisioner
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "toolProvisioner".to_string(),
                    parent_object_name: "ToolProvisioningBinding".to_string(),
                })?
                .try_into()?,
            provisioning_commands: s
                .provisioning_commands
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "provisioningCommands".to_string(),
                    parent_object_name: "ToolProvisioningBinding".to_string(),
                })?
                .try_into()?,
        })
    }
}

impl TryFrom<ParsedToolProvisionerFallbacks> for NormalizedToolProvisionerFallbacks {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedToolProvisionerFallbacks) -> Result<Self, Self::Error> {
        Ok(Self {
            fallbacks: s
                .fallbacks
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "fallbacks".to_string(),
                    parent_object_name: "ToolProvisionerFallbacks".to_string(),
                })?
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
            name: s
                .name
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "name".to_string(),
                    parent_object_name: "ToolProvisioner".to_string(),
                })?,
            executable: s.executable.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "executable".to_string(),
                    parent_object_name: "ToolProvisioner".to_string(),
                }
            })?,
        })
    }
}

impl TryFrom<ParsedToolProvisioningCommands> for NormalizedToolProvisioningCommands {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedToolProvisioningCommands) -> Result<Self, Self::Error> {
        Ok(Self {
            install_command_prefix_args: s.install_command_prefix_args,
            install_command: s.install_command.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "installCommand".to_string(),
                    parent_object_name: "ToolProvisioningCommands".to_string(),
                }
            })?,
            verify_command_prefix_args: s.verify_command_prefix_args,
            verify_command: s.verify_command.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "verifyCommand".to_string(),
                    parent_object_name: "ToolProvisioningCommands".to_string(),
                }
            })?,
            uninstall_command_prefix_args: s.uninstall_command_prefix_args,
            uninstall_command: s.uninstall_command.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "uninstallCommand".to_string(),
                    parent_object_name: "ToolProvisioningCommands".to_string(),
                }
            })?,
        })
    }
}
