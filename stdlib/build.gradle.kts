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
    register("evalStdlib") {
      projectDir.set(file("."))
      sourceModules.set(fileTree(projectDir) { include("**/*.pkl") })
      outputFile.set(file("${layout.buildDirectory.get()}/%{moduleName}.%{outputFormat}"))
    }
  }

  project {
    resolvers {
      register("resolveStdlibDeps") {
        projectDirectories.from(file("."))
      }
    }

    packagers {
      register("makeStdlibPackage") {
        projectDirectories.from(file("."))
      }
    }
  }
}
