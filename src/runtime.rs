use mlua::prelude::*;
use raug::{prelude::*, runtime::RuntimeHandle};

#[derive(Clone, FromLua)]
pub struct LuaRuntime(pub(crate) Runtime);

impl LuaUserData for LuaRuntime {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("run_for", |_, this, duration: f64| {
            this.0
                .run_for(
                    Duration::from_secs_f64(duration),
                    Backend::Default,
                    Device::Default,
                )
                .unwrap();
            Ok(())
        });

        methods.add_method_mut("run", |_, this, _: ()| {
            let handle = this.0.run(Backend::Default, Device::Default).unwrap();
            Ok(LuaRuntimeHandle(handle))
        });
    }
}

#[derive(Clone, FromLua)]
pub struct LuaRuntimeHandle(pub(crate) RuntimeHandle);

impl LuaUserData for LuaRuntimeHandle {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("stop", |_, this, _: ()| {
            this.0.stop();
            Ok(())
        });
    }
}
