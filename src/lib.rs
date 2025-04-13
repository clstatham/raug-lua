use std::sync::OnceLock;

use mlua::prelude::*;

use node::{LuaNode, LuaOutput};
use raug::{
    graph::Graph,
    prelude::{AudioStream, CpalStream},
};

use graph::graph;

pub mod graph;
pub mod node;
pub mod processors;

pub static GRAPH: OnceLock<Graph> = OnceLock::new();

pub fn get_graph() -> Graph {
    GRAPH.get_or_init(Graph::new).clone()
}

pub fn sleep(_: &Lua, duration: f64) -> LuaResult<()> {
    std::thread::sleep(std::time::Duration::from_secs_f64(duration));
    Ok(())
}

pub fn audio_output(lua: &Lua, input: LuaValue) -> LuaResult<LuaNode> {
    let graph = get_graph();
    let node = graph.add_audio_output();
    let input_output = LuaOutput::from_lua(input, lua)?;
    node.input(0).connect(input_output.0);
    Ok(LuaNode(node))
}

pub fn run_for(_lua: &Lua, duration: f64) -> LuaResult<()> {
    let graph = get_graph();
    let mut stream = CpalStream::default();
    stream.spawn(&graph).unwrap();
    stream.play().unwrap();
    let start = std::time::Instant::now();
    while start.elapsed().as_secs_f64() < duration {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    stream.stop().unwrap();
    Ok(())
}

#[mlua::lua_module]
fn raug(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("sleep", lua.create_function(sleep)?)?;
    exports.set("graph", lua.create_function(graph)?)?;
    exports.set("audio_output", lua.create_function(audio_output)?)?;
    exports.set("run_for", lua.create_function(run_for)?)?;
    processors::register_all(lua, &exports)?;
    Ok(exports)
}
