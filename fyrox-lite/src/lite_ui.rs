use fyrox::{
    core::{color, pool::Handle},
    gui::{
        brush,
        message::MessageDirection,
        text,
        widget,
        UiNode,
    },
};
use fyrox_lite_macro::{fyrox_lite_engine_class, fyrox_lite_pod};

use crate::{lite_math::PodVector2, script_context::with_script_context};

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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LiteText {
    handle: Handle<UiNode>,
}

impl LiteText {
    pub fn from(handle: Handle<UiNode>) -> Self {
        Self { handle }
    }

    pub fn inner(&self) -> Handle<UiNode> {
        self.handle
    }
}

#[fyrox_lite_engine_class(Text)]
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
            if let Some(foreground) = state.foregound {
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

impl From<Color> for color::Color {
    fn from(value: Color) -> Self {
        color::Color::from_rgba(value.r, value.g, value.b, value.a)
    }
}

impl From<Brush> for brush::Brush {
    fn from(value: Brush) -> Self {
        match value {
            Brush::Solid(color) => brush::Brush::Solid(color.into()),
            Brush::LinearGradient { from, to, stops } => brush::Brush::LinearGradient {
                from: from.into(),
                to: to.into(),
                stops: stops.into_iter().map(|it| it.into()).collect(),
            },
            Brush::RadialGradient { center, stops } => brush::Brush::RadialGradient {
                center: center.into(),
                stops: stops.into_iter().map(|it| it.into()).collect(),
            },
        }
    }
}

#[fyrox_lite_pod(TextBuilder)]
pub struct TextBuilder {
    pub foregound: Option<Brush>,
    pub font_size: Option<f32>,
}

/// Brush defines a way to fill an arbitrary surface.
#[derive(PartialEq)]
#[fyrox_lite_pod(Brush)]
pub enum Brush {
    /// A brush, that fills a surface with a solid color.
    Solid(Color),
    /// A brush, that fills a surface with a linear gradient, which is defined by two points in local coordinates
    /// and a set of stop points. See [`GradientPoint`] for more info.
    LinearGradient {
        /// Beginning of the gradient in local coordinates.
        from: PodVector2,
        /// End of the gradient in local coordinates.
        to: PodVector2,
        /// Stops of the gradient.
        stops: Vec<GradientPoint>,
    },
    /// A brush, that fills a surface with a radial gradient, which is defined by a center point in local coordinates
    /// and a set of stop points. See [`GradientPoint`] for more info.
    RadialGradient {
        /// Center of the gradient in local coordinates.
        center: PodVector2,
        /// Stops of the gradient.
        stops: Vec<GradientPoint>,
    },
}

#[derive(Copy, PartialOrd, PartialEq, Eq)]
#[fyrox_lite_pod(Color)]
pub struct Color {
    // Do not change order! OpenGL requires this order!
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(PartialEq)]
#[fyrox_lite_pod(GradientPoint)]
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
