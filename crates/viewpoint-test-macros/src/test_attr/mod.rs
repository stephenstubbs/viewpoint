//! Implementation of the `#[viewpoint::test]` attribute macro.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, FnArg, Ident, ItemFn, LitBool, LitInt, LitStr, Pat, Result, Token, Type,
};

/// Parsed arguments from the `#[test(...)]` attribute.
#[derive(Debug, Default)]
pub struct TestArgs {
    pub headless: Option<bool>,
    pub timeout: Option<u64>,
    pub scope: Option<String>,
    pub browser: Option<String>,
    pub context: Option<String>,
}

impl Parse for TestArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut args = TestArgs::default();

        if input.is_empty() {
            return Ok(args);
        }

        let pairs = Punctuated::<KeyValue, Token![,]>::parse_terminated(input)?;

        for kv in pairs {
            match kv.key.to_string().as_str() {
                "headless" => {
                    args.headless = Some(kv.value_bool()?);
                }
                "timeout" => {
                    args.timeout = Some(kv.value_int()?);
                }
                "scope" => {
                    args.scope = Some(kv.value_string()?);
                }
                "browser" => {
                    args.browser = Some(kv.value_string()?);
                }
                "context" => {
                    args.context = Some(kv.value_string()?);
                }
                other => {
                    return Err(Error::new(kv.key.span(), format!("unknown attribute: {other}")));
                }
            }
        }

        Ok(args)
    }
}

/// A key-value pair in the attribute arguments.
struct KeyValue {
    key: Ident,
    value: KeyValueValue,
}

enum KeyValueValue {
    Bool(LitBool),
    Int(LitInt),
    Str(LitStr),
}

impl KeyValue {
    fn value_bool(&self) -> Result<bool> {
        match &self.value {
            KeyValueValue::Bool(lit) => Ok(lit.value()),
            _ => Err(Error::new(self.key.span(), "expected boolean value")),
        }
    }

    fn value_int(&self) -> Result<u64> {
        match &self.value {
            KeyValueValue::Int(lit) => lit.base10_parse(),
            _ => Err(Error::new(self.key.span(), "expected integer value")),
        }
    }

    fn value_string(&self) -> Result<String> {
        match &self.value {
            KeyValueValue::Str(lit) => Ok(lit.value()),
            _ => Err(Error::new(self.key.span(), "expected string value")),
        }
    }
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;

        let lookahead = input.lookahead1();
        let value = if lookahead.peek(LitBool) {
            KeyValueValue::Bool(input.parse()?)
        } else if lookahead.peek(LitInt) {
            KeyValueValue::Int(input.parse()?)
        } else if lookahead.peek(LitStr) {
            KeyValueValue::Str(input.parse()?)
        } else {
            return Err(lookahead.error());
        };

        Ok(KeyValue { key, value })
    }
}

/// Detected fixture parameters from function signature.
#[derive(Debug, Default)]
struct Fixtures {
    has_page: bool,
    has_context: bool,
    has_browser: bool,
    page_name: Option<Ident>,
    context_name: Option<Ident>,
    browser_name: Option<Ident>,
}

/// Expand the test macro.
#[allow(clippy::needless_pass_by_value)]
pub fn expand_test(args: TestArgs, input: ItemFn) -> Result<TokenStream> {
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_attrs = &input.attrs;
    let fn_vis = &input.vis;

    // Parse fixture parameters
    let fixtures = parse_fixtures(&input)?;

    // Validate scope arguments
    validate_scope_args(&args)?;

    // Generate harness setup code
    let harness_setup = generate_harness_setup(&args)?;

    // Generate fixture extraction code
    let fixture_extraction = generate_fixture_extraction(&fixtures);

    // Generate the expanded function
    let expanded = quote! {
        #(#fn_attrs)*
        #[::tokio::test]
        #fn_vis async fn #fn_name() -> ::std::result::Result<(), ::std::boxed::Box<dyn ::std::error::Error>> {
            let _harness = #harness_setup;

            #fixture_extraction

            // User's test body
            let __result: ::std::result::Result<(), ::std::boxed::Box<dyn ::std::error::Error>> = async {
                #fn_block
                Ok(())
            }.await;

            __result
        }
    };

    Ok(expanded)
}

fn parse_fixtures(input: &ItemFn) -> Result<Fixtures> {
    let mut fixtures = Fixtures::default();

    for arg in &input.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            let param_name = match pat_type.pat.as_ref() {
                Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                _ => continue,
            };

            let type_name = extract_type_name(&pat_type.ty)?;

            match type_name.as_str() {
                "Page" => {
                    fixtures.has_page = true;
                    fixtures.page_name = Some(param_name);
                }
                "BrowserContext" => {
                    fixtures.has_context = true;
                    fixtures.context_name = Some(param_name);
                }
                "Browser" => {
                    fixtures.has_browser = true;
                    fixtures.browser_name = Some(param_name);
                }
                _ => {
                    return Err(Error::new_spanned(
                        &pat_type.ty,
                        format!("unsupported fixture type: {type_name}. Expected Page, BrowserContext, or Browser"),
                    ));
                }
            }
        }
    }

    Ok(fixtures)
}

fn extract_type_name(ty: &Type) -> Result<String> {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                Ok(segment.ident.to_string())
            } else {
                Err(Error::new_spanned(ty, "could not extract type name"))
            }
        }
        Type::Reference(type_ref) => extract_type_name(&type_ref.elem),
        _ => Ok(ty.to_token_stream().to_string()),
    }
}

fn validate_scope_args(args: &TestArgs) -> Result<()> {
    match args.scope.as_deref() {
        Some("browser") => {
            if args.browser.is_none() {
                return Err(Error::new(
                    proc_macro2::Span::call_site(),
                    "scope = \"browser\" requires browser = \"<function_name>\" to specify the shared browser source",
                ));
            }
        }
        Some("context") => {
            if args.context.is_none() {
                return Err(Error::new(
                    proc_macro2::Span::call_site(),
                    "scope = \"context\" requires context = \"<function_name>\" to specify the shared context source",
                ));
            }
        }
        Some(other) => {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                format!("unknown scope: \"{other}\". Expected \"browser\" or \"context\""),
            ));
        }
        None => {}
    }

    Ok(())
}

fn generate_harness_setup(args: &TestArgs) -> Result<TokenStream> {
    let headless = args.headless.unwrap_or(true);
    let timeout_ms = args.timeout.unwrap_or(30000);

    match args.scope.as_deref() {
        Some("browser") => {
            let browser_fn: Ident = syn::parse_str(args.browser.as_ref().unwrap())?;
            Ok(quote! {
                ::viewpoint_test::TestHarness::from_browser(#browser_fn().await).await?
            })
        }
        Some("context") => {
            let context_fn: Ident = syn::parse_str(args.context.as_ref().unwrap())?;
            Ok(quote! {
                ::viewpoint_test::TestHarness::from_context(#context_fn().await).await?
            })
        }
        None => {
            Ok(quote! {
                ::viewpoint_test::TestHarness::builder()
                    .headless(#headless)
                    .timeout(::std::time::Duration::from_millis(#timeout_ms))
                    .build()
                    .await?
            })
        }
        _ => unreachable!("scope validated earlier"),
    }
}

fn generate_fixture_extraction(fixtures: &Fixtures) -> TokenStream {
    let mut tokens = TokenStream::new();

    if let Some(ref name) = fixtures.page_name {
        tokens.extend(quote! {
            let #name = _harness.page();
        });
    }

    if let Some(ref name) = fixtures.context_name {
        tokens.extend(quote! {
            let #name = _harness.context().expect("context not available");
        });
    }

    if let Some(ref name) = fixtures.browser_name {
        tokens.extend(quote! {
            let #name = _harness.browser().expect("browser not available");
        });
    }

    tokens
}
