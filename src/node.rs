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

        impl_unary_op!(sin, cos, tan, sqrt, ceil, floor, round, abs, signum, fract);

        impl_binary_op!(powf, add, sub, mul, div, rem, min, max, atan2, hypot);
    }
}

#[derive(Clone)]
pub struct LuaOutput(pub(crate) Output);

impl LuaUserData for LuaOutput {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("node", |_, this, _: ()| Ok(LuaNode(this.0.node())));
    }
}
