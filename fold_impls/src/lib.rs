// The general idea is to extract all the child `impl` blocks,
// verify that they have the same form ("shell"),
// then coalesce all the substance ("meat"),
// into a single `impl` block with that same form.

#![forbid(unsafe_code)]

use proc_macro_error::{abort_call_site, proc_macro_error};
use std::{cell::RefCell, mem::take};
use syn::{parse_macro_input, visit_mut::VisitMut};

thread_local! {
 static IMPLS: RefCell<Vec<syn::ItemImpl>> = RefCell::new(vec![]);
}

struct ImplExtractor;

impl VisitMut for ImplExtractor {
 fn visit_stmt_mut(&mut self, node: &mut syn::Stmt) {
  if let syn::Stmt::Item(syn::Item::Impl(item_impl)) = node {
   IMPLS.with(|impls| impls.borrow_mut().push(item_impl.to_owned()));
   *node = syn::parse_quote!(();); // replace the impl with a very boring stmt
  }
  syn::visit_mut::visit_stmt_mut(self, node); // visit node's children
 }
}

// @mark fold_impls
#[proc_macro_attribute]
#[proc_macro_error]
pub fn fold_impls(
 _args: proc_macro::TokenStream,
 stmt: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
 let mut outer = parse_macro_input!(stmt as syn::Stmt);

 // Extract (find and remove) impls
 ImplExtractor.visit_stmt_mut(&mut outer);

 let mut f = if let syn::Stmt::Item(syn::Item::Fn(f)) = outer {
  f
 } else {
  abort_call_site!("#[fold_impls] needs to be declared on a function.")
 };

 // Separate meat (impl bodies) from shells (impl declarations)
 let (meat, mut shells) = IMPLS.with(|i| -> (Vec<_>, Vec<_>) {
  i.borrow_mut().drain(..).map(|mut s| (take(&mut s.items), s)).unzip()
 });

 // Verify impl shells match
 shells.dedup();
 let mut shell = match shells.len() {
  0 => abort_call_site!("No impl blocks found."),
  1 => shells.drain(..).next().unwrap(),
  _ => abort_call_site!("All impl blocks must have the same outer definition."),
 };

 // Put bodies into one shell
 shell.items = meat.into_iter().flatten().collect();

 f.block.stmts.push(syn::Stmt::Item(syn::Item::Impl(shell)));

 quote::quote!(#f).into()
}
