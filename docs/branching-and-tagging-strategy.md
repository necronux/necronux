<!--
# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: CC-BY-SA-4.0
# ==-----------------------------------------------------------== #
-->

# Git Branching & Tagging Strategy

## Overview

This strategy ensures a **clean version history**, **stable releases**, and **proper versioning**
in a **monorepo**. It follows **Semantic Versioning (SemVer)** and defines clear branching rules
to manage multiple products effectively.

The following diagram illustrates the key branching flows and how they interact:
```
      {bump/bugfix/feature}/*
         -----------------  #-> Always Rebase & FF Merge
develop /                 \
--------------------------[-]--------------------------------[-]---------------------------------->
         \   \   \   \   \                                   /
          \   \   \   \   \                                 /
           |   |   |   |   |              ------------------
           |   |   |   |   |             |    release-sync/{hotfix/bugfix/feature}/product-A/*
           |   |   |   |   |             |    #-> Rebase w/ latest develop & fix regression issues
           |   |   |   |   |             |    #-> Ensure this is merged before next release
           |   |   |   |   |             |    #-> of that product
           |   |   |   |   |             |
           |   |   |   |    -------------------------------> X
           |   |   |   |                 |   release/product-B/1.22.x
           |   |   |   |                 |
           |   |   |   |                 |
           |   |   |    -------------------------------> X
           |   |   |                     |   release/product-B/v1.21.x
           |   |   |                     |
           |   |   |                     |
           |    -------------------------------> X
           |  release/product-A/v1.8.x   |
           |                            /
           |                           /
            -------------------------[-]----> X
release/product-A/v1.7.x \           /
                          -----------
            {hotfix/bugfix/feature}/product-A/*

            bump/product-A/*   #-> Do Not Merge this to develop
```

## Key Rules

- **Always rebase & fast-forward merge** before integrating changes.
- **No direct pushes** to `develop` or release branches.
- **Release branches** are maintained as per the support policy and can be created retroactively using tags.
- **Hotfix releases** must be tagged using the next patch version. If unavailable, use `+hotfix.N`.
- **No automated branch cleanup**; branches are retained based on the support policy to prevent accidental deletions.
- **Do not merge bump branches** (`bump/product-P/*`) from release branches to `develop`,
  neither directly nor through release-sync branches.
- **Ensure all release changes are merged back to `develop`** before the next release of that product.
- **Avoid cherry-picking** between branches unless absolutely necessary.

---

## Branching Strategy

### **Long-Lived Branches**
- `develop` → The main development branch containing the latest changes.

### **Release Branches**
- `release/product-P/vX.Y.x` → Stabilization & maintenance per SemVer (MAJOR.MINOR).

### **Short-Lived Branches**
- **For Active Development:**
  - `bump/*` → Version bumps for any product in `develop`.
  - `feature/*` → New features for any product in `develop`.
  - `bugfix/*` → Bug fixes for any product in `develop`.

- **For Release Stabilization:**
  - `bump/product-P/*` → Version bumps in `release/product-P/vX.Y.x`.
  - `feature/product-P/*` → New features in `release/product-P/vX.Y.x`.
  - `bugfix/product-P/*` → Bug fixes in `release/product-P/vX.Y.x`.

- **For Hotfixes:**
  - `hotfix/product-P/*` → Urgent fixes in `release/product-P/vX.Y.x`.

- **For Syncing Back release changes to `develop`:**
  - `release-sync/feature/product-P/*` → Sync new features from release to `develop`.
  - `release-sync/bugfix/product-P/*` → Sync bug fixes from release to `develop`.
  - `release-sync/hotfix/product-P/*` → Sync hotfixes from release to `develop`.

---

## Release Changes Syncing

- **Release Branches**
  - Created from `develop`, maintained until the support period ends.
  - All fixes and features must be merged back into `develop` via `release-sync/{feature/bugfix/hotifx}/*` branches.
  - **Before merging a `release-sync` branch to `develop`, rebase it onto the latest `develop` and
  fix any regression bugs** to ensure stability.

---

## Semantic Versioning (SemVer)

We follow [Semantic Versioning 2.0.0](https://semver.org/) for all products in this monorepo.

---

## Version Handling

### **Development Versions**
- Use the `-dev` suffix for versions in `develop` (SemVer pre-release format).
  - Example: `1.7.0-dev`
- **Do NOT use** `-snapshot` as it implies a point-in-time tag.
- Development versions span multiple commits and are not final releases.

### **Version Bumping Rules**
- After branching for a release, bump the `develop` version according to SemVer:
  - If current is `1.7.0-dev`, bump to:
    - `1.8.0-dev` (for a minor update)
    - `2.0.0-dev` (if breaking changes are introduced)
  - **Only bump versions for affected products/components.**

### **Valid SemVer Tag Sequence**
Example of a correctly structured release cycle:
| Release Stage | Tag Format | Example |
|--------------|-----------|---------|
| Minor Release Candidate | `product-P@X.Y.Z-rc.N` | `product-A@1.7.0-rc.1` |
| Minor Stable Release | `product-P@X.Y.Z` | `product-A@1.7.0` |
| Patch Release Candidate | `product-P@X.Y.Z-rc.N` | `product-A@1.7.1-rc.1` |
| Patch Stable Release | `product-P@X.Y.Z` | `product-A@1.7.1` |

---

## Tagging Strategy

### **Stable Releases**
- Format: `product-P@X.Y.Z`
  - Example: `product-A@1.7.0`

### **Pre-release Versions**
- Format: `product-P@X.Y.Z-[identifier].[N]`
  - Example: `product-A@1.7.0-rc.1`

### **Hotfixes (if no patch is available)**
- Format: `product-P@X.Y.Z+hotfix.N`
  - Example: `product-A@1.7.1+hotfix.1`

---

## Further Reading
- [Semantic Versioning 2.0.0](https://semver.org/)
- See [release-process.md](/docs/release-process.md) for a **detailed release process**.
