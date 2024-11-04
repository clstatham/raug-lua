use mlua::prelude::*;

use graph_builder::graph_builder;
use serde::Serialize;

pub mod graph;
pub mod graph_builder;
pub mod node_builder;
pub mod runtime;

#[derive(Clone, Serialize, FromLua)]
pub struct LuaBang;

impl LuaUserData for LuaBang {}

pub fn bang(_: &Lua, _: ()) -> LuaResult<LuaBang> {
    Ok(LuaBang)
}

pub fn sleep(_: &Lua, duration: f64) -> LuaResult<()> {
    std::thread::sleep(std::time::Duration::from_secs_f64(duration));
    Ok(())
}

#[mlua::lua_module]
fn raug(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("bang", lua.create_function(bang)?)?;
    exports.set("sleep", lua.create_function(sleep)?)?;
    exports.set("graph_builder", lua.create_function(graph_builder)?)?;
    Ok(exports)
}
