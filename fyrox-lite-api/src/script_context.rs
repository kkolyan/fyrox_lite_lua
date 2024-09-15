use std::{cell::RefCell, mem};

use fyrox::script::{ScriptContext, ScriptMessageContext};

type StaticSc = ScriptContext<'static, 'static, 'static>;

thread_local! {
    static SCRIPT_CONTEXT: RefCell<Option<&'static mut Context<'static, 'static>>> = RefCell::new(None);
}
const SC_404: &str = "Fyrox ScriptContext is not available outside of main thread and ";

/// the way to access Fyrox engine API from the Rust functions called from Lua
pub(crate) fn with_script_context<T, F>(f: F) -> T
where
    F: FnOnce(&mut Context) -> T,
{
    SCRIPT_CONTEXT.with_borrow_mut(|sc| {
        let sc = sc.as_mut().expect(SC_404);
        f(sc)
    })
}

pub(crate) fn without_script_context<T, F>(sc: &mut Context, f: F) -> T
where
    F: FnOnce() -> T,
{
    // this code publishes local mut ref to threadlocal, and then takes it back. seems sound provided nobody else takes it from this threadlocal, which is asserted.
    unsafe {
		SCRIPT_CONTEXT.with(|it| assert!(it.borrow().is_none(), "script context reentrancy is not supported"));
        let sc: &mut Context<'static, 'static> = mem::transmute(sc);
        SCRIPT_CONTEXT.replace(Some(sc));
        let result = f();
        SCRIPT_CONTEXT.take()
			.expect("threadlocally published mutable reference was taken from other party. memory safety compomised.");
		result
    }
}

pub enum Context<'a, 'b> {
	SC(&'a mut ScriptContext<'b, 'b, 'b>),
	SMC(&'a mut ScriptMessageContext<'b, 'b, 'b>),
}