// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::{CastSpellError, CastSpellErrorWithContext},
    grimoire_unified_model::model::UnifiedGrimoire,
};
use necronux_utils::trace_instrument;
use tracing::debug;

#[derive(Debug)]
pub enum CastSpellEvent {
    Metadata {
        name: String,
        description: Option<String>,
        keywords: Option<Vec<String>>,
    },
    InvocationStarted {
        command: String,
    },
    InvocationResult {
        command: String,
        success: bool,
        stdout: String,
        stderr: String,
    },
    Finished {
        success: bool,
    },
}

impl UnifiedGrimoire {
    #[trace_instrument(level = "debug")]
    pub fn cast_spell(
        &self,
        id: &str,
    ) -> Result<impl Iterator<Item = CastSpellEvent>, CastSpellErrorWithContext> {
        debug!(
            id = %id,
            "Casting spell..."
        );

        let spell_stream = self
            .cast_spell_inner(id)
            .map_err(|e| CastSpellErrorWithContext { source: e })?;

        debug!(
            id = %id,
            "Successfully cast spell"
        );
        Ok(spell_stream)
    }

    fn cast_spell_inner(
        &self,
        id: &str,
    ) -> Result<impl Iterator<Item = CastSpellEvent>, CastSpellError> {
        // Fetch spell
        let chapters = self
            .core_contents
            .as_option()
            .ok_or_else(|| CastSpellError::UnsupportedGrimoireSchemaVersion {
                version: self.schema_version_info.schema_version.to_string(),
                field_name: "coreContents".to_string(),
            })?
            .chapters
            .as_option()
            .ok_or_else(|| CastSpellError::UnsupportedGrimoireSchemaVersion {
                version: self.schema_version_info.schema_version.to_string(),
                field_name: "chapters".to_string(),
            })?
            .as_ref();
        let spell = match chapters {
            Some(chapters_map) => chapters_map
                .values()
                .find_map(|chapter| chapter.spells.as_option()?.as_ref()?.get(id))
                .ok_or_else(|| CastSpellError::SpellNotFound { id: id.to_string() })?,
            None => {
                return Err(CastSpellError::SpellNotFound { id: id.to_string() });
            }
        };

        // Get spell metadata
        let name = spell
            .name
            .as_option()
            .map(|s| s.clone())
            .unwrap_or_else(|| "<unnamed spell>".to_string());
        let description = spell.description.as_option().and_then(|d| d.clone());
        let keywords = spell.keywords.into_option().and_then(|k| k.clone());

        // Build the event stream
        let metadata_event = CastSpellEvent::Metadata {
            name,
            description,
            keywords,
        };

        let cast_event = CastSpellEvent::InvocationStarted {
            command: spell.invocations.cast.execution_command,
        };

        Ok(vec![metadata_event, cast_event].into_iter())
    }
}
