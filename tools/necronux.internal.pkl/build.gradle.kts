// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: Apache-2.0
// ==-----------------------------------------------------------== //

plugins {
  id("org.pkl-lang") version "0.27.2"
}

pkl {
  evaluators {
    register("evalWorkflows") {
      projectDir.set(file("."))
      sourceModules.set(fileTree(projectDir) { include("internal/.github/workflows/*.pkl") })
      outputFile.set(file("${rootProject.projectDir}/.github/workflows/%{moduleName}.%{outputFormat}"))
      outputFormat.set("yml")
    }
  }

  project {
    resolvers {
      register("resolveInternalPklDeps") {
        projectDirectories.from(file("."))
      }
    }
  }
}
