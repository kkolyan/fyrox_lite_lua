
use fyrox_lite_parser::extract_pod_struct::extract_pod_struct;
use quote::quote;
use syn::{parse_quote, ItemStruct};

fn main() {
    let struct_: ItemStruct = parse_quote! {
        	#[fyrox_lite_pod(Intersection)]
            /// A ray intersection result.
            #[derive(PartialEq)]
            pub struct LiteIntersection {
                /// A handle of the collider with which intersection was detected.
                pub collider: LiteNode,

                /// A normal at the intersection position.
                pub normal: Vector3<f32>,

                /// A position of the intersection in world coordinates.
                pub position: Point3<f32>,

                /// Additional data that contains a kind of the feature with which
                /// intersection was detected as well as its index.
                ///
                /// # Important notes.
                ///
                /// FeatureId::Face might have index that is greater than amount of triangles in
                /// a triangle mesh, this means that intersection was detected from "back" side of
                /// a face. To "fix" that index, simply subtract amount of triangles of a triangle
                /// mesh from the value.
                pub feature: FeatureId,

                /// Distance from the ray origin.
                pub toi: f32,
            }
    };

    let mut errors = Vec::new();
    extract_pod_struct(quote! {"Intersection"}, &struct_, &mut errors).unwrap();
    let error = errors.into_iter().reduce(|mut a, b| {
        a.combine(b);
        a
    });
    if let Some(error) = error {
        let var_name: syn::Result<()> = Err(error);
        var_name.unwrap();
    }
}
