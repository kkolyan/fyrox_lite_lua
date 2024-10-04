use syn::parse_quote;

fn main() {
    let _a: syn::Attribute = ::syn::__private::parse_quote({
        let mut _s = ::quote::__private::TokenStream::new();
        ::quote::__private::push_pound(&mut _s);
        ::quote::__private::push_group(&mut _s, ::quote::__private::Delimiter::Bracket, {
            let mut _s = ::quote::__private::TokenStream::new();
            ::quote::__private::push_ident(&mut _s, "xyz");
            ::quote::__private::push_group(&mut _s, ::quote::__private::Delimiter::Parenthesis, {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "A");
                ::quote::__private::push_comma(&mut _s);
                ::quote::__private::push_ident(&mut _s, "B");
                _s
            });
            _s
        });
        _s
    });

    let a: syn::Attribute = parse_quote!(#[xyz(A, B)]);

    println!("{:?}", a);

    /*
    Attribute {
        pound_token: Pound,
        style: AttrStyle::Outer,
        bracket_token: Bracket,
        meta: Meta::List {
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment {
                        ident: Ident(xyz),
                         arguments: PathArguments::None
                    }
                ]
            },
            delimiter: MacroDelimiter::Paren(Paren),
            tokens: TokenStream [
                Ident { sym: A },
                Punct { char: ',', spacing: Alone },
                Ident { sym: B }
            ]
        }
    };
     */
}
