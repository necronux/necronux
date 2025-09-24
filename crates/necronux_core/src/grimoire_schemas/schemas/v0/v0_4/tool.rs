// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedTool {
    // Option as RFB (See mod.rs)
    pub name: Option<String>,

    // Option as RFB (See mod.rs)
    pub executable: Option<String>,

    // Option as RFB (See mod.rs)
    pub provisioning_strategy: Option<Vec<ParsedProvisioningStrategy>>,
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
    // Option as RFB (See mod.rs)
    pub tool_provisioner: Option<ParsedToolProvisioner>,

    // Option as RFB (See mod.rs)
    pub provisioning_commands: Option<ParsedToolProvisioningCommands>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedToolProvisionerFallbacks {
    // Option as RFB (See mod.rs)
    pub fallbacks: Option<Vec<ParsedToolProvisioningBinding>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedToolProvisioner {
    // Option as RFB (See mod.rs)
    pub name: Option<String>,

    // Option as RFB (See mod.rs)
    pub executable: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedToolProvisioningCommands {
    pub install_command_prefix_args: Option<String>,

    // Option as RFB (See mod.rs)
    pub install_command: Option<String>,

    pub verify_command_prefix_args: Option<String>,

    // Option as RFB (See mod.rs)
    pub verify_command: Option<String>,

    pub uninstall_command_prefix_args: Option<String>,

    // Option as RFB (See mod.rs)
    pub uninstall_command: Option<String>,
}
