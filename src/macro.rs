// no-coverage:start
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident,
    Error,
    Result,
    Path,
    parse_macro_input,
    Token,
    ItemFn,
};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::{collections::HashSet, hash::Hash};

// features="tests/features", spawner=MySpawner, wraps-tokio, use-tokio

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AttributeOption {
    Features(String),
    Spawner(Path),
    WrapsTokio,
    UseTokio,
}

#[derive(Debug, Clone)]
struct AttributeOptionItem {
    option: AttributeOption,
    span: Span,
}

impl PartialEq for AttributeOptionItem {
    fn eq(&self, other: &Self) -> bool {
        self.option == other.option
    }
}

impl Eq for AttributeOptionItem {}

impl Hash for AttributeOptionItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.option.hash(state);
    }
}

impl Parse for AttributeOptionItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let option_name = input.parse::<Ident>()?;
        let span = option_name.span();
        let option_name = option_name.to_string();

        match option_name.as_str() {
            "features" => {
                input.parse::<Token![=]>()?;
                let features = input.parse::<syn::LitStr>()?;

                Ok(AttributeOptionItem {
                    option: AttributeOption::Features(features.value()),
                    span,
                })
            }

            "spawner" => {
                input.parse::<Token![=]>()?;
                let spawner = input.parse::<Path>()?;

                Ok(AttributeOptionItem {
                    option: AttributeOption::Spawner(spawner),
                    span,
                })
            }

            "wraps-tokio" => Ok(AttributeOptionItem {
                option: AttributeOption::WrapsTokio,
                span,
            }),

            "use-tokio" => Ok(AttributeOptionItem {
                option: AttributeOption::UseTokio,
                span,
            }),

            _ => Err(Error::new(span, "Unknown option")),
        }
    }
}

struct AttributeOptions {
    features: Option<String>,
    spawner: Option<Path>,
    wraps_tokio: bool,
    use_tokio: bool,
}

impl Parse for AttributeOptions {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut features = None::<String>;
        let mut spawner = None::<Path>;
        let mut wraps_tokio = None::<bool>;
        let mut use_tokio = None::<bool>;

        {
            let mut options = HashSet::new();

            for option in Punctuated::<AttributeOptionItem, Token![,]>::parse_terminated(input)? {
                if options.insert(option.option.clone()) {
                    return Err(Error::new(option.span.clone(), format!("Duplicate option: {:?}", option.option)));
                };

                match option.option {
                    AttributeOption::Features(value) => { features.replace(value); }
                    AttributeOption::Spawner(value) => { spawner.replace(value); }
                    AttributeOption::WrapsTokio => { wraps_tokio.replace(true); }
                    AttributeOption::UseTokio => { use_tokio.replace(true); }
                }
            }

            if options.contains(&AttributeOption::UseTokio) && options.contains(&AttributeOption::WrapsTokio) {
                return Err(Error::new(Span::call_site(), "Cannot use both `use-tokio` and `wraps-tokio`"));
            }
        }

        if use_tokio.unwrap() && spawner.is_some() {
            return Err(Error::new(Span::call_site(), "Cannot use both `use-tokio` and `spawner`"));
        }

        Ok(AttributeOptions {
            features,
            spawner,
            wraps_tokio: wraps_tokio.unwrap_or(false),
            use_tokio: use_tokio.unwrap_or(false),
        })
    }
}

/// This is a attribute procedural macro that will be used to define the cucumber tests
#[proc_macro_attribute]
pub fn cucumber_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let options = parse_macro_input!(attr as AttributeOptions);
    let function = parse_macro_input!(item as ItemFn);

    let name = function.sig.ident.clone();

    let spawner = if let Some(spawner) = options.spawner {
        quote! {
            trellis = trellis.with_spawner::<#spawner>()
        }
    } else if options.use_tokio {
        quote!{
            if cfg!(feature = "tokio") {
                trellis = trellis.with_spawner::<cucumber_trellis::spawners::TokioSpawner>();
            }
        }
    } else {
        quote!{}
    };

    let features = if let Some(features) = options.features {
        quote! {
            let feature_path = PathBuf::from(#features);
            let features = Some(feature_path.as_path());
        }
    } else {
        quote! {
            let features = None::<&Path>;
        }
    };

    let main = quote! {
        fn main() {
            let mut trellis = {
                #features
                let trellis = cucumber_trellis::CucumberTrellis::new(features);

                #spawner
                trellis
            };

            #name(&mut trellis);

            trellis.run_tests();
        }
    };

    let implementation = if options.use_tokio {
        quote! {
            #[cfg(feature = "tokio")]
            #[tokio::main]
            async #main

            #[cfg(not(feature = "tokio"))]
            #main
        }
    } else {
        quote! {
            #main
        }
    };

    let output = quote! {
        #function

        #implementation
    };

    output.into()
}
// no-coverage:stop
