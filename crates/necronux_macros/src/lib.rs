// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn trace_instrument(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    // Convert attribute arguments to token stream
    let args = if attr.is_empty() {
        quote! {}
    } else {
        let attr_tokens = proc_macro2::TokenStream::from(attr);
        quote! { #attr_tokens }
    };

    if cfg!(feature = "trace") {
        quote! {
            #[tracing::instrument(#args)]
            #func
        }
    } else {
        quote! {
            #func
        }
    }
    .into()
}
