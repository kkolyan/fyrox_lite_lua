use crate::{Constant, DataType, Field, Param};

pub trait NamedValue {
    fn name(&self) -> &str;
    fn ty(&self) -> &DataType;
}

impl NamedValue for Field {
    fn name(&self) -> &str {
        &self.name
    }

    fn ty(&self) -> &DataType {
        &self.ty
    }
}

impl NamedValue for Param {
    fn name(&self) -> &str {
        &self.name
    }

    fn ty(&self) -> &DataType {
        &self.ty
    }
}

impl NamedValue for Constant {
    fn name(&self) -> &str {
        &self.const_name
    }

    fn ty(&self) -> &DataType {
        &self.ty
    }
}
