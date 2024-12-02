//! [Mocktopus](https://docs.rs/mocktopus) procedural macros making items mockable
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/CodeSandwich/mocktopus/master/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/CodeSandwich/mocktopus/master/logo.png"
)]
// #![feature(proc_macro_diagnostic)]

extern crate proc_macro;

mod display_delegate;
mod header_builder;
mod item_injector;

use proc_macro::{Span, TokenStream};
use quote::ToTokens;
use log::{info, warn};

/// Procedural macro, makes items and their sub-items mockable
///
/// # Valid to annotate
/// - module definitions (makes all its valid to annotate items annotated)
///
/// ```
/// #[mockable]
/// mod module {
///     mod nested {
///         fn mockable() { ... }
///     }
/// }
/// ```
/// - standalone functions
///
/// ```
/// #[mockable]
/// fn mockable() { ... }
/// ```
/// - struct impl blocks (makes all functions inside mockable)
///
/// ```
/// #[mockable]
/// impl Structure {
///     fn mockable() { ... }
/// }
/// ```
/// - trait impl blocks (makes all functions inside mockable)
///
/// ```
/// #[mockable]
/// impl Trait for Structure {
///     fn mockable() { ... }
/// }
/// ```
/// - traits (makes all default functions inside mockable)
///
/// ```
/// #[mockable]
/// trait Trait {
///     fn mockable() { ... }
/// }
/// ```
/// # Invalid to annotate **(WILL FAIL TO COMPILE OR BREAK MOCKING!)**
///
/// - single functions in struct impls
///
/// ```
/// impl Structure {
///     #[mockable] //INVALID USAGE!
///     fn mockable() { ... }
/// }
/// ```
/// - single functions in trait impls
///
/// ```
/// impl Trait for Structure {
///     #[mockable] //INVALID USAGE!
///     fn mockable() { ... }
/// }
/// ```
/// - single default functions in traits
///
/// ```
/// trait Trait {
///     #[mockable] //INVALID USAGE!
///     fn mockable() { ... }
/// }
/// ```
/// # Indifferent to annotate
/// - already mockable items (inside annotated modules)
/// - module declarations
///
/// ```
/// #[mockable]
/// mod module;
/// ```
/// - const functions (they are impossible to mock)
/// - unsafe functions (they are impossible to mock)
/// - any macro generated items (they are impossible to mock)
/// - any other items
#[proc_macro_attribute]
pub fn mockable(_: TokenStream, token_stream: TokenStream) -> TokenStream {
    let mut item: syn::Item = match syn::parse(token_stream.clone()) {
        Ok(item) => item,
        Err(err) => {
            // Span::call_site()
            //     .warning("Failed to make code mockable")
            //     .error(format!("Failed to parse: {}", err))
            //     .emit();
            warn!("Failed to parse token stream: {}", err);
            return token_stream;
        }
    };
    item_injector::inject_item(&mut item);
    item.into_token_stream().into()
}

/// Procedural macro, guards items from being made mockable by enclosing item.
///
/// # Valid to annotate
/// - module definitions
///
/// ```
/// #[mockable]
/// mod module {
///     #[not_mockable]
///     mod nested {
///         fn not_mockable() { ... }
///     }
/// }
/// ```
/// - standalone functions
///
/// ```
/// #[mockable]
/// mod module {
///     #[not_mockable]
///     fn not_mockable() { ... }
/// }
/// ```
/// - struct impl blocks
///
/// ```
/// #[mockable]
/// mod module {
///     #[not_mockable]
///     impl Struct {
///         fn not_mockable() { ... }
///     }
/// }
/// ```
/// - trait impl blocks
///
/// ```
/// #[mockable]
/// mod module {
///     #[not_mockable]
///     impl Trait for Struct {
///         fn not_mockable() { ... }
///     }
/// }
/// ```
/// - traits
///
/// ```
/// #[mockable]
/// mod module {
///     #[not_mockable]
///     trait Trait {
///         fn not_mockable() { ... }
///     }
/// }
/// ```
/// - single functions in struct impls
///
/// ```
/// #[mockable]
/// impl Struct {
///     #[not_mockable]
///     fn not_mockable() { ... }
/// }
/// ```
/// - single functions in trait impls
///
/// ```
/// #[mockable]
/// impl Trait for Struct {
///     #[not_mockable]
///     fn not_mockable() { ... }
/// }
/// ```
/// - single default functions in traits
///
/// ```
/// #[mockable]
/// trait Trait {
///     #[not_mockable]
///     fn not_mockable() { ... }
/// }
/// ```
///
/// # Indifferent to annotate
/// - items not made mockable by enclosing item
/// - any other items
#[proc_macro_attribute]
pub fn not_mockable(_: TokenStream, token_stream: TokenStream) -> TokenStream {
    token_stream
}
