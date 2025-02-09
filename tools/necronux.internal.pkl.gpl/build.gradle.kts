// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

plugins {
  id("org.pkl-lang") version "0.27.2"
}

pkl {
  evaluators {
    register("evalWorkflowsGPL") {
      projectDir.set(file("."))
      sourceModules.set(fileTree(projectDir) { include("internal/.github/workflows/*.pkl") })
      outputFile.set(file("${rootProject.projectDir}/.github/workflows/%{moduleName}.%{outputFormat}"))
      outputFormat.set("yml")
    }
  }
  evaluators {
    register("evalWorkflowsCommonsGPL") {
      projectDir.set(file("."))
      sourceModules.set(fileTree(projectDir) { include("internal/.github/workflows/commons/*.pkl") })
      outputFile.set(file("${layout.buildDirectory.get()}/%{moduleName}.%{outputFormat}"))
      outputFormat.set("pcf")
    }
  }

  project {
    resolvers {
      register("resolveInternalPklGPLDeps") {
        projectDirectories.from(file("."))
      }
    }
  }
}
