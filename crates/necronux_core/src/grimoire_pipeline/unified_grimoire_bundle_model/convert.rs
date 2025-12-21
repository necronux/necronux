// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::model::{
    UnifiedCommonMetadata, UnifiedGrimoire, UnifiedGrimoireBundle, UnifiedPackageMetadata,
};
use crate::{
    common_metadata_validated_model::model::ValidatedCommonMetadata,
    grimoire_validated_models::ValidatedGrimoire,
    package_metadata_validated_model::model::ValidatedPackageMetadata,
};
use necronux_utils::trace_instrument;
use tracing::debug;

#[trace_instrument(level = "debug", skip(self))]
pub fn into_unified_grimoire_bundle(
    package_metadata: UnifiedPackageMetadata,
    common_metadata: UnifiedCommonMetadata,
    grimoire: UnifiedGrimoire,
) -> UnifiedGrimoireBundle {
    debug!("Bundling unified components into unified grimoire bundle...");

    let bundle = UnifiedGrimoireBundle {
        package_metadata,
        common_metadata,
        grimoire,
    };

    debug!("Successfully bundled unifed components into unified grimoire bundle");
    bundle
}

impl ValidatedPackageMetadata {
    #[trace_instrument(
        level = "debug",
        name = "validated_package_metadata_into_unified",
        skip(self)
    )]
    pub fn into_unified(self) -> UnifiedPackageMetadata {
        debug!("Converting validated grimoire package metadata into unified grimoire package metadata...");

        let unified = self.into();

        debug!("Successfully converted validated grimoire package metadata into unified grimoire package metadata");
        unified
    }
}

impl ValidatedCommonMetadata {
    #[trace_instrument(
        level = "debug",
        name = "validated_common_metadata_into_unified",
        skip(self)
    )]
    pub fn into_unified(self) -> UnifiedCommonMetadata {
        debug!("Converting validated grimoire common metadata into unified grimoire common metadata...");

        let unified = self.into();

        debug!("Successfully converted validated grimoire common metadata into unified grimoire common metadata");
        unified
    }
}

impl ValidatedGrimoire {
    #[trace_instrument(level = "debug", name = "validated_grimoire_into_unified", skip(self))]
    pub fn into_unified(self) -> UnifiedGrimoire {
        debug!("Converting validated grimoire into unified grimoire...");

        let unified = match self {
            #[cfg(feature = "grimoire_schema_v0")]
            ValidatedGrimoire::V0_4(g_boxed) => {
                let g = *g_boxed;
                g.into()
            }
        };

        debug!("Successfully converted validated grimoire into unified grimoire");
        unified
    }
}
