// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SchemaField<T> {
    Absent,     // field missing in this schema version
    Present(T), // field exists
}

impl<T> SchemaField<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            SchemaField::Present(v) => Some(v),
            SchemaField::Absent => None,
        }
    }

    pub fn as_option(&self) -> Option<&T> {
        match self {
            SchemaField::Present(v) => Some(v),
            SchemaField::Absent => None,
        }
    }
}
