// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tool {
    pub name: String,

    pub executable: String,

    #[serde(rename = "provisioningStrategy")]
    pub provisioning_strategy: Vec<ProvisioningStrategy>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ProvisioningStrategy {
    Binding(ToolProvisioningBinding),
    Fallbacks(ToolProvisionerFallbacks),
}

#[derive(Debug, Deserialize)]
pub struct ToolProvisioningBinding {
    #[serde(rename = "toolProvisioner")]
    pub tool_provisioner: ToolProvisioner,

    #[serde(rename = "provisioningCommands")]
    pub provisioning_commands: ToolProvisioningCommands,
}

#[derive(Debug, Deserialize)]
pub struct ToolProvisionerFallbacks {
    pub fallbacks: Vec<ToolProvisioningBinding>,
}

#[derive(Debug, Deserialize)]
pub struct ToolProvisioner {
    pub name: String,

    pub executable: String,
}

#[derive(Debug, Deserialize)]
pub struct ToolProvisioningCommands {
    #[serde(rename = "installCommandPrefixArgs")]
    pub install_command_prefix_args: Option<String>,

    #[serde(rename = "installCommand")]
    pub install_command: String,

    #[serde(rename = "verifyCommandPrefixArgs")]
    pub verify_command_prefix_args: Option<String>,

    #[serde(rename = "verifyCommand")]
    pub verify_command: String,

    #[serde(rename = "uninstallCommandPrefixArgs")]
    pub uninstall_command_prefix_args: Option<String>,

    #[serde(rename = "uninstallCommand")]
    pub uninstall_command: String,
}
