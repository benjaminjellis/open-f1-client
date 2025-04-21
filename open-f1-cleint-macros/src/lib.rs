use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn client_request(_: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(annotated_item as ItemStruct);

    let request_name = &input.ident;

    let name_without_suffix = input.ident.to_string();
    let name_without_suffix = name_without_suffix
        .strip_suffix("Request")
        .expect("Failed to strup suffix 'Request', the struct isn't named as expected");

    let response_name = syn::Ident::new(
        &format!("{name_without_suffix}Response"),
        input.ident.span(),
    );

    let field_methods = input
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.ident.clone().expect("missing field name");

            let field_type =
                option_type(&field.ty).expect("Expected only option fields in the request");

            quote! {
                pub fn #field_name (mut self, #field_name :  #field_type ) -> Self {
                    self.request.#field_name = Some( #field_name );
                    self
                }
            }
        })
        .collect::<Vec<_>>();

    let all_methods = quote! {
        #( #field_methods )*
    };

    let request_impl = quote! {
        impl crate::RequestBuilder<'_, #request_name > {
            #all_methods

            pub async fn send(self) -> Result<Vec< #response_name >, crate::OpenF1ClientError> {
                let response = self
                    .client
                    .get(URL)
                    .query(&self.request)
                    .send()
                    .await
                    .map_err(|source| OpenF1ClientError::Request {
                        endpoint: URL,
                        source,
                    })?;

                match response.status() {
                    StatusCode::OK => response
                        .json::<Vec< #response_name >>()
                        .await
                        .map_err(|source| OpenF1ClientError::Dezerialization {
                            endpoint: URL,
                            source,
                        }),
                    status_code => Err(OpenF1ClientError::Non200StatusCode {
                        status_code,
                        body: response.text().await.unwrap_or_default(),
                        endpoint: URL,
                    }),
                }
            }

        }
    };

    let both = quote! {
        #input
        #request_impl
    };

    both.into()
}

fn option_type(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(ty) = ty else { return None };
    if ty.qself.is_some() {
        return None;
    }

    let ty = &ty.path;

    if ty.segments.is_empty() || ty.segments.last().unwrap().ident != "Option" {
        return None;
    }

    if !(ty.segments.len() == 1
        || (ty.segments.len() == 3
            && ["core", "std"].contains(&ty.segments[0].ident.to_string().as_str())
            && ty.segments[1].ident == "option"))
    {
        return None;
    }

    let last_segment = ty.segments.last().unwrap();
    let syn::PathArguments::AngleBracketed(generics) = &last_segment.arguments else {
        return None;
    };
    if generics.args.len() != 1 {
        return None;
    }
    let syn::GenericArgument::Type(inner_type) = &generics.args[0] else {
        return None;
    };

    Some(inner_type)
}
