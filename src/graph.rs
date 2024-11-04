use mlua::prelude::*;
use raug::prelude::*;
use serde::Serialize;

#[derive(Clone, Serialize, FromLua)]
pub struct LuaGraph(pub(crate) Graph);

impl LuaUserData for LuaGraph {}
