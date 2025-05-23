use mlua::prelude::*;
use raug::prelude::*;
use raug_ext::prelude::*;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, FromLua)]
pub struct LuaNode(pub(crate) Node);

impl LuaUserData for LuaNode {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("output", |_, this, output: LuaValue| match output {
            LuaValue::Integer(index) => Ok(LuaOutput(this.0.output(index as u32))),
            LuaValue::String(name) => Ok(LuaOutput(this.0.output(&*name.to_string_lossy()))),
            _ => Err(mlua::Error::external("Invalid output type")),
        });

        methods.add_meta_method("__add", |_, this, other: LuaValue| match other {
            LuaValue::Number(float) => Ok(LuaNode(&this.0 + float)),
            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                Ok(LuaNode(&this.0 + &data.borrow::<LuaNode>().unwrap().0))
            }
            _ => Err(mlua::Error::external("Invalid operand type")),
        });

        methods.add_meta_method("__sub", |_, this, other: LuaValue| match other {
            LuaValue::Number(float) => Ok(LuaNode(&this.0 - float)),
            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                Ok(LuaNode(&this.0 - &data.borrow::<LuaNode>().unwrap().0))
            }
            _ => Err(mlua::Error::external("Invalid operand type")),
        });

        methods.add_meta_method("__mul", |_, this, other: LuaValue| match other {
            LuaValue::Number(float) => Ok(LuaNode(&this.0 * float)),
            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                Ok(LuaNode(&this.0 * &data.borrow::<LuaNode>().unwrap().0))
            }
            _ => Err(mlua::Error::external("Invalid operand type")),
        });

        methods.add_meta_method("__div", |_, this, other: LuaValue| match other {
            LuaValue::Number(float) => Ok(LuaNode(&this.0 / float)),
            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                Ok(LuaNode(&this.0 / &data.borrow::<LuaNode>().unwrap().0))
            }
            _ => Err(mlua::Error::external("Invalid operand type")),
        });

        methods.add_meta_method("__mod", |_, this, other: LuaValue| match other {
            LuaValue::Number(float) => Ok(LuaNode(&this.0 % float)),
            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                Ok(LuaNode(&this.0 % &data.borrow::<LuaNode>().unwrap().0))
            }
            _ => Err(mlua::Error::external("Invalid operand type")),
        });

        methods.add_meta_method("__unm", |_, this, _: ()| Ok(LuaNode(-&this.0)));

        macro_rules! impl_unary_op {
            ($($method:ident),*) => {
                $(
                    methods.add_method(stringify!($method), |_, this, _: ()| {
                        Ok(LuaNode(this.0.$method()))
                    });
                )*
            };
        }

        macro_rules! impl_binary_op {
            ($($method:ident),*) => {
                $(
                    methods.add_method(stringify!($method), |_, this, other: LuaValue| {
                        match other {
                            LuaValue::Number(float) => Ok(LuaNode(this.0.clone().$method(float))),
                            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                                Ok(LuaNode(this.0.clone().$method(&data.borrow::<LuaNode>().unwrap().0)))
                            }
                            _ => Err(mlua::Error::external("Invalid operand type")),
                        }
                    });
                )*
            };
        }

        impl_unary_op!(sin, cos, tan, sqrt, ceil, floor, round, abs, signum, fract, recip);

        impl_binary_op!(powf, add, sub, mul, div, rem, min, max, atan2, hypot);
    }
}

#[derive(Clone)]
pub struct LuaOutput(pub(crate) Output);

impl LuaUserData for LuaOutput {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("node", |_, this, _: ()| Ok(LuaNode(this.0.node())));

        methods.add_meta_method("__add", |_, this, other: LuaValue| {
            let graph = crate::get_graph();
            match other {
                LuaValue::Number(float) => Ok(LuaOutput(
                    (this.0.clone() + (float as f32)).into_output(&graph),
                )),
                LuaValue::Integer(intenger) => {
                    Ok(LuaOutput((this.0.clone() + intenger).into_output(&graph)))
                }
                LuaValue::UserData(data) if data.is::<LuaOutput>() => Ok(LuaOutput(
                    (this.0.clone() + data.borrow::<LuaOutput>().unwrap().0.clone())
                        .into_output(&graph),
                )),
                _ => Err(mlua::Error::external("Invalid operand type")),
            }
        });

        methods.add_meta_method("__sub", |_, this, other: LuaValue| {
            let graph = crate::get_graph();
            match other {
                LuaValue::Number(float) => Ok(LuaOutput(
                    (this.0.clone() - (float as f32)).into_output(&graph),
                )),
                LuaValue::Integer(intenger) => {
                    Ok(LuaOutput((this.0.clone() - intenger).into_output(&graph)))
                }
                LuaValue::UserData(data) if data.is::<LuaOutput>() => Ok(LuaOutput(
                    (this.0.clone() - data.borrow::<LuaOutput>().unwrap().0.clone())
                        .into_output(&graph),
                )),
                _ => Err(mlua::Error::external("Invalid operand type")),
            }
        });

        methods.add_meta_method("__mul", |_, this, other: LuaValue| {
            let graph = crate::get_graph();
            match other {
                LuaValue::Number(float) => Ok(LuaOutput(
                    (this.0.clone() * (float as f32)).into_output(&graph),
                )),
                LuaValue::Integer(intenger) => {
                    Ok(LuaOutput((this.0.clone() * intenger).into_output(&graph)))
                }
                LuaValue::UserData(data) if data.is::<LuaOutput>() => Ok(LuaOutput(
                    (this.0.clone() * data.borrow::<LuaOutput>().unwrap().0.clone())
                        .into_output(&graph),
                )),
                _ => Err(mlua::Error::external("Invalid operand type")),
            }
        });

        methods.add_meta_method("__div", |_, this, other: LuaValue| {
            let graph = crate::get_graph();
            match other {
                LuaValue::Number(float) => Ok(LuaOutput(
                    (this.0.clone() / (float as f32)).into_output(&graph),
                )),
                LuaValue::Integer(intenger) => {
                    Ok(LuaOutput((this.0.clone() / intenger).into_output(&graph)))
                }
                LuaValue::UserData(data) if data.is::<LuaOutput>() => Ok(LuaOutput(
                    (this.0.clone() / data.borrow::<LuaOutput>().unwrap().0.clone())
                        .into_output(&graph),
                )),
                _ => Err(mlua::Error::external("Invalid operand type")),
            }
        });

        methods.add_meta_method("__mod", |_, this, other: LuaValue| {
            let graph = crate::get_graph();
            match other {
                LuaValue::Number(float) => Ok(LuaOutput(
                    (this.0.clone() % (float as f32)).into_output(&graph),
                )),
                LuaValue::Integer(intenger) => {
                    Ok(LuaOutput((this.0.clone() % intenger).into_output(&graph)))
                }
                LuaValue::UserData(data) if data.is::<LuaOutput>() => Ok(LuaOutput(
                    (this.0.clone() % data.borrow::<LuaOutput>().unwrap().0.clone())
                        .into_output(&graph),
                )),
                _ => Err(mlua::Error::external("Invalid operand type")),
            }
        });

        methods.add_meta_method("__unm", |_, this, _: ()| {
            let graph = crate::get_graph();
            Ok(LuaOutput((-this.0.clone()).into_output(&graph)))
        });

        methods.add_method("unwrap_or", |lua, this, default: LuaValue| {
            let graph = crate::get_graph();
            let default = LuaOutput::from_lua(default, lua)?;
            Ok(LuaOutput(
                this.0.clone().unwrap_or(default.0).into_output(&graph),
            ))
        });
    }
}

impl FromLua for LuaOutput {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        let graph = crate::get_graph();
        match value {
            LuaValue::Number(n) => Ok(LuaOutput((n as f32).into_output(&graph))),
            LuaValue::Integer(i) => Ok(LuaOutput(i.into_output(&graph))),
            LuaValue::Boolean(b) => Ok(LuaOutput(b.into_output(&graph))),
            LuaValue::String(s) => {
                let str = StringSignal::from_str(&s.to_string_lossy());
                Ok(LuaOutput(str.into_output(&graph)))
            }
            LuaValue::Table(tab) => {
                let values = tab
                    .sequence_values()
                    .collect::<Result<Vec<LuaValue>, LuaError>>()?;

                let output = match values[0].clone() {
                    LuaValue::Boolean(_) => {
                        let mut bools = Vec::new();
                        for value in values {
                            if let LuaValue::Boolean(b) = value {
                                bools.push(b);
                            } else {
                                return Err(LuaError::FromLuaConversionError {
                                    from: "LuaValue",
                                    to: "LuaOutput".to_string(),
                                    message: Some("Invalid LuaOutput".to_string()),
                                });
                            }
                        }
                        let list = List::from_slice(&bools);
                        list.into_output(&graph)
                    }
                    LuaValue::Number(_) => {
                        let mut floats = Vec::new();
                        for value in values {
                            if let LuaValue::Number(f) = value {
                                floats.push(f as f32);
                            } else {
                                return Err(LuaError::FromLuaConversionError {
                                    from: "LuaValue",
                                    to: "LuaOutput".to_string(),
                                    message: Some("Invalid LuaOutput".to_string()),
                                });
                            }
                        }
                        let list = List::from_slice(&floats);
                        list.into_output(&graph)
                    }
                    LuaValue::Integer(_) => {
                        let mut ints = Vec::new();
                        for value in values {
                            if let LuaValue::Integer(i) = value {
                                ints.push(i);
                            } else {
                                return Err(LuaError::FromLuaConversionError {
                                    from: "LuaValue",
                                    to: "LuaOutput".to_string(),
                                    message: Some("Invalid LuaOutput".to_string()),
                                });
                            }
                        }
                        let list = List::from_slice(&ints);
                        list.into_output(&graph)
                    }
                    LuaValue::String(_) => {
                        let mut strings = Vec::new();
                        for value in values {
                            if let LuaValue::String(s) = value {
                                strings.push(StringSignal::from_str(s.to_string_lossy().as_ref()));
                            } else {
                                return Err(LuaError::FromLuaConversionError {
                                    from: "LuaValue",
                                    to: "LuaOutput".to_string(),
                                    message: Some("Invalid LuaOutput".to_string()),
                                });
                            }
                        }
                        let list = List::from_slice(&strings);
                        list.into_output(&graph)
                    }
                    _ => {
                        return Err(LuaError::FromLuaConversionError {
                            from: "LuaValue",
                            to: "LuaOutput".to_string(),
                            message: Some("Invalid LuaOutput".to_string()),
                        });
                    }
                };

                Ok(LuaOutput(output))
            }
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
