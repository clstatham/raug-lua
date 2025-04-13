use mlua::prelude::*;
use raug::prelude::*;
use raug_ext::prelude::*;

use crate::{get_graph, node::LuaNode};

#[derive(Clone, Default, FromLua)]
pub struct LuaGraph(Graph);

impl LuaUserData for LuaGraph {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("input", |_lua, this, _args: ()| Ok(this.input()));
        methods.add_method("output", |_lua, this, _args: ()| Ok(this.output()));

        methods.add_method(
            "sample_rate",
            |_lua, this, _args: ()| Ok(this.sample_rate()),
        );
    }
}

impl LuaGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(&self) -> LuaNode {
        LuaNode(self.0.add_audio_input())
    }

    pub fn output(&self) -> LuaNode {
        LuaNode(self.0.add_audio_output())
    }

    pub fn sample_rate(&self) -> LuaNode {
        LuaNode(self.0.add(SampleRate::default()))
    }
}

pub fn graph(lua: &Lua, _args: ()) -> LuaResult<LuaAnyUserData> {
    lua.create_userdata(LuaGraph(get_graph()))
}
