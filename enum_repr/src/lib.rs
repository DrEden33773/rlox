extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::DeriveInput;

#[proc_macro_derive(EnumU8)]
pub fn enum_u8(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();

  let name = &ast.ident;

  let variants = match &ast.data {
    syn::Data::Enum(e) => e.variants.iter().map(|v| v.ident.clone()),
    _ => panic!("EnumFrom can only be used with enums"),
  };

  let gen = quote! {
    impl From<u8> for #name {
      fn from(val: u8) -> Self {
        match val {
          #(
            x if x == #name::#variants as u8 => #name::#variants,
          )*
          _ => panic!("Invalid value for enum {}", val),
        }
      }
    }

    impl Into<u8> for #name {
      fn into(self) -> u8 {
        self as u8
      }
    }
  };

  gen.into()
}
