// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: Apache-2.0
// ==-----------------------------------------------------------== //

rootProject.name = "necronux"

include("stdschema")
include("tools:necronux.internal.pkl.apache")
include("tools:necronux.internal.pkl.gpl")

pluginManagement {
    repositories {
        mavenCentral()
    }
}
