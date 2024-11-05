use mlua::prelude::*;

use graph_builder::graph_builder;
use node_builder::LuaParam;
use raug::prelude::Param;

pub mod graph;
pub mod graph_builder;
pub mod node_builder;
pub mod runtime;

#[derive(Clone, FromLua)]
pub struct LuaBang;

impl LuaUserData for LuaBang {}

pub fn bang(_: &Lua, _: ()) -> LuaResult<LuaBang> {
    Ok(LuaBang)
}

pub fn param(_: &Lua, name: LuaString) -> LuaResult<LuaParam> {
    Ok(LuaParam(Param::new(name.to_str()?.to_string())))
}

pub fn sleep(_: &Lua, duration: f64) -> LuaResult<()> {
    std::thread::sleep(std::time::Duration::from_secs_f64(duration));
    Ok(())
}

#[mlua::lua_module]
fn raug(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("bang", lua.create_function(bang)?)?;
    exports.set("param", lua.create_function(param)?)?;
    exports.set("sleep", lua.create_function(sleep)?)?;
    exports.set("graph_builder", lua.create_function(graph_builder)?)?;
    Ok(exports)
}
