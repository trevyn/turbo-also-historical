use once_cell::sync::Lazy;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use std::cell::RefCell;
use std::sync::Mutex;
use syn::visit_mut::VisitMut;

#[proc_macro]
pub fn struct_lift(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
 dbg!(&item);

 let mut syntax_tree: syn::Item = syn::parse(item).unwrap();
 // StructExtract.visit_stmt_mut(&mut syntax_tree);
 println!("syntax tree is :{}", quote!(#syntax_tree));

 let mut tokens = proc_macro2::TokenStream::new();

 // @mark struct_lift
 STRUCTS.with(|structs| {
  let structs = structs.borrow();
  dbg!(&structs);
  // #(#structs)*
  // #[allow(dead_code, unused_must_use, clippy::no_effect)]

  tokens = syn::parse_quote! {
   #syntax_tree

  };
  dbg!(&tokens);
  let str = dbg!(format!("{}", &tokens));
  eprintln!("{}", &tokens);

  // println!(
  //  "{}",
  //  quote! {
  //   #(#structs)*
  //   #syntax_tree
  //  }
  // );
 });

 tokens.into()

 // let InputTokens { structs, rest } = parse_macro_input!(item);

 // (quote! {
 // })
 // .into()

 // let attr = parse_macro_input!(attr as proc_macro2::TokenStream);
 // let item = parse_macro_input!(item as proc_macro2::TokenStream);

 // match struct_lift_macro::struct_lift(attr, item) {
 //  Ok(tokens) => tokens.into(),
 //  Err(err) => TokenStream::from(err.to_compile_error()),
 // }
}

// #[derive(Debug)]
// struct InputTokens {
//  structs: Vec<proc_macro2::TokenStream>,
//  rest: proc_macro2::TokenStream,
// }

thread_local! {
 // static STRUCTS: Lazy<Mutex<Vec<syn::Stmt>>> = Lazy::new(|| Mutex::new(Vec::new()));
      static STRUCTS: RefCell<Vec<syn::Stmt>> = RefCell::new(Vec::new());

}

struct StructExtract;

impl VisitMut for StructExtract {
 fn visit_stmt_mut(&mut self, node: &mut syn::Stmt) {
  if let syn::Stmt::Item(syn::Item::Struct(s)) = &node {
   dbg!(s);
   STRUCTS.with(|structs| {
    structs.borrow_mut().push(syn::Stmt::Item(syn::Item::Struct(s.to_owned())));
   });

   // let structs = STRUCTS.lock().unwrap();
   // *structs.push(s);
   *node = syn::parse_quote!((););
  }
  // if let Expr::Lit(expr) = &node {
  //  if let Lit::Int(int) = &expr.lit {
  //   if int.suffix() == "u256" {
  //    let digits = int.base10_digits();
  //    let unsuffixed: LitInt = syn::parse_str(digits).unwrap();
  //    *node = parse_quote!(bigint::u256!(#unsuffixed));
  //    return;
  //   }
  //  }
  // }

  // Delegate to the default impl to visit nested expressions.
  syn::visit_mut::visit_stmt_mut(self, node);
 }
}

// #[derive(Debug)]
// struct AttributeTokens(proc_macro2::TokenStream);

// impl Parse for StructExtract {
//  fn parse(input: ParseStream) -> syn::Result<Self> {
//   // Punct {
//   //     ch: '#',
//   //     spacing: Alone,
//   //     span: #0 bytes(4157..4158),
//   // },
//   // let _pound_token: Result<Token![#], _> = dbg!(input.parse());
//   let itemimpl: Result<syn::ItemImpl, _> = dbg!(input.parse());

//   // Group {
//   //     delimiter: Bracket,
//   //     stream: TokenStream [
//   //         Ident {
//   //             ident: "graphql_object",
//   //             span: #0 bytes(4159..4173),
//   //         },
//   //     ],
//   //     span: #0 bytes(4158..4174),
//   // },
//   // let _group = dbg!(input.parse());

//   // Ident {
//   //     ident: "impl",
//   //     span: #0 bytes(4175..4179),
//   // },
//   // Ident {
//   //     ident: "Query",
//   //     span: #0 bytes(4180..4185),
//   // },
//   // Group {
//   // let result_type = input.parse::<P>().ok();

//   let span = input.span();
//   let result_type = input.parse::<Type>().ok();
//   let sql = input.parse::<LitStr>().ok().map(|s| s.value());

//   Ok(InputTokens { structs: vec![], rest: proc_macro2::TokenStream::new() })
//  }
// }

// #[proc_macro_attribute]
// // #[proc_macro_error]
// pub fn struct_lift(_attr: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
//  let ExtractedTokens { structs, rest } = parse_macro_input!(input);

//  // let (structs, rest) = extract_structs(item)
//  // quote!{structs, rest}

//  Ok(dbg!(item))
// }
