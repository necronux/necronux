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
    register("evalWorkflows") {
      projectDir.set(file("."))
      sourceModules.set(fileTree(projectDir) { include("internal/.github/workflows/*.pkl") })
      outputFile.set(file("${rootProject.projectDir}/.github/workflows/%{moduleName}.%{outputFormat}"))
      outputFormat.set("yml")
    }
  }
  evaluators {
    register("evalWorkflowsModules") {
      projectDir.set(file("."))
      sourceModules.set(fileTree(projectDir) { include("internal/.github/workflows/modules/*.pkl") })
      outputFile.set(file("${layout.buildDirectory.get()}/%{moduleName}.%{outputFormat}"))
      outputFormat.set("pcf")
    }
  }
  evaluators {
    register("evalWorkflowsHelpers") {
      projectDir.set(file("."))
      sourceModules.set(fileTree(projectDir) { include("internal/.github/workflows/helpers/*.pkl") })
      outputFile.set(file("${layout.buildDirectory.get()}/%{moduleName}.%{outputFormat}"))
      outputFormat.set("pcf")
    }
  }

  project {
    resolvers {
      register("resolveInternalCI") {
        projectDirectories.from(file("."))
      }
    }
  }
}
