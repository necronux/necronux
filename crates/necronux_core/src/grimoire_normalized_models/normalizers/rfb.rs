// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::NormalizeGrimoireError;
use std::result as stdrt;

pub fn ensure_req_field_is_not_missing<T>(
    val: Option<T>,
    field_name: &str,
    parent_object_name: &str,
) -> stdrt::Result<T, NormalizeGrimoireError> {
    val.ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
        field_name: field_name.to_string(),
        parent_object_name: parent_object_name.to_string(),
    })
}
pub fn ensure_top_level_req_field_is_not_missing<T>(
    val: Option<T>,
    field_name: &str,
) -> stdrt::Result<T, NormalizeGrimoireError> {
    val.ok_or_else(|| NormalizeGrimoireError::MissingTopLevelRequiredField {
        field_name: field_name.to_string(),
    })
}
