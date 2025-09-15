// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::LogSetup;
use tracing_fancytree::FancyTree;

impl LogSetup {
    pub fn stderr_fancytree_layer(&self) -> FancyTree<std::io::Stderr> {
        FancyTree::new(std::io::stderr(), self.color, self.unicode)
    }
}
