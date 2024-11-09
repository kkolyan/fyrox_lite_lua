use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use send_wrapper::SendWrapper;
use fyrox_lite::LiteDataType;
use crate::bindings_manual::{NativeInstanceId, UserScriptMessage};
use crate::scripted_app::APP;

//==================================================================================================

#[derive(Clone)]
pub struct AutoDisposableUserMessage {
    tracker: SendWrapper<Rc<UserMessageTracker>>,
}

impl Debug for AutoDisposableUserMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.tracker.value.fmt(f)
    }
}

impl LiteDataType for AutoDisposableUserMessage {}

impl AutoDisposableUserMessage {
    pub fn new(value: crate::UserScriptMessageImpl) -> Self {
        Self { tracker: SendWrapper::new(Rc::new(UserMessageTracker { value })) }
    }

    pub fn inner(&self) -> crate::UserScriptMessageImpl {
        self.tracker.value
    }
}

impl From<UserScriptMessage> for AutoDisposableUserMessage {
    fn from(value: UserScriptMessage) -> Self {
        Self::new(value)
    }
}

struct UserMessageTracker {
    value: crate::UserScriptMessageImpl,
}

impl Drop for UserMessageTracker {
    fn drop(&mut self) {
        APP.with_borrow(|it| {
            (it.as_ref().unwrap().functions.dispose_message)(self.value);
        });
    }
}

//==================================================================================================

#[derive(Debug, Clone)]
pub struct AutoDisposableScriptInstance {
    tracker: SendWrapper<Rc<ScriptInstanceTracker>>,
}

#[derive(Debug)]
struct ScriptInstanceTracker {
    value: NativeInstanceId,
}

impl AutoDisposableScriptInstance {
    pub fn new(value: NativeInstanceId) -> Self {
        Self { tracker: SendWrapper::new(Rc::new(ScriptInstanceTracker { value })) }
    }

    pub fn inner(&self) -> NativeInstanceId {
        self.tracker.value
    }
}

impl Drop for ScriptInstanceTracker {
    fn drop(&mut self) {
        APP.with_borrow(|it| {
            (it.as_ref().unwrap().functions.dispose_script)(self.value);
        });
    }
}

//==================================================================================================
