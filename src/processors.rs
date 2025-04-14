use mlua::prelude::*;
use raug::prelude::*;
use raug_ext::prelude::*;

use crate::node::LuaOutput;

pub fn register_all(lua: &Lua, exports: &LuaTable) -> LuaResult<()> {
    macro_rules! processor {
        ($fn_name:ident, $name:ty) => {
            pub fn $fn_name(lua: &Lua, args: LuaMultiValue) -> LuaResult<LuaMultiValue> {
                let graph = crate::get_graph();
                let node = graph.add(<$name>::default());

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

    macro_rules! generic_processor {
        ($fn_name:ident, $name:ident => $($options:ty),*) => {
            pub fn $fn_name(lua: &Lua, args: LuaMultiValue) -> LuaResult<LuaMultiValue> {
                let graph = crate::get_graph();
                let first_arg = args.front().filter(|v| !v.is_nil()).cloned();
                let Some(first_arg) = first_arg else {
                    return Err(LuaError::RuntimeError(
                        "Expected at least one argument".to_string(),
                    ));
                };
                let first_arg = LuaOutput::from_lua(first_arg, lua)?;
                let node = match first_arg.0.signal_type() {
                    $(
                        t if t == <$options>::signal_type() => graph.add($name::<$options>::default()),
                    )*
                    _ => {
                        return Err(LuaError::RuntimeError(format!(
                            "Unsupported signal type: {}",
                            first_arg.0.signal_type().name()
                        )));
                    }
                };


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

    processor!(peak_limiter, PeakLimiter);

    processor!(metro, Metro);
    processor!(decay_env, DecayEnv);

    processor!(sin, Sin);
    processor!(cos, Cos);
    processor!(hypot, Hypot);
    processor!(log, Log);
    processor!(exp, Exp);
    processor!(sqrt, Sqrt);
    processor!(abs, Abs);
    processor!(powf, Powf);
    processor!(powi, Powi);

    processor!(f2i, Cast::<f32, i64>);
    processor!(i2f, Cast::<i64, f32>);

    generic_processor!(add, Add => f32, i64);
    generic_processor!(sub, Sub => f32, i64);
    generic_processor!(mul, Mul => f32, i64);
    generic_processor!(div, Div => f32, i64);
    generic_processor!(min, Min => f32, i64);
    generic_processor!(max, Max => f32, i64);
    generic_processor!(clamp, Clamp => f32, i64);

    generic_processor!(unwrap_or, UnwrapOr => f32, i64, bool, Option<f32>, Option<i64>, Option<bool>);
    generic_processor!(some, Some => f32, i64, bool, Option<f32>, Option<i64>, Option<bool>);

    Ok(())
}
