// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::ValidateGrimoireError;
use std::result as stdrt;

pub fn ensure_vec_is_not_empty<T>(
    val: Vec<T>,
    field_name: &str,
    parent_object_name: &str,
) -> stdrt::Result<Vec<T>, ValidateGrimoireError> {
    if val.is_empty() {
        Err(ValidateGrimoireError::EmptyFieldValue {
            field_name: field_name.to_string(),
            value: "[]".to_string(),
            parent_object_name: parent_object_name.to_string(),
        })
    } else {
        Ok(val)
    }
}
pub fn ensure_some_vec_is_not_empty<T>(
    val: Option<Vec<T>>,
    field_name: &str,
    parent_object_name: &str,
) -> stdrt::Result<Option<Vec<T>>, ValidateGrimoireError> {
    match val {
        Some(v) => Ok(Some(ensure_vec_is_not_empty(
            v,
            field_name,
            parent_object_name,
        )?)),
        None => Ok(None),
    }
}
