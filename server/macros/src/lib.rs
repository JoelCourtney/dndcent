//! This sub-crate contains macros that make content management easier.
//!
//! The intent is to make adding a new Modifier (see top-level crate) as easy as making a new struct
//! in a new file. Without macros or other means of generating source, that would require the
//! dev to also add module imports and map entries to register the new struct with the rest of dndcent,
//! and that sounds terrible and error-prone. These macros automate that process.

#[deny(missing_docs)]

extern crate proc_macro;
extern crate inflector;

mod helpers;

use proc_macro::TokenStream;
use quote::quote;
use quote::format_ident;
use syn;
use syn::Expr;
use helpers::*;
use inflector::Inflector;

/// Generates auto-imports and registry entries for the contents of a directory.
///
/// Works for both dirs that have directory children and dirs that have file children. It is assumed
/// that no directory has BOTH, other than the obligitory mod.rs, which is ignored. It should still
/// work if a dir does have both, but I'm not testing for it.
///
/// # Input
///
/// Input is either a 2-tuple of strings, containing a path and a name, or a single string path. Such as:
///
/// ```
/// macros::registry!("/official/players_handbook/races");
/// ```
///
/// Or:
///
/// ```
/// macros::registry!(("/official", "Player's Handbook"));
/// ```
///
/// Use the tuple version only for Book directories that require pretty printing. See
/// `/content/mod.rs` for file structure and naming convention. I'll repeat some here:
///
/// When using the tuple input, the name arg is the name of the dir *that contains the mod.rs file
/// you are registering*. Make this string exactly equal to the name of the book, including capitalization,
/// spaces, and punctuation. Don't include that directory in the path arg. Make sure the name of the dir
/// matches the name of the book, in lower snake case with no punctuation.
///
/// When using the single-string path input, make it the path of the current dir.
///
/// Always start one level down from `content`. I.E., the path will *always* start with one of:
///
/// - "/official"
/// - "/playtest"
/// - "/homebrew"
#[proc_macro]
pub fn registry(input: TokenStream) -> TokenStream {
    let ast: syn::Expr = syn::parse(input).unwrap();
    let gen = match ast {
        Expr::Tuple(t) => {
            match unwrap_string_tuple(t) {
                Ok((dir_str, name)) => {
                    let dir_str = format!("./src/content{}/{}", dir_str.as_str(), convert_to_fs(&*name));
                    list_imports(dir_str)
                }
                Err(e) => e
            }
        }
        Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(lit),..}) => {
            let dir_str = format!("./src/content{}",lit.value());
            list_imports(dir_str)
        }
        _ => unimplemented!()
    };
    gen
}

/// Registers a race struct and pastes some boilerplate code.
///
/// Calling this macro at the top of the file is required for all content races. The only argument is
/// the string name of the race, is you want it displayed to the user. (I.E. "Variant Human", not "VariantHuman"
/// or "variant_human".
///
/// It makes the race implement the Race trait, which does nothing except require that the dev implements
/// Featured, Modify, Debug, Deserialize, and Serialize. Note that you must also implement Default,
/// but it is required through other means, and the error message may be a little esoteric if you don't.
///
/// For almost all races, Debug, Deserialize, Serialize, and Default can be implemented with the
/// derive macro.
///
/// This also pastes in some use declarations for character, modify, feature, Deserialze, and Serialize.
#[proc_macro]
pub fn race(input: TokenStream) -> TokenStream {
    let ast: syn::LitStr = syn::parse(input).unwrap();
    let pretty_name = ast.value();
    let snake_name = convert_to_fs(&pretty_name);
    let pascal_name = snake_name.to_pascal_case();
    let name_ident = format_ident!("{}", pascal_name);
    (quote! {
        use crate::character::*;
        use crate::modify::*;
        use crate::feature::*;
        use serde::{Serialize, Deserialize};

        #[typetag::serde]
        impl Race for #name_ident {}
    }).into()
}