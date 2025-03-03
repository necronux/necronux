// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: Apache-2.0
// ==-----------------------------------------------------------== //

rootProject.name = "necronux"

include("stdschema")
include("tools:ci:necronux.internal.ci.apache")
include("tools:ci:necronux.internal.ci.gpl")

pluginManagement {
    repositories {
        mavenCentral()
    }
}
