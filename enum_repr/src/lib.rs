extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::DeriveInput;

#[proc_macro_derive(EnumFromU8)]
pub fn enum_from_u8(input: TokenStream) -> TokenStream {
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
  };

  gen.into()
}

#[proc_macro_derive(EnumFromU16)]
pub fn enum_from_u16(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  let name = &ast.ident;

  let variants = match &ast.data {
    syn::Data::Enum(e) => e.variants.iter().map(|v| v.ident.clone()),
    _ => panic!("EnumFrom can only be used with enums"),
  };

  let gen = quote! {
      impl From<u16> for #name {
          fn from(val: u16) -> Self {
              match val {
                  #(
                      x if x == #name::#variants as u16 => #name::#variants,
                  )*
                  _ => panic!("Invalid value for enum {}", val),
              }
          }
      }
  };

  gen.into()
}
