use proc_macro::TokenStream;
use syn::export::TokenStream2;
use quote::{quote, format_ident};

pub(crate) fn choose(ast: syn::DeriveInput) -> TokenStream {
    let ident = ast.ident.clone();
    (match ast.data.clone() {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let vars = variants.iter().map( |var| {
                var.ident.to_string()
            }).collect();
            let process = process_choose_attribute(ident.to_string(), vars);
            quote! {
                #[derive(Debug, Serialize, Deserialize, Copy, Clone)]
                #ast
                #process
            }
        }
        _ => quote! {
            compile_error("Choose derive only valid for enums");
        }
    }).into()
}

fn process_choose_attribute(name: String, vars: Vec<String>) -> TokenStream2 {
    let enum_ident = format_ident!("{}", name);
    let choice_ident = format_ident!("{}Choice", name);
    let mut acc = quote! {
        impl Default for #enum_ident {
            fn default() -> Self {
                #enum_ident::Unknown
            }
        }

        impl Choose for #enum_ident {
            fn choose<'a>(loc: &'a mut Self) -> Box<dyn Choice + 'a> {
                Box::new( #choice_ident { locs: vec![ loc ] } )
            }
            fn choose_multiple<'a>(locs: Vec<&'a mut Self>) -> Box<dyn Choice + 'a> {
                Box::new( #choice_ident { locs } )
            }
        }

        #[derive(Debug)]
        pub struct #choice_ident<'a> {
            locs: Vec<&'a mut #enum_ident>
        }
    };
    let mut choices = "".to_string();
    for var in &vars {
        choices.extend(format!(r#""{}","#, var).chars());
    }
    let choices_tokens: TokenStream2 = choices.parse().unwrap();
    let mut match_rules = "".to_string();
    for var in &vars {
        match_rules.extend(format!(r#""{}" => {}::{},"#, var, name, var).chars());
    }
    let match_rules_tokens: TokenStream2 = match_rules.parse().unwrap();
    acc = quote! {
        #acc

        impl Choice for #choice_ident<'_> {
            fn choices(&self) -> Vec<&'static str> {
                vec![ #choices_tokens ]
            }
            fn choose(&mut self, choice: &str, index: usize) {
                **self.locs.get_mut(index).unwrap() = match choice {
                    #match_rules_tokens
                    _ => unimplemented!()
                }
            }
        }
    };
    acc
}

pub(crate) fn dynamic_choose(ast: syn::ItemTrait) -> TokenStream {
    let ident = ast.ident.clone();
    let lower_ident = format_ident!("{}", ident.to_string().to_lowercase());
    let get_all_ident = format_ident!("get_all_{}", lower_ident);
    let choice_ident = format_ident!("{}Choice", ident);
    let unknown_string = format!("Unknown {}", ident.to_string());
    (quote! {
        #[typetag::serde]
        #ast
        impl Choose for Box<dyn #ident> {
            fn choose<'a>(loc: &'a mut Self) -> Box<dyn Choice + 'a> {
                Box::new( #choice_ident { locs: vec![ loc ] } )
            }
            fn choose_multiple<'a>(locs: Vec<&'a mut Self>) -> Box<dyn Choice + 'a> {
                Box::new( #choice_ident { locs } )
            }
        }

        #[derive(Debug)]
        struct #choice_ident<'a> {
            pub locs: Vec<&'a mut Box<dyn #ident>>
        }

        impl Choice for #choice_ident<'_> {
            fn choices(&self) -> Vec<&'static str> {
                content::#get_all_ident()
            }
            fn choose(&mut self, choice: &str, index: usize) {
                **self.locs.get_mut(index).unwrap() = content::#lower_ident(choice).unwrap()
            }
        }

        impl Default for Box<dyn #ident> {
            fn default() -> Self {
                content::#lower_ident(#unknown_string).expect(&format!("{} cant be found", #unknown_string))
            }
        }
    }).into()
}
