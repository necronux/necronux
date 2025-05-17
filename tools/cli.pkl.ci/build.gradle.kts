// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: Apache-2.0
// ==-----------------------------------------------------------== //

plugins {
  id("org.pkl-lang") version "0.28.2"
}

pkl {
  evaluators {
    register("evalCliPklCiModules") {
      projectDir.set(file("."))
      sourceModules.set(fileTree(projectDir) { include("pkl/.github/workflows/modules/*.pkl") })
      outputFile.set(file("${layout.buildDirectory.get()}/%{moduleName}.%{outputFormat}"))
      outputFormat.set("pcf")
    }
  }
  evaluators {
    register("evalCliPklCiWorkflows") {
      projectDir.set(file("."))
      sourceModules.set(fileTree(projectDir) { include("pkl/.github/workflows/*.pkl") })
      outputFile.set(file("${rootProject.projectDir}/.github/workflows/%{moduleName}.%{outputFormat}"))
      outputFormat.set("yml")
    }
  }

  project {
    resolvers {
      register("resolveCliPklCi") {
        projectDirectories.from(file("."))
      }
    }
  }
}
