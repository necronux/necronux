// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedTool {
    pub name: String,

    pub executable: String,

    pub provisioning_strategy: Vec<ParsedProvisioningStrategy>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ParsedProvisioningStrategy {
    Binding(ParsedToolProvisioningBinding),
    Fallbacks(ParsedToolProvisionerFallbacks),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedToolProvisioningBinding {
    pub tool_provisioner: ParsedToolProvisioner,

    pub provisioning_commands: ParsedToolProvisioningCommands,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedToolProvisionerFallbacks {
    pub fallbacks: Vec<ParsedToolProvisioningBinding>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedToolProvisioner {
    pub name: String,

    pub executable: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedToolProvisioningCommands {
    pub install_command_prefix_args: Option<String>,

    pub install_command: String,

    pub verify_command_prefix_args: Option<String>,

    pub verify_command: String,

    pub uninstall_command_prefix_args: Option<String>,

    pub uninstall_command: String,
}
