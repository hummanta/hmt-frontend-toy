// Copyright (c) The Hummanta Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cranelift::prelude::{types::I64, EntityRef, InstBuilder, Value};

use crate::emit::{EmitContext, Emitter};

#[derive(Clone, Debug)]
pub struct Program(pub Vec<Statement>);

impl Emitter for Program {
    fn emit(&self, ctx: &mut EmitContext) -> Value {
        let entry = ctx.builder.create_block();
        ctx.builder.switch_to_block(entry);

        for stmt in &self.0 {
            stmt.emit(ctx);
        }

        ctx.builder.ins().return_(&[]);
        Value::new(0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Variable(VarStatement),
    Print(PrintStatement),
}

impl Emitter for Statement {
    fn emit(&self, ctx: &mut EmitContext) -> Value {
        match self {
            Statement::Variable(stmt) => stmt.emit(ctx),
            Statement::Print(stmt) => stmt.emit(ctx),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarStatement {
    pub name: String,
    pub value: Box<Expression>,
}

impl Emitter for VarStatement {
    fn emit(&self, ctx: &mut EmitContext) -> Value {
        let val = self.value.emit(ctx);
        let var = ctx.declare_var(&self.name);
        ctx.builder.declare_var(var, I64);
        ctx.builder.def_var(var, val);
        val
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrintStatement {
    pub value: Box<Expression>,
}

impl Emitter for PrintStatement {
    fn emit(&self, ctx: &mut EmitContext) -> Value {
        self.value.emit(ctx)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Integer(i64),
    Variable(String),
    BinaryOperation { lhs: Box<Expression>, operator: Operator, rhs: Box<Expression> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Emitter for Expression {
    fn emit(&self, ctx: &mut EmitContext) -> Value {
        match self {
            Expression::Integer(num) => ctx.builder.ins().iconst(I64, *num),
            Expression::Variable(name) => {
                let var = ctx
                    .get_variable(name)
                    .unwrap_or_else(|| panic!("Undefined variable: {}", name));
                ctx.builder.use_var(var)
            }
            Expression::BinaryOperation { lhs, operator, rhs } => {
                let (lhs, rhs) = (lhs.emit(ctx), rhs.emit(ctx));

                match operator {
                    Operator::Add => ctx.builder.ins().iadd(lhs, rhs),
                    Operator::Sub => ctx.builder.ins().isub(lhs, rhs),
                    Operator::Mul => ctx.builder.ins().imul(lhs, rhs),
                    Operator::Div => ctx.builder.ins().sdiv(lhs, rhs),
                }
            }
        }
    }
}
