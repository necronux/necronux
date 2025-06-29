// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

pub fn make_fetch_options<'cb>() -> git2::FetchOptions<'cb> {
    let mut fetch_opts = git2::FetchOptions::new();
    fetch_opts
        .download_tags(git2::AutotagOption::All)
        .update_fetchhead(true)
        .prune(git2::FetchPrune::On);
    fetch_opts
}
