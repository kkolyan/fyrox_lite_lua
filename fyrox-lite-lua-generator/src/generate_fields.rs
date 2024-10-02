// use fyrox_lite_model::StructClass;

// use crate::context::GenerationContext;


// pub fn generate_fields(
//     s: &mut String,
//     class: &StructClass,
//     ctx: &GenerationContext,
//     instance: bool,
// ) {
// 	if !instance {
// 		return;
// 	}
//     for field in class.fields.iter(){

//         render(
//             s,
//             r#"
// 				methods.add_method_mut(
// 					"${method_name}",
// 					|lua, this, (${input_names}): (${param_types})| {
// 			"#,
//             [
//                 ("method_name", &method.method_name),
//                 ("input_names", &input_names),
//                 ("param_types", &param_types),
//             ],
//         );

//         for param in method.signature.params.iter() {
//             let expression = mlua_to_rust_expr(param, ctx);
//             render(
//                 s,
//                 r#"
// 						let ${param_name} = ${expression};
// 				"#,
//                 [("param_name", &param.name), ("expression", &expression)],
//             );
//         }

//         let target = match instance {
//             true => Cow::Borrowed("this."),
//             false => Cow::Owned(format!("{}::", class.rust_struct_path.0)),
//         };

//         let ret_expression = rust_expr_to_mlua(
//             ctx,
//             "ret",
//             method
//                 .signature
//                 .return_ty
//                 .as_ref()
//                 .unwrap_or(&DataType::Unit),
//         );

//         render(
//             s,
//             r#"
// 						let ret = ${target}${method_name}${generics}(${output_names});
//                         let ret = ${ret_expression};
// 						Ok(ret)
// 					},
// 				);
// 			"#,
//             [
//                 ("target", &target),
//                 ("method_name", &method.method_name),
//                 ("output_names", &output_names),
//                 ("generics", &generics),
//                 ("ret_expression", &ret_expression),
//             ],
//         );
//     }
// }