use convert_case::{Case, Casing};
use gen_common::code_model::Module;
use gen_common::context::GenerationContext;
use gen_common::templating::render;
use lite_model::{Field, StructClass};
use crate::lite_csgen::api_types;
use crate::lite_csgen::api_types::TypeMarshalling;

pub(crate) fn generate_bindings(class: &StructClass, ctx: &GenerationContext) -> Module {
    let mut s = String::new();
    render(&mut s, r#"
            // ${rust_path}
            [StructLayout(LayoutKind.Sequential)]
            public struct ${class}
            {
    "#, [("class", &class.class_name), ("rust_path", &class.rust_struct_path)]);

    for field in class.fields.iter() {
        generate_property(&mut s, class, field, ctx);
    }
    s += "
            //===============================================================
            // private fields for all properties (not only mapped),
            // because it makes ABI much more readable.
            // I hope, NativeAOT will optimize out this.
            //===============================================================";

    for field in class.fields.iter() {
        generate_field(&mut s, class, field, ctx);
    }

    render(&mut s, r#"
            }
            "#, []);


    render(&mut s, r#"
            
            [StructLayout(LayoutKind.Sequential)]
            internal struct ${class}_optional {
                internal ${class} Value;
                internal bool HasValue;

                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                public static ${class}? ToFacade(in ${class}_optional value) => value.HasValue ? value.Value : null;

                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                public static ${class}_optional FromFacade(in ${class}? value) => new ${class}_optional { Value = value ?? default, HasValue = value.HasValue };
            }
    "#, [("class", &class.class_name)]);

    render(&mut s, r#"
            
            [StructLayout(LayoutKind.Explicit)]
            internal struct ${class}_result {
                [FieldOffset(0)]
                internal ${bool_type} ok;

                [FieldOffset(sizeof(${bool_type}))]
                internal ${class} value;

                [FieldOffset(sizeof(${bool_type}))]
                internal string err;
            }
    "#, [("class", &class.class_name), ("bool_type", &"int")]);

    render(&mut s, r#"
            
            // it iterates over the unmanaged memory (Vec allocated by Rust and stored for the length of a frame in the arena).
            // if user attempts to iterate this iterator after backing data is disposed,
            // the methods throws exception (hash is used to check if the backing data is still alive to make it
            // possible to throw exceptions instead of SIGSEGV-ing)
            [StructLayout(LayoutKind.Sequential)]
            public struct ${class}Iterator : IEnumerator<${class}> {
                // hash is a random number,  allocated in unmanaged memory next to the items with the same lifetime.
                // arena (Vec<(Hash,Vec<${class}>)>) is zeroed at the end of every frame.
                private unsafe int* hash;
                private unsafe ${class}* items;
                private int length;
                private int position;
                private int expectedHash;

                public ${class} Current
                {
                    get
                    {
                        unsafe {
                          if (*hash != expectedHash) {
                             throw new Exception("iterator is not valid anymore (it's valid only for one frame)");
                          }
                          return *(items + position);
                        }
                    }
                }

                public bool MoveNext()
                {
                    if (position < length - 2) {
                        position ++;
                        return true;
                    }
                    return false;
                }

                public void Dispose()
                {
                }

                public void Reset() => position = 0;

                object? IEnumerator.Current => Current;
            }
    "#, [("class", &class.class_name)]);

    Module::code(&class.class_name, s)
}

fn generate_property(s: &mut String, class: &StructClass, field: &Field, ctx: &GenerationContext) {
    let ty = api_types::type_rs2cs(&field.ty);
    let facade_name = field.name.to_case(Case::Pascal);
    let private_name = format!("_{}", field.name);
    match &ty {
        TypeMarshalling::Blittable(ty) => {
            render(s, r#"
                public ${type} ${facade_name} {
                    get => ${private_name};
                    set => ${private_name} = value;
                }
                "#, [
                ("type", &ty),
                ("private_name", &private_name),
                ("facade_name", &facade_name),
            ]);
        }
        TypeMarshalling::Mapped { facade, blittable } => {
            render(s, r#"
                public ${facade_ty} ${facade_name} {
                    get => ${blittable_ty}.ToFacade(${private_name});
                    set => ${private_name} = ${blittable_ty}.FromFacade(value);
                }
                "#, [
                ("blittable_ty", &blittable),
                ("facade_ty", &facade),
                ("private_name", &private_name),
                ("facade_name", &facade_name),
            ]);
        }
    }
}
fn generate_field(s: &mut String, class: &StructClass, field: &Field, ctx: &GenerationContext) {
    let ty = api_types::type_rs2cs(&field.ty);
    let facade_name = field.name.to_case(Case::Pascal);
    let private_name = format!("_{}", field.name);
    render(s, r#"
                private ${type} ${private_name};
                "#, [
        ("type", &ty.to_blittable()),
        ("private_name", &private_name),
    ]);
}