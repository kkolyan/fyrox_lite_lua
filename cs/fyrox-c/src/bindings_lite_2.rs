
            #[repr(C)]
            pub struct NativeInput_optional {
                pub value: fyrox_lite::lite_input::Input,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_input::Input>> for NativeInput_optional {
                fn from(value: Option<fyrox_lite::lite_input::Input>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeInput_optional> for Option<fyrox_lite::lite_input::Input> {
                fn from(value: NativeInput_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeUiNode_optional {
                pub value: fyrox_lite::lite_ui::LiteUiNode,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_ui::LiteUiNode>> for NativeUiNode_optional {
                fn from(value: Option<fyrox_lite::lite_ui::LiteUiNode>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeUiNode_optional> for Option<fyrox_lite::lite_ui::LiteUiNode> {
                fn from(value: NativeUiNode_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeText_optional {
                pub value: fyrox_lite::lite_ui::LiteText,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_ui::LiteText>> for NativeText_optional {
                fn from(value: Option<fyrox_lite::lite_ui::LiteText>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeText_optional> for Option<fyrox_lite::lite_ui::LiteText> {
                fn from(value: NativeText_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeTextBuilder {
            pub _foreground: NativeBrush_optional,
            pub _font_size: f32_optional,
            }

            #[repr(C)]
            pub struct NativeTextBuilder_optional {
                pub value: fyrox_lite::lite_ui::TextBuilder,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_ui::TextBuilder>> for NativeTextBuilder_optional {
                fn from(value: Option<fyrox_lite::lite_ui::TextBuilder>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeTextBuilder_optional> for Option<fyrox_lite::lite_ui::TextBuilder> {
                fn from(value: NativeTextBuilder_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeBrush {
            pub _solid_color: NativeColor_optional,
            pub _linear_gradient: NativeLinearGradient_optional,
            pub _radial_gradient: NativeRadialGradient_optional,
            }

            #[repr(C)]
            pub struct NativeBrush_optional {
                pub value: fyrox_lite::lite_ui::Brush,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_ui::Brush>> for NativeBrush_optional {
                fn from(value: Option<fyrox_lite::lite_ui::Brush>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeBrush_optional> for Option<fyrox_lite::lite_ui::Brush> {
                fn from(value: NativeBrush_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeLinearGradient {
            pub _from: NativeVector2,
            pub _to: NativeVector2,
            pub _stops: NativeGradientPoint_slice,
            }

            #[repr(C)]
            pub struct NativeLinearGradient_optional {
                pub value: fyrox_lite::lite_ui::LinearGradient,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_ui::LinearGradient>> for NativeLinearGradient_optional {
                fn from(value: Option<fyrox_lite::lite_ui::LinearGradient>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeLinearGradient_optional> for Option<fyrox_lite::lite_ui::LinearGradient> {
                fn from(value: NativeLinearGradient_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeRadialGradient {
            pub _center: NativeVector2,
            pub _stops: NativeGradientPoint_slice,
            }

            #[repr(C)]
            pub struct NativeRadialGradient_optional {
                pub value: fyrox_lite::lite_ui::RadialGradient,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_ui::RadialGradient>> for NativeRadialGradient_optional {
                fn from(value: Option<fyrox_lite::lite_ui::RadialGradient>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeRadialGradient_optional> for Option<fyrox_lite::lite_ui::RadialGradient> {
                fn from(value: NativeRadialGradient_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeColor_optional {
                pub value: fyrox_lite::lite_ui::Color,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_ui::Color>> for NativeColor_optional {
                fn from(value: Option<fyrox_lite::lite_ui::Color>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeColor_optional> for Option<fyrox_lite::lite_ui::Color> {
                fn from(value: NativeColor_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeGradientPoint {
            pub _stop: f32,
            pub _color: NativeColor,
            }

            #[repr(C)]
            pub struct NativeGradientPoint_optional {
                pub value: fyrox_lite::lite_ui::GradientPoint,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_ui::GradientPoint>> for NativeGradientPoint_optional {
                fn from(value: Option<fyrox_lite::lite_ui::GradientPoint>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeGradientPoint_optional> for Option<fyrox_lite::lite_ui::GradientPoint> {
                fn from(value: NativeGradientPoint_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeLog_optional {
                pub value: fyrox_lite::lite_log::LiteLog,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_log::LiteLog>> for NativeLog_optional {
                fn from(value: Option<fyrox_lite::lite_log::LiteLog>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeLog_optional> for Option<fyrox_lite::lite_log::LiteLog> {
                fn from(value: NativeLog_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeWindow_optional {
                pub value: fyrox_lite::lite_window::LiteWindow,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_window::LiteWindow>> for NativeWindow_optional {
                fn from(value: Option<fyrox_lite::lite_window::LiteWindow>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeWindow_optional> for Option<fyrox_lite::lite_window::LiteWindow> {
                fn from(value: NativeWindow_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeNode_optional {
                pub value: fyrox_lite::lite_node::LiteNode,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_node::LiteNode>> for NativeNode_optional {
                fn from(value: Option<fyrox_lite::lite_node::LiteNode>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeNode_optional> for Option<fyrox_lite::lite_node::LiteNode> {
                fn from(value: NativeNode_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativePlugin_optional {
                pub value: fyrox_lite::lite_plugin::LitePlugin,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_plugin::LitePlugin>> for NativePlugin_optional {
                fn from(value: Option<fyrox_lite::lite_plugin::LitePlugin>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativePlugin_optional> for Option<fyrox_lite::lite_plugin::LitePlugin> {
                fn from(value: NativePlugin_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeVector3 {
            pub _x: f32,
            pub _y: f32,
            pub _z: f32,
            }

            #[repr(C)]
            pub struct NativeVector3_optional {
                pub value: fyrox_lite::lite_math::PodVector3,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_math::PodVector3>> for NativeVector3_optional {
                fn from(value: Option<fyrox_lite::lite_math::PodVector3>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeVector3_optional> for Option<fyrox_lite::lite_math::PodVector3> {
                fn from(value: NativeVector3_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeVector2 {
            pub _x: f32,
            pub _y: f32,
            }

            #[repr(C)]
            pub struct NativeVector2_optional {
                pub value: fyrox_lite::lite_math::PodVector2,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_math::PodVector2>> for NativeVector2_optional {
                fn from(value: Option<fyrox_lite::lite_math::PodVector2>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeVector2_optional> for Option<fyrox_lite::lite_math::PodVector2> {
                fn from(value: NativeVector2_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeVector2i {
            pub _x: i64,
            pub _y: i64,
            }

            #[repr(C)]
            pub struct NativeVector2i_optional {
                pub value: fyrox_lite::lite_math::PodVector2i,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_math::PodVector2i>> for NativeVector2i_optional {
                fn from(value: Option<fyrox_lite::lite_math::PodVector2i>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeVector2i_optional> for Option<fyrox_lite::lite_math::PodVector2i> {
                fn from(value: NativeVector2i_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeQuaternion {
            pub _i: f32,
            pub _j: f32,
            pub _k: f32,
            pub _w: f32,
            }

            #[repr(C)]
            pub struct NativeQuaternion_optional {
                pub value: fyrox_lite::lite_math::PodQuaternion,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_math::PodQuaternion>> for NativeQuaternion_optional {
                fn from(value: Option<fyrox_lite::lite_math::PodQuaternion>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeQuaternion_optional> for Option<fyrox_lite::lite_math::PodQuaternion> {
                fn from(value: NativeQuaternion_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativePrefab_optional {
                pub value: fyrox_lite::lite_prefab::LitePrefab,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_prefab::LitePrefab>> for NativePrefab_optional {
                fn from(value: Option<fyrox_lite::lite_prefab::LitePrefab>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativePrefab_optional> for Option<fyrox_lite::lite_prefab::LitePrefab> {
                fn from(value: NativePrefab_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeScene_optional {
                pub value: fyrox_lite::lite_scene::LiteScene,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_scene::LiteScene>> for NativeScene_optional {
                fn from(value: Option<fyrox_lite::lite_scene::LiteScene>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeScene_optional> for Option<fyrox_lite::lite_scene::LiteScene> {
                fn from(value: NativeScene_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativePhysics_optional {
                pub value: fyrox_lite::lite_physics::LitePhysics,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_physics::LitePhysics>> for NativePhysics_optional {
                fn from(value: Option<fyrox_lite::lite_physics::LitePhysics>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativePhysics_optional> for Option<fyrox_lite::lite_physics::LitePhysics> {
                fn from(value: NativePhysics_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeIntersection {
            pub _collider: NativeNode,
            pub _normal: NativeVector3,
            pub _position: NativeVector3,
            pub _feature: NativeFeatureId,
            pub _toi: f32,
            }

            #[repr(C)]
            pub struct NativeIntersection_optional {
                pub value: fyrox_lite::lite_physics::LiteIntersection,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_physics::LiteIntersection>> for NativeIntersection_optional {
                fn from(value: Option<fyrox_lite::lite_physics::LiteIntersection>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeIntersection_optional> for Option<fyrox_lite::lite_physics::LiteIntersection> {
                fn from(value: NativeIntersection_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeFeatureId {
            pub _kind: NativeFeatureKind,
            pub _id: i32,
            }

            #[repr(C)]
            pub struct NativeFeatureId_optional {
                pub value: fyrox_lite::lite_physics::LiteFeatureId,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_physics::LiteFeatureId>> for NativeFeatureId_optional {
                fn from(value: Option<fyrox_lite::lite_physics::LiteFeatureId>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeFeatureId_optional> for Option<fyrox_lite::lite_physics::LiteFeatureId> {
                fn from(value: NativeFeatureId_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeRayCastOptions {
            pub _ray_origin: NativeVector3,
            pub _ray_direction: NativeVector3,
            pub _max_len: f32,
            pub _groups: NativeInteractionGroups_optional,
            pub _sort_results: bool,
            }

            #[repr(C)]
            pub struct NativeRayCastOptions_optional {
                pub value: fyrox_lite::lite_physics::LiteRayCastOptions,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_physics::LiteRayCastOptions>> for NativeRayCastOptions_optional {
                fn from(value: Option<fyrox_lite::lite_physics::LiteRayCastOptions>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeRayCastOptions_optional> for Option<fyrox_lite::lite_physics::LiteRayCastOptions> {
                fn from(value: NativeRayCastOptions_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeInteractionGroups {
            pub _memberships: i32,
            pub _filter: i32,
            }

            #[repr(C)]
            pub struct NativeInteractionGroups_optional {
                pub value: fyrox_lite::lite_physics::LiteInteractionGroups,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_physics::LiteInteractionGroups>> for NativeInteractionGroups_optional {
                fn from(value: Option<fyrox_lite::lite_physics::LiteInteractionGroups>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeInteractionGroups_optional> for Option<fyrox_lite::lite_physics::LiteInteractionGroups> {
                fn from(value: NativeInteractionGroups_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct NativeRigidBody_optional {
                pub value: fyrox_lite::lite_physics::LiteRigidBody,
                pub has_value: i32,
            }

            impl From<Option<fyrox_lite::lite_physics::LiteRigidBody>> for NativeRigidBody_optional {
                fn from(value: Option<fyrox_lite::lite_physics::LiteRigidBody>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<NativeRigidBody_optional> for Option<fyrox_lite::lite_physics::LiteRigidBody> {
                fn from(value: NativeRigidBody_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct f32_optional {
                pub value: f32,
                pub has_value: i32,
            }

            impl From<Option<f32>> for f32_optional {
                fn from(value: Option<f32>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<f32_optional> for Option<f32> {
                fn from(value: f32_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }

            #[repr(C)]
            pub struct UserScript_optional {
                pub value: UserScript!!!,
                pub has_value: i32,
            }

            impl From<Option<UserScript!!!>> for UserScript_optional {
                fn from(value: Option<UserScript!!!>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<UserScript_optional> for Option<UserScript!!!> {
                fn from(value: UserScript_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }
