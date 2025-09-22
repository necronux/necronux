// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    grimoire_unified_model::{
        SchemaField,
        model::{
            UnifiedProvisioningStrategy, UnifiedTool, UnifiedToolProvisioner,
            UnifiedToolProvisionerFallbacks, UnifiedToolProvisioningBinding,
            UnifiedToolProvisioningCommands,
        },
    },
    models::v0::{
        ValidatedProvisioningStrategy, ValidatedTool, ValidatedToolProvisioner,
        ValidatedToolProvisionerFallbacks, ValidatedToolProvisioningBinding,
        ValidatedToolProvisioningCommands,
    },
};

impl From<ValidatedTool> for UnifiedTool {
    fn from(s: ValidatedTool) -> Self {
        Self {
            name: SchemaField::Present(s.name),
            executable: SchemaField::Present(s.executable),
            provisioning_strategy: SchemaField::Present(
                s.provisioning_strategy
                    .into_iter()
                    .map(|p| p.into())
                    .collect(),
            ),
        }
    }
}

impl From<ValidatedProvisioningStrategy> for UnifiedProvisioningStrategy {
    fn from(s: ValidatedProvisioningStrategy) -> Self {
        match s {
            ValidatedProvisioningStrategy::Binding(b) => {
                UnifiedProvisioningStrategy::Binding(b.into())
            }
            ValidatedProvisioningStrategy::Fallbacks(f) => {
                UnifiedProvisioningStrategy::Fallbacks(f.into())
            }
        }
    }
}

impl From<ValidatedToolProvisioningBinding> for UnifiedToolProvisioningBinding {
    fn from(s: ValidatedToolProvisioningBinding) -> Self {
        Self {
            tool_provisioner: SchemaField::Present(s.tool_provisioner.into()),
            provisioning_commands: SchemaField::Present(s.provisioning_commands.into()),
        }
    }
}

impl From<ValidatedToolProvisioner> for UnifiedToolProvisioner {
    fn from(s: ValidatedToolProvisioner) -> Self {
        Self {
            name: SchemaField::Present(s.name),
            executable: SchemaField::Present(s.executable),
        }
    }
}

impl From<ValidatedToolProvisioningCommands> for UnifiedToolProvisioningCommands {
    fn from(s: ValidatedToolProvisioningCommands) -> Self {
        Self {
            install_command_prefix_args: SchemaField::Present(s.install_command_prefix_args),
            install_command: SchemaField::Present(s.install_command),
            verify_command_prefix_args: SchemaField::Present(s.verify_command_prefix_args),
            verify_command: SchemaField::Present(s.verify_command),
            uninstall_command_prefix_args: SchemaField::Present(s.uninstall_command_prefix_args),
            uninstall_command: SchemaField::Present(s.uninstall_command),
        }
    }
}

impl From<ValidatedToolProvisionerFallbacks> for UnifiedToolProvisionerFallbacks {
    fn from(s: ValidatedToolProvisionerFallbacks) -> Self {
        Self {
            fallbacks: SchemaField::Present(s.fallbacks.into_iter().map(|f| f.into()).collect()),
        }
    }
}
