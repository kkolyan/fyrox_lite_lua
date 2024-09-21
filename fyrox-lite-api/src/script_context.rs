use std::{cell::RefCell, mem};

use fyrox::{
    core::pool::Handle,
    engine::{GraphicsContext, ScriptMessageDispatcher},
    scene::{node::Node, Scene},
    script::{PluginsRefMut, ScriptContext, ScriptDeinitContext, ScriptMessageContext, ScriptMessageSender},
};

type StaticSc = UnifiedContext<'static, 'static, 'static>;

thread_local! {
    static SCRIPT_CONTEXT: RefCell<Option<&'static mut StaticSc>> = RefCell::new(None);
}
const SC_404: &str = "Fyrox ScriptContext is not available outside of main thread and ";

/// the way to access Fyrox engine API from the Rust functions called from Lua
pub fn with_script_context<T, F>(f: F) -> T
where
    F: FnOnce(&mut StaticSc) -> T,
{
    SCRIPT_CONTEXT.with_borrow_mut(|sc| {
        let sc = sc.as_mut().expect(SC_404);
        f(sc)
    })
}

pub fn without_script_context<T, F>(sc: &mut dyn UnsafeAsUnifiedContext<'_, '_, '_>, f: F) -> T
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
        let usc = &mut sc.as_unified_context();
        let usc: &mut UnifiedContext<'static, 'static, 'static> = mem::transmute(usc);
        SCRIPT_CONTEXT.replace(Some(usc));
        let result = f();
        SCRIPT_CONTEXT.take()
			.expect("threadlocally published mutable reference was taken from other party. memory safety compomised.");
        result
    }
}

pub trait UnsafeAsUnifiedContext<'a, 'b, 'c> {
    unsafe fn as_unified_context(&mut self) -> UnifiedContext<'a, 'b, 'c>;
    fn plugins(&mut self) -> &mut PluginsRefMut<'a>;
}

impl<'a, 'b, 'c> UnsafeAsUnifiedContext<'a, 'b, 'c> for ScriptContext<'a, 'b, 'c> {
    unsafe fn as_unified_context(&mut self) -> UnifiedContext<'a, 'b, 'c> {
        let sc = self as *mut ScriptContext;
        UnifiedContext {
            dt: Some(sc.read().dt),
            scene: sc.read().scene,
            handle: Some(sc.read().handle),
            plugins: sc.read().plugins,
            message_sender: sc.read().message_sender,
            message_dispatcher: None,
            graphics_context: sc.read().graphics_context,
        }
    }
    
    fn plugins(&mut self) -> &mut PluginsRefMut<'a> {
        &mut self.plugins
    }
}

impl<'a, 'b, 'c> UnsafeAsUnifiedContext<'a, 'b, 'c> for ScriptMessageContext<'a, 'b, 'c> {
    unsafe fn as_unified_context(&mut self) -> UnifiedContext<'a, 'b, 'c> {
        let sc = self as *mut ScriptMessageContext;
        UnifiedContext {
            dt: Some(sc.read().dt),
            scene: sc.read().scene,
            handle: Some(sc.read().handle),
            plugins: sc.read().plugins,
            message_sender: sc.read().message_sender,
            message_dispatcher: None,
            graphics_context: sc.read().graphics_context,
        }
    }
    
    fn plugins(&mut self) -> &mut PluginsRefMut<'a> {
        &mut self.plugins
    }
}

impl<'a, 'b, 'c> UnsafeAsUnifiedContext<'a, 'b, 'c> for ScriptDeinitContext<'a, 'b, 'c> {
    unsafe fn as_unified_context(&mut self) -> UnifiedContext<'a, 'b, 'c> {
        let sc = self as *mut ScriptDeinitContext;
        UnifiedContext {
            dt: None,
            scene: sc.read().scene,
            handle: None,
            plugins: sc.read().plugins,
            message_sender: sc.read().message_sender,
            message_dispatcher: None,
            graphics_context: sc.read().graphics_context,
        }
    }
    
    fn plugins(&mut self) -> &mut PluginsRefMut<'a> {
        &mut self.plugins
    }
}

pub struct UnifiedContext<'a, 'b, 'c> {
    pub dt: Option<f32>,
    pub scene: &'b mut Scene,
    pub handle: Option<Handle<Node>>,
    pub plugins: PluginsRefMut<'a>,
    pub message_sender: &'c ScriptMessageSender,
    pub message_dispatcher: Option<&'c mut ScriptMessageDispatcher>,
    pub graphics_context: &'a mut GraphicsContext,
}
