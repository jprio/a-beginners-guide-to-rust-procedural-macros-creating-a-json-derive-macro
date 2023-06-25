use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput, Data };

#[proc_macro_derive(Json)]
pub fn json_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let out = match input.data {
        Data::Struct(s) => {
            let fields = s.fields.into_iter().map(|field| field.ident.unwrap());
            quote! {
                impl json_trait::Json for #name {
                    fn to_json(&self) -> String {
                        let mut json = "{ ".to_string();
                        #(
                            json.push_str(&format!("\"{}\": {}, ", stringify!(#fields), json_trait::Json::to_json(&self.#fields)));
                        )*
                        json.remove(json.len() - 2); // remove trailling comma
                        json.push('}');
                        json
                    }
                }
            }
        }
        _ => todo!(),
    };
    out.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
