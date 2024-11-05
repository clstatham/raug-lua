use mlua::prelude::*;
use raug::prelude::*;

#[derive(Clone, FromLua)]
pub struct LuaGraph(#[allow(unused)] pub(crate) Graph);

impl LuaUserData for LuaGraph {}
