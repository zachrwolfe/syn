use crate::punctuated::Punctuated;
use super::*;

ast_struct! {
    pub struct PartialBorrow {
        pub mutability: Option<Token![mut]>,
        pub ident: Ident,
    }
}

ast_struct! {
    pub struct PartialBorrows {
        pub brace_token: token::Brace,
        pub borrows: Punctuated<PartialBorrow, Token![,]>,
    }
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;
    use crate::parse::{Parse, ParseStream, Result};

    impl Parse for PartialBorrow {
        fn parse(input: ParseStream) -> Result<Self> {
            let lookahead = input.lookahead1();
            let mutability = if lookahead.peek(Token![mut]) {
                Some(input.parse()?)
            } else {
                None
            };
            Ok(PartialBorrow {
                mutability,
                ident: input.parse()?,
            })
        }
    }

    impl Parse for PartialBorrows {
        fn parse(input: ParseStream) -> Result<Self> {
            let content;
            Ok(PartialBorrows {
                brace_token: braced!(content in input),
                borrows: content.parse_terminated(PartialBorrow::parse)?,
            })
        }
    }
}

#[cfg(feature = "printing")]
pub mod printing {
    use super::*;

    use proc_macro2::TokenStream;
    use quote::ToTokens;

    impl ToTokens for PartialBorrow {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.mutability.to_tokens(tokens);
            self.ident.to_tokens(tokens);
        }
    }

    impl ToTokens for PartialBorrows {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.brace_token.surround(tokens, |tokens| {
                self.borrows.to_tokens(tokens);
            });
        }
    }
}