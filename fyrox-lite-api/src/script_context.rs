use std::{cell::RefCell, mem};

use fyrox::{
    core::pool::Handle, engine::{GraphicsContext, ScriptMessageDispatcher}, scene::{node::Node, Scene}, script::{PluginsRefMut, ScriptContext, ScriptMessageContext, ScriptMessageSender}
};

type StaticSc = UnifiedContext<'static, 'static, 'static>;

thread_local! {
    static SCRIPT_CONTEXT: RefCell<Option<&'static mut StaticSc>> = RefCell::new(None);
}
const SC_404: &str = "Fyrox ScriptContext is not available outside of main thread and ";

/// the way to access Fyrox engine API from the Rust functions called from Lua
pub(crate) fn with_script_context<T, F>(f: F) -> T
where
    F: FnOnce(&mut StaticSc) -> T,
{
    SCRIPT_CONTEXT.with_borrow_mut(|sc| {
        let sc = sc.as_mut().expect(SC_404);
        f(sc)
    })
}

pub(crate) fn without_script_context<T, F>(sc: &mut ScriptContext, f: F) -> T
where
    F: FnOnce() -> T,
{
    // this code publishes local mut ref to threadlocal, and then takes it back. seems sound provided nobody else takes it from this threadlocal, which is asserted.
    unsafe {
        SCRIPT_CONTEXT.with(|it| {
            assert!(
                it.borrow().is_none(),
                "script context reentrancy is not supported"
            )
        });
        let sc = sc as *mut ScriptContext;
        let usc: &mut UnifiedContext<'_, '_, '_> = &mut UnifiedContext {
            dt: sc.read().dt,
            scene: sc.read().scene,
            handle: sc.read().handle,
            plugins: sc.read().plugins,
            message_sender: sc.read().message_sender,
            message_dispatcher: Some(sc.read().message_dispatcher),
            graphics_context: sc.read().graphics_context,
        };
        let usc: &mut UnifiedContext<'static, 'static, 'static> = mem::transmute(usc);
        SCRIPT_CONTEXT.replace(Some(usc));
        let result = f();
        SCRIPT_CONTEXT.take()
			.expect("threadlocally published mutable reference was taken from other party. memory safety compomised.");
        result
    }
}

pub(crate) fn without_script_message_context<T, F>(sc: &mut ScriptMessageContext, f: F) -> T
where
    F: FnOnce() -> T,
{
    // this code publishes local mut ref to threadlocal, and then takes it back. seems sound provided nobody else takes it from this threadlocal, which is asserted.
    unsafe {
        let sc = sc as *mut ScriptMessageContext;
        let usc: &mut UnifiedContext<'_, '_, '_> = &mut UnifiedContext {
            dt: sc.read().dt,
            scene: sc.read().scene,
            handle: sc.read().handle,
            plugins: sc.read().plugins,
            message_sender: sc.read().message_sender,
            message_dispatcher: None,
            graphics_context: sc.read().graphics_context,
        };
        let usc: &mut UnifiedContext<'static, 'static, 'static> = mem::transmute(usc);
        SCRIPT_CONTEXT.replace(Some(usc));
        let result = f();
        SCRIPT_CONTEXT.take()
			.expect("threadlocally published mutable reference was taken from other party. memory safety compomised.");
		result
    }
}

pub struct UnifiedContext<'a, 'b, 'c> {
    pub dt: f32,
    pub scene: &'b mut Scene,
    pub handle: Handle<Node>,
    pub plugins: PluginsRefMut<'a>,
    pub message_sender: &'c ScriptMessageSender,
    pub message_dispatcher: Option<&'c mut ScriptMessageDispatcher>,
    pub graphics_context: &'a mut GraphicsContext,
}
