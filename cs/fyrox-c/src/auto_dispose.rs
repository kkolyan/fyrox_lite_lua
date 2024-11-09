use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use send_wrapper::SendWrapper;
use fyrox_lite::LiteDataType;
use crate::bindings_manual::NativeScriptAppFunctions;
use crate::scripted_app::{ScriptedApp, APP};

//==================================================================================================

pub trait DisposableHandle: Debug + Copy {
    fn dispose_handle(&self, funcs: &NativeScriptAppFunctions);
}

pub struct AutoDispose<T: DisposableHandle> {
    tracker: SendWrapper<Rc<UserMessageTracker<T>>>,
}

impl <T: DisposableHandle> Clone for AutoDispose<T> {
    fn clone(&self) -> Self {
        AutoDispose { tracker: self.tracker.clone() }
    }
}

impl <T: DisposableHandle> Debug for AutoDispose<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.tracker.value.fmt(f)
    }
}

impl <T: DisposableHandle> LiteDataType for AutoDispose<T> {}

impl <T: DisposableHandle> AutoDispose<T> {
    pub fn new(value: T) -> Self {
        Self { tracker: SendWrapper::new(Rc::new(UserMessageTracker { value })) }
    }

    pub fn inner(&self) -> T {
        self.tracker.value
    }
}

impl<T: DisposableHandle>  From<T> for AutoDispose<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

struct UserMessageTracker<T: DisposableHandle> {
    value: T,
}

impl <T: DisposableHandle> Drop for UserMessageTracker<T> {
    fn drop(&mut self) {
        APP.with_borrow(|it| {
            T::dispose_handle(&self.value, &it.as_ref().unwrap().functions);
        });
    }
}
