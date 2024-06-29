// no-coverage:start
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Path,
    Ident,
    Error,
    Result,
    ItemFn,
    LitStr,
    Token,
    parse_macro_input,
};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::{
    hash::{Hasher, Hash},
    collections::HashSet,
};

#[derive(Clone)]
enum AttributeOption {
    Features(LitStr),
    Executor(Path),
    UseTokio,
}

impl AttributeOption {
    fn name(&self) -> String {
        match self {
            Self::Features(_) => String::from("features"),
            Self::Executor(_) => String::from("executor"),
            Self::UseTokio => String::from("use_tokio"),
        }
    }
}

impl Eq for AttributeOption {}

impl PartialEq for AttributeOption {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Features(_), Self::Features(_)) => true,
            (Self::Executor(_), Self::Executor(_)) => true,
            (Self::UseTokio, Self::UseTokio) => true,
            _ => false,
        }
    }
}

impl Hash for AttributeOption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Features(_) => 0.hash(state),
            Self::Executor(_) => 1.hash(state),
            Self::UseTokio => 2.hash(state),
        }
    }
}

#[derive(Clone)]
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

                Ok(AttributeOptionItem {
                    option: AttributeOption::Features(input.parse::<syn::LitStr>()?),
                    span,
                })
            }

            "executor" => {
                input.parse::<Token![=]>()?;
                let executor = input.parse::<Path>()?;

                Ok(AttributeOptionItem {
                    option: AttributeOption::Executor(executor),
                    span,
                })
            }

            "use_tokio" => Ok(AttributeOptionItem {
                option: AttributeOption::UseTokio,
                span,
            }),

            _ => Err(Error::new(span, "Unknown option")),
        }
    }
}

struct AttributeOptions {
    features: Option<LitStr>,
    executor: Option<Path>,
    use_tokio: bool,
}

impl Parse for AttributeOptions {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut features = None::<LitStr>;
        let mut executor = None::<Path>;
        let mut use_tokio = None::<bool>;

        {
            let mut options = HashSet::new();

            for option in Punctuated::<AttributeOptionItem, Token![,]>::parse_terminated(input)? {
                if !options.insert(option.option.clone()) {
                    return Err(Error::new(
                        option.span.clone(),
                        format!("Duplicate option: {}", option.option.name()),
                    ));
                };

                match option.option {
                    AttributeOption::Features(value) => {
                        features.replace(value);
                    }
                    AttributeOption::Executor(value) => {
                        executor.replace(value);
                    }
                    AttributeOption::UseTokio => {
                        use_tokio.replace(true);
                    }
                }
            }
        }

        let use_tokio = use_tokio.unwrap_or(false);
        if use_tokio && executor.is_some() {
            return Err(Error::new(
                Span::call_site(),
                "Cannot use both `use_tokio` and `executor`",
            ));
        }

        Ok(AttributeOptions {
            features,
            executor,
            use_tokio,
        })
    }
}

/// This is an attribute procedural macro that will be used to define the cucumber tests
#[proc_macro_attribute]
pub fn cucumber_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let options = parse_macro_input!(attr as AttributeOptions);
    let function = parse_macro_input!(item as ItemFn);

    let name = function.sig.ident.clone();

    let features = if let Some(features) = options.features {
        quote! {
            let feature_path = std::path::PathBuf::from(#features);
            let features = Some(feature_path.as_path());
        }
    } else {
        quote! {
            let features = None::<&std::path::Path>;
        }
    };

    let (fn_main, run_tests) = if options.use_tokio {
        (
            quote! {
                #[tokio::main]
                async fn main()
            },
            quote! {
                trellis.run_tests().await
            },
        )
    } else {
        let executor = match options.executor {
            Some(executor) => quote! {
                #executor
            },
            None => quote! {
                futures::executor::block_on
            },
        };

        (
            quote! {
                fn main()
            },
            quote! {
                #executor(trellis.run_tests())
            },
        )
    };

    let output = quote! {
        #function

        #fn_main {
            let mut trellis = {
                #features
                cucumber_trellis::CucumberTrellis::new(features)
            };

            #name(&mut trellis);

            #run_tests;
        }
    };

    output.into()
}
// no-coverage:stop
