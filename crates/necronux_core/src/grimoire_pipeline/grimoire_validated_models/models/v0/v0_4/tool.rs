// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

#[derive(Debug)]
pub struct ValidatedTool {
    pub name: String,
    pub executable: String,
    pub provisioning_strategy: Vec<ValidatedProvisioningStrategy>,
}

#[derive(Debug)]
pub enum ValidatedProvisioningStrategy {
    Binding(ValidatedToolProvisioningBinding),
    Fallbacks(ValidatedToolProvisionerFallbacks),
}

#[derive(Debug)]
pub struct ValidatedToolProvisioningBinding {
    pub tool_provisioner: ValidatedToolProvisioner,
    pub provisioning_commands: ValidatedToolProvisioningCommands,
}

#[derive(Debug)]
pub struct ValidatedToolProvisionerFallbacks {
    pub fallbacks: Vec<ValidatedToolProvisioningBinding>,
}

#[derive(Debug)]
pub struct ValidatedToolProvisioner {
    pub name: String,
    pub executable: String,
}

#[derive(Debug)]
pub struct ValidatedToolProvisioningCommands {
    pub install_command_prefix_args: Option<String>,
    pub install_command: String,
    pub verify_installation_command_prefix_args: Option<String>,
    pub verify_installation_command: String,
    pub uninstall_command_prefix_args: Option<String>,
    pub uninstall_command: String,
}
