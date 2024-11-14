// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use std::borrow::Cow;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Ident, LitStr};

#[proc_macro_attribute]
pub fn boxed(attr: TokenStream, item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as DeriveInput);
  let mut specified_struct_name: Option<LitStr> = None;
  let parser = syn::meta::parser(|meta| {
    if meta.path.is_ident("name") {
      specified_struct_name = Some(meta.value()?.parse()?);
      Ok(())
    } else {
      Err(meta.error("unsupported property"))
    }
  });
  parse_macro_input!(attr with parser);

  // retrieve the struct name from the input
  let struct_name = input.ident.clone();
  let struct_name_str = struct_name.to_string();
  let new_error_struct_name = match specified_struct_name {
    Some(name) => Cow::Owned(name.value()),
    None => match struct_name_str.strip_suffix("Kind") {
      Some(name) => Cow::Borrowed(name),
      None => {
        let error = Error::new(struct_name.span(), "Struct name must end with 'Kind' or you must the name of the error struct as an argument to the attribute (ex. #[boxed_error(MyError)]");
        return TokenStream::from(error.to_compile_error());
      }
    },
  };

  let error_name = Ident::new(&new_error_struct_name, struct_name.span());

  // generate the code for the wrapper struct and implementations
  let expanded = quote! {
    // original struct definition
    #input

    // implement a method to box the kind into the error wrapper
    impl #struct_name {
      pub fn into_box(self) -> #error_name {
        #error_name(Box::new(self))
      }
    }

    // define the boxed error wrapper struct
    #[derive(Debug)]
    pub struct #error_name(pub Box<#struct_name>);

    impl std::fmt::Display for #error_name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
      }
    }

    impl std::error::Error for #error_name where #struct_name: std::error::Error {
      fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
          Some(self.0.as_ref())
      }
    }

    impl #error_name {
      pub fn as_kind(&self) -> &#struct_name {
        &self.0
      }

      pub fn into_kind(self) -> #struct_name {
        *self.0
      }
    }

    // implement conversion from other errors into the boxed error
    impl<E> From<E> for #error_name
    where
      #struct_name: From<E>,
    {
      fn from(err: E) -> Self {
        #error_name(Box::new(#struct_name::from(err)))
      }
    }
  };

  TokenStream::from(expanded)
}
