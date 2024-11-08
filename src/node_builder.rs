use mlua::prelude::*;
use raug::{
    builder::node_builder::{Input, Node, Output},
    prelude::{Message, Param},
};

use crate::LuaBang;

#[derive(Clone, FromLua)]
pub struct LuaNode(pub(crate) Node);

impl LuaUserData for LuaNode {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("input", |_, this, input: LuaValue| match input {
            LuaValue::Integer(index) => Ok(LuaInput(this.0.input(index as u32))),
            LuaValue::String(name) => Ok(LuaInput(this.0.input(&*name.to_string_lossy()))),
            _ => Err(mlua::Error::external("Invalid input type")),
        });

        methods.add_method("output", |_, this, output: LuaValue| match output {
            LuaValue::Integer(index) => Ok(LuaOutput(this.0.output(index as u32))),
            LuaValue::String(name) => Ok(LuaOutput(this.0.output(&*name.to_string_lossy()))),
            _ => Err(mlua::Error::external("Invalid output type")),
        });

        methods.add_method("to_audio", |_, this, _args: ()| {
            Ok(LuaNode(this.0.to_audio()))
        });
        methods.add_method("to_message", |_, this, _args: ()| {
            Ok(LuaNode(this.0.to_message()))
        });

        methods.add_meta_method("__add", |_, this, other: LuaValue| match other {
            LuaValue::Number(float) => Ok(LuaNode(&this.0 + float)),
            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                Ok(LuaNode(&this.0 + &data.borrow::<LuaNode>().unwrap().0))
            }
            LuaValue::UserData(data) if data.is::<LuaParam>() => Ok(LuaNode(
                &this.0 + data.borrow::<LuaParam>().unwrap().0.clone(),
            )),
            _ => Err(mlua::Error::external("Invalid operand type")),
        });

        methods.add_meta_method("__sub", |_, this, other: LuaValue| match other {
            LuaValue::Number(float) => Ok(LuaNode(&this.0 - float)),
            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                Ok(LuaNode(&this.0 - &data.borrow::<LuaNode>().unwrap().0))
            }
            LuaValue::UserData(data) if data.is::<LuaParam>() => Ok(LuaNode(
                &this.0 - data.borrow::<LuaParam>().unwrap().0.clone(),
            )),
            _ => Err(mlua::Error::external("Invalid operand type")),
        });

        methods.add_meta_method("__mul", |_, this, other: LuaValue| match other {
            LuaValue::Number(float) => Ok(LuaNode(&this.0 * float)),
            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                Ok(LuaNode(&this.0 * &data.borrow::<LuaNode>().unwrap().0))
            }
            LuaValue::UserData(data) if data.is::<LuaParam>() => Ok(LuaNode(
                &this.0 * data.borrow::<LuaParam>().unwrap().0.clone(),
            )),
            _ => Err(mlua::Error::external("Invalid operand type")),
        });

        methods.add_meta_method("__div", |_, this, other: LuaValue| match other {
            LuaValue::Number(float) => Ok(LuaNode(&this.0 / float)),
            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                Ok(LuaNode(&this.0 / &data.borrow::<LuaNode>().unwrap().0))
            }
            LuaValue::UserData(data) if data.is::<LuaParam>() => Ok(LuaNode(
                &this.0 / data.borrow::<LuaParam>().unwrap().0.clone(),
            )),
            _ => Err(mlua::Error::external("Invalid operand type")),
        });

        methods.add_meta_method("__mod", |_, this, other: LuaValue| match other {
            LuaValue::Number(float) => Ok(LuaNode(&this.0 % float)),
            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                Ok(LuaNode(&this.0 % &data.borrow::<LuaNode>().unwrap().0))
            }
            LuaValue::UserData(data) if data.is::<LuaParam>() => Ok(LuaNode(
                &this.0 % data.borrow::<LuaParam>().unwrap().0.clone(),
            )),
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
                            LuaValue::Number(float) => Ok(LuaNode(this.0.$method(float))),
                            LuaValue::UserData(data) if data.is::<LuaNode>() => {
                                Ok(LuaNode(this.0.$method(&data.borrow::<LuaNode>().unwrap().0)))
                            }
                            LuaValue::UserData(data) if data.is::<LuaParam>() => {
                                Ok(LuaNode(this.0.$method(data.borrow::<LuaParam>().unwrap().0.clone())))
                            }
                            _ => Err(mlua::Error::external("Invalid operand type")),
                        }
                    });
                )*
            };
        }

        impl_unary_op!(sin, cos, tan, sqrt, cbrt, ceil, floor, round, abs, signum, fract, recip);

        impl_binary_op!(powf, add, sub, mul, div, rem, min, max, atan2, hypot);
    }
}

#[derive(Clone, FromLua)]
pub struct LuaInput(pub(crate) Input);

impl LuaUserData for LuaInput {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("connect", |_, this, output: LuaOutput| {
            this.0.connect(&output.0);
            Ok(())
        });

        methods.add_method("set", |_, this, value: LuaValue| {
            match value {
                LuaValue::UserData(data) if data.is::<LuaBang>() => this.0.set(Message::Bang),
                LuaValue::Number(float) => this.0.set(Message::Float(float)),
                LuaValue::Integer(int) => this.0.set(Message::Int(int)),
                LuaValue::String(string) => this.0.set(Message::String(string.to_string_lossy())),
                _ => return Err(mlua::Error::external("Invalid message type")),
            }
            Ok(())
        });

        methods.add_method("param", |_, this, name: LuaString| {
            Ok(LuaParam(this.0.param(name.to_str()?.to_string())))
        });
    }
}

#[derive(Clone, FromLua)]
pub struct LuaOutput(pub(crate) Output);

impl LuaUserData for LuaOutput {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("connect", |_, this, input: LuaInput| {
            this.0.connect(&input.0);
            Ok(())
        });
    }
}

#[derive(Clone, FromLua)]
pub struct LuaParam(pub(crate) Param);

impl LuaUserData for LuaParam {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set", |_, this, value: LuaValue| {
            match value {
                LuaValue::Number(float) => this.0.set(float),
                LuaValue::Integer(int) => this.0.set(int),
                LuaValue::String(string) => this.0.set(string.to_string_lossy()),
                _ => return Err(mlua::Error::external("Invalid value type")),
            }
            Ok(())
        });

        methods.add_method_mut("get", |lua, this, _: ()| match this.0.get() {
            Some(value) => match value {
                Message::Bang => Ok(LuaValue::UserData(lua.create_userdata(LuaBang).unwrap())),
                Message::Int(int) => Ok(LuaValue::Integer(int)),
                Message::Float(float) => Ok(LuaValue::Number(float)),
                Message::String(string) => Ok(LuaValue::String(lua.create_string(&string)?)),
                _ => Err(mlua::Error::external("Invalid value type")),
            },
            None => Err(mlua::Error::external("Parameter is not set")),
        });
    }
}
