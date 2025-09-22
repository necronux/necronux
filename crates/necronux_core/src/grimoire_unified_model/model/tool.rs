// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::grimoire_unified_model::SchemaField;

#[derive(Debug)]
pub struct UnifiedTool {
    pub name: SchemaField<String>,
    pub executable: SchemaField<String>,
    pub provisioning_strategy: SchemaField<Vec<UnifiedProvisioningStrategy>>,
}

#[derive(Debug)]
pub enum UnifiedProvisioningStrategy {
    Binding(UnifiedToolProvisioningBinding),
    Fallbacks(UnifiedToolProvisionerFallbacks),
}

#[derive(Debug)]
pub struct UnifiedToolProvisioningBinding {
    pub tool_provisioner: SchemaField<UnifiedToolProvisioner>,
    pub provisioning_commands: SchemaField<UnifiedToolProvisioningCommands>,
}

#[derive(Debug)]
pub struct UnifiedToolProvisionerFallbacks {
    pub fallbacks: SchemaField<Vec<UnifiedToolProvisioningBinding>>,
}

#[derive(Debug)]
pub struct UnifiedToolProvisioner {
    pub name: SchemaField<String>,
    pub executable: SchemaField<String>,
}

#[derive(Debug)]
pub struct UnifiedToolProvisioningCommands {
    pub install_command_prefix_args: SchemaField<Option<String>>,
    pub install_command: SchemaField<String>,
    pub verify_command_prefix_args: SchemaField<Option<String>>,
    pub verify_command: SchemaField<String>,
    pub uninstall_command_prefix_args: SchemaField<Option<String>>,
    pub uninstall_command: SchemaField<String>,
}
