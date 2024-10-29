use quote::quote;
use syn::parse2;
use lite_parser::extract_engine_class::extract_fn;

fn main() {
    let q = quote! {
        pub fn cast_ray<T: UserScript>(opts: LiteRayCastOptions, mut results: T::Buffer<'_, LiteIntersection>) {}
    };

    let mut func = parse2(q).unwrap();

    let mut errors = Vec::new();
    let method = extract_fn(&mut func, &mut errors);
    println!("{:#?}", errors);
    println!("{:#?}", method);
}