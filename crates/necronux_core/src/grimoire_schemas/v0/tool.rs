// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub name: String,

    pub executable: String,

    pub provisioning_strategy: Vec<ProvisioningStrategy>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ProvisioningStrategy {
    Binding(ToolProvisioningBinding),
    Fallbacks(ToolProvisionerFallbacks),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolProvisioningBinding {
    pub tool_provisioner: ToolProvisioner,

    pub provisioning_commands: ToolProvisioningCommands,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolProvisionerFallbacks {
    pub fallbacks: Vec<ToolProvisioningBinding>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolProvisioner {
    pub name: String,

    pub executable: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolProvisioningCommands {
    pub install_command_prefix_args: Option<String>,

    pub install_command: String,

    pub verify_command_prefix_args: Option<String>,

    pub verify_command: String,

    pub uninstall_command_prefix_args: Option<String>,

    pub uninstall_command: String,
}
