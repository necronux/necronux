// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::ValidateGrimoireError;
use std::{collections::HashMap, result as stdrt};

pub fn ensure_map_is_not_empty<K, V>(
    val: HashMap<K, V>,
    field_name: &str,
    parent_object_name: &str,
) -> stdrt::Result<HashMap<K, V>, ValidateGrimoireError> {
    if val.is_empty() {
        return Err(ValidateGrimoireError::EmptyFieldValue {
            field_name: field_name.to_string(),
            value: "{}".to_string(),
            parent_object_name: parent_object_name.to_string(),
        });
    } else {
        Ok(val)
    }
}
pub fn ensure_some_map_is_not_empty<K, V>(
    val: Option<HashMap<K, V>>,
    field_name: &str,
    parent_object_name: &str,
) -> stdrt::Result<Option<HashMap<K, V>>, ValidateGrimoireError> {
    match val {
        Some(v) => Ok(Some(ensure_map_is_not_empty(
            v,
            field_name,
            parent_object_name,
        )?)),
        None => Ok(None),
    }
}
