// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

mod cmd_exec;
pub mod error;
mod grimoire_pipeline;
mod parsers;
mod spell_magic;

pub use cmd_exec::*;
pub use grimoire_pipeline::*;
pub use parsers::*;

// Additional documentation
//
// Unification:
// bool and Option<bool> -> bool and use sensible default
// Vec<T> and Option<Vec<T>> -> SchemaField<Vec<T>> and create empty if None
// HashMap<K, V> and Option<HashMap<K, V>> -> SchemaField<HashMap<K, V>> and create empty if None
// For others:
// Option<T> -> SchemaField<Option<T>>
// T -> SchemaField<T>
//
// RFBs are Required Field Barriers for serde to not let serde throw its own unhelpful error
// when a required field is missing. Instead a new helpful error is thrown for the same in
// normalization. Instead of a new enum, Option is used when parsing to act as a RFB. This is
// because RFBs work as intended only with Option due to how serde sees Option.
//
// Working with RFBs in
//
// normalization:
// Option<bool> -> bool
// Option<Vec<_>> -> Vec<_>
// Option<HashMap<_, _>> -> HashMap<_, _>
//
// validation:
// bool -> bool
// Vec<_> -> Vec<_> ensure Vec is not empty
// HashMap<_, _> -> HashMap<_, _> ensure HashMap is not empty
// also consider cross fields validation strategies
