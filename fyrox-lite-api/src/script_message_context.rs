use std::{cell::RefCell, mem};

use fyrox::script::{ScriptMessageContext};

type StaticSc = ScriptMessageContext<'static, 'static, 'static>;

thread_local! {
    static SCRIPT_MESSAGE_CONTEXT: RefCell<Option<&'static mut StaticSc>> = RefCell::new(None);
}
const SC_404: &str = "Fyrox ScriptContext is not available outside of main thread and ";

/// the way to access Fyrox engine API from the Rust functions called from Lua
pub(crate) fn with_script_message_context<T, F>(f: F) -> T
where
    F: FnOnce(&mut StaticSc) -> T,
{
    SCRIPT_MESSAGE_CONTEXT.with_borrow_mut(|sc| {
        let sc = sc.as_mut().expect(SC_404);
        f(sc)
    })
}

pub(crate) fn without_script_message_context<T, F>(sc: &mut ScriptMessageContext, f: F) -> T
where
    F: FnOnce() -> T,
{
    // this code publishes local mut ref to threadlocal, and then takes it back. seems sound provided nobody else takes it from this threadlocal, which is asserted.
    unsafe {
        let sc: &mut ScriptMessageContext<'static, 'static, 'static> = mem::transmute(sc);
        SCRIPT_MESSAGE_CONTEXT.replace(Some(sc));
        let result = f();
        SCRIPT_MESSAGE_CONTEXT.take()
			.expect("threadlocally published mutable reference was taken from other party. memory safety compomised.");
		result
    }
}
