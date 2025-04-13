use mlua::prelude::*;
use raug::prelude::*;
use raug_ext::prelude::*;

use crate::node::{LuaNode, LuaOutput};

impl FromLua for LuaOutput {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        let graph = crate::get_graph();
        match value {
            LuaValue::Number(n) => Ok(LuaOutput((n as f32).into_output(&graph))),
            LuaValue::Integer(i) => Ok(LuaOutput(i.into_output(&graph))),
            LuaValue::Boolean(b) => Ok(LuaOutput(b.into_output(&graph))),
            LuaValue::UserData(ud) => {
                if let Ok(output) = ud.borrow::<LuaOutput>() {
                    return Ok(output.clone());
                } else if let Ok(node) = ud.borrow::<LuaNode>() {
                    if node.0.num_outputs() == 1 {
                        return Ok(LuaOutput(node.0.output(0).clone()));
                    } else {
                        return Err(LuaError::FromLuaConversionError {
                            from: "LuaValue",
                            to: "LuaOutput".to_string(),
                            message: Some("Invalid LuaOutput".to_string()),
                        });
                    }
                }
                Err(LuaError::FromLuaConversionError {
                    from: "LuaValue",
                    to: "LuaOutput".to_string(),
                    message: Some("Invalid LuaOutput".to_string()),
                })
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: "LuaValue",
                to: "LuaOutput".to_string(),
                message: Some("Unsupported type".to_string()),
            }),
        }
    }
}

pub fn register_all(lua: &Lua, exports: &LuaTable) -> LuaResult<()> {
    macro_rules! processor {
        ($fn_name:ident, $name:ident) => {
            pub fn $fn_name(lua: &Lua, args: LuaMultiValue) -> LuaResult<LuaMultiValue> {
                let graph = crate::get_graph();
                let node = graph.add($name::default());

                for i in 0..node.num_inputs() {
                    if args.get(i).is_some_and(|v| !v.is_nil()) {
                        let input = LuaOutput::from_lua(args[i].clone(), lua)?;
                        node.input(i as u32).connect(input.0);
                    }
                }

                let mut values = LuaMultiValue::new();
                for i in 0..node.num_outputs() {
                    let output = node.output(i as u32);
                    let output = LuaOutput(output);
                    values.push_back(LuaValue::UserData(lua.create_userdata(output)?));
                }

                Ok(values)
            }

            exports.set(stringify!($fn_name), lua.create_function($fn_name)?)?;
        };
    }

    processor!(sample_rate, SampleRate);
    processor!(sine_oscillator, SineOscillator);
    processor!(bl_saw_oscillator, BlSawOscillator);
    processor!(phase_accumulator, PhaseAccumulator);
    processor!(sin, Sin);
    processor!(cos, Cos);

    Ok(())
}
