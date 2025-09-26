// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::ValidateGrimoireError;
use std::result as stdrt;

pub fn ensure_str_is_not_empty(
    val: String,
    field_name: &str,
    parent_object_name: &str,
) -> stdrt::Result<String, ValidateGrimoireError> {
    if val.is_empty() {
        return Err(ValidateGrimoireError::EmptyFieldValue {
            field_name: field_name.to_string(),
            value: val.clone(),
            parent_object_name: parent_object_name.to_string(),
        });
    } else {
        Ok(val)
    }
}
pub fn ensure_some_str_is_not_empty(
    val: Option<String>,
    field_name: &str,
    parent_object_name: &str,
) -> stdrt::Result<Option<String>, ValidateGrimoireError> {
    match val {
        Some(v) => Ok(Some(ensure_str_is_not_empty(
            v,
            field_name,
            parent_object_name,
        )?)),
        None => Ok(None),
    }
}
