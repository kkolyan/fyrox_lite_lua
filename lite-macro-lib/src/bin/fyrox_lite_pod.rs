use lite_macro_lib::fyrox_lite::lite_api;
use quote::quote;

fn main() {
	let attr = quote! {
		#[lite_api(Intersection)]
	};
	let item = quote! {
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
	let result = lite_api(attr, item);
	println!("{}", result);
}