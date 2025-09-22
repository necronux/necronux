// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

#[derive(Debug)]
pub struct NormalizedTool {
    pub name: String,
    pub executable: String,
    pub provisioning_strategy: Vec<NormalizedProvisioningStrategy>,
}

#[derive(Debug)]
pub enum NormalizedProvisioningStrategy {
    Binding(NormalizedToolProvisioningBinding),
    Fallbacks(NormalizedToolProvisionerFallbacks),
}

#[derive(Debug)]
pub struct NormalizedToolProvisioningBinding {
    pub tool_provisioner: NormalizedToolProvisioner,
    pub provisioning_commands: NormalizedToolProvisioningCommands,
}

#[derive(Debug)]
pub struct NormalizedToolProvisionerFallbacks {
    pub fallbacks: Vec<NormalizedToolProvisioningBinding>,
}

#[derive(Debug)]
pub struct NormalizedToolProvisioner {
    pub name: String,
    pub executable: String,
}

#[derive(Debug)]
pub struct NormalizedToolProvisioningCommands {
    pub install_command_prefix_args: Option<String>,
    pub install_command: String,
    pub verify_command_prefix_args: Option<String>,
    pub verify_command: String,
    pub uninstall_command_prefix_args: Option<String>,
    pub uninstall_command: String,
}
