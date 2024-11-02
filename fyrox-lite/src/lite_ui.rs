use fyrox::{
    core::{color, pool::Handle},
    gui::{brush, message::MessageDirection, text, widget, UiNode},
};
use lite_macro::lite_api;

use crate::{
    externalizable::Externalizable, lite_math::PodVector2, script_context::with_script_context,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LiteUiNode {
    handle: Handle<UiNode>,
}

impl LiteUiNode {
    pub fn new(handle: Handle<UiNode>) -> Self {
        Self { handle }
    }

    pub fn inner(&self) -> Handle<UiNode> {
        self.handle
    }
}

#[lite_api(class=UiNode)]
impl LiteUiNode {}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LiteText {
    handle: Handle<UiNode>,
}

impl LiteText {
    pub fn inner(&self) -> Handle<UiNode> {
        self.handle
    }
}

impl From<Handle<UiNode>> for LiteText {
    fn from(handle: Handle<UiNode>) -> Self {
        Self { handle }
    }
}

#[lite_api(class=Text)]
impl LiteText {
    pub fn set_text_async(&self, text: String) {
        with_script_context(|ctx| {
            ctx.ui().first_mut().send_message(text::TextMessage::text(
                self.handle,
                MessageDirection::ToWidget,
                text,
            ));
        })
    }

    pub fn new(state: TextBuilder) -> LiteText {
        with_script_context(|ctx| {
            let mut wb = widget::WidgetBuilder::new();
            if let Some(foreground) = state.foreground {
                wb = wb.with_foreground(foreground.into());
            }
            let mut builder = text::TextBuilder::new(wb);
            if let Some(font_size) = state.font_size {
                builder = builder.with_font_size(font_size);
            }
            LiteText {
                handle: builder.build(&mut ctx.ui().first_mut().build_ctx()),
            }
        })
    }
}

impl Externalizable for LiteText {
    fn to_external(&self) -> u128 {
        self.handle.encode_to_u128()
    }

    fn from_external(handle: u128) -> Self {
        Self {
            handle: Handle::decode_from_u128(handle),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
#[lite_api]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Color> for color::Color {
    fn from(value: Color) -> Self {
        color::Color::from_rgba(value.r, value.g, value.b, value.a)
    }
}

impl From<Brush> for brush::Brush {
    fn from(value: Brush) -> Self {
        if let Some(color) = value.solid_color {
            return brush::Brush::Solid(color.into());
        }
        if let Some(LinearGradient { from, to, stops }) = value.linear_gradient {
            return brush::Brush::LinearGradient {
                from: from.into(),
                to: to.into(),
                stops: stops.into_iter().map(|it| it.into()).collect(),
            }
        }
        if let Some(RadialGradient { center, stops }) = value.radial_gradient {
            return brush::Brush::RadialGradient {
                center: center.into(),
                stops: stops.into_iter().map(|it| it.into()).collect(),
            };
        }
        panic!("Unsupported brush type: {:?}", value)
    }
}

#[derive(Debug, Clone)]
#[lite_api(class=TextBuilder)]
pub struct TextBuilder {
    pub foreground: Option<Brush>,
    pub font_size: Option<f32>,
}

/// Brush defines a way to fill an arbitrary surface.
#[derive(Debug, Clone, PartialEq)]
#[lite_api]
pub struct Brush {
    /// A brush, that fills a surface with a solid color.
    pub solid_color: Option<Color>,

    /// A brush, that fills a surface with a linear gradient, which is defined by two points in local coordinates
    /// and a set of stop points. See [`GradientPoint`] for more info.
    pub linear_gradient: Option<LinearGradient>,
    
    /// A brush, that fills a surface with a radial gradient, which is defined by a center point in local coordinates
    /// and a set of stop points. See [`GradientPoint`] for more info.
    pub radial_gradient: Option<RadialGradient>,
}
#[derive(Debug, Clone, PartialEq)]
#[lite_api]
pub struct LinearGradient  {
    /// Beginning of the gradient in local coordinates.
    pub from: PodVector2,
    /// End of the gradient in local coordinates.
    pub to: PodVector2,
    /// Stops of the gradient.
    pub stops: Vec<GradientPoint>,
}

#[derive(Debug, Clone, PartialEq)]
#[lite_api]
pub struct RadialGradient {
    /// Center of the gradient in local coordinates.
    pub center: PodVector2,
    /// Stops of the gradient.
    pub stops: Vec<GradientPoint>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[lite_api(class=GradientPoint)]
pub struct GradientPoint {
    /// A distance from an origin of the gradient.
    pub stop: f32,
    /// Color of the point.
    pub color: Color,
}

impl From<GradientPoint> for brush::GradientPoint {
    fn from(value: GradientPoint) -> Self {
        Self {
            stop: value.stop,
            color: value.color.into(),
        }
    }
}
