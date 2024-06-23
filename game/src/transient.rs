use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use fyrox::core::blank_reflect;
use fyrox::core::{reflect::prelude::*, visitor::prelude::*};
use std::any::Any;

/// syntax sugar for `#[visit(skip)]` and `#[reflect(hidden)]`
#[derive(Debug, Default, Clone, Copy)]
pub struct Transient<T>(T);

impl<T> From<T> for Transient<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for Transient<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Transient<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Visit for Transient<T> {
    fn visit(
        &mut self,
        _name: &str,
        _visitor: &mut fyrox::core::visitor::Visitor,
    ) -> fyrox::core::visitor::VisitResult {
        Ok(())
    }
}

impl<T: Debug + 'static> Reflect for Transient<T> {
    blank_reflect!();
}
