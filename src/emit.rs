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

use std::collections::HashMap;

use cranelift::{
    module::FuncId,
    object::ObjectModule,
    prelude::{EntityRef, FunctionBuilder, Value, Variable},
};

pub trait Emitter {
    fn emit(&self, ctx: &mut EmitContext) -> Value;
}

pub struct EmitContext<'a> {
    pub module: &'a mut ObjectModule,
    pub builder: FunctionBuilder<'a>,
    pub functions: HashMap<String, FuncId>,
    pub variables: HashMap<String, Variable>,
    pub index: usize,
}

impl<'a> EmitContext<'a> {
    pub fn new(module: &'a mut ObjectModule, builder: FunctionBuilder<'a>) -> Self {
        Self { module, builder, functions: HashMap::new(), variables: HashMap::new(), index: 0 }
    }

    pub fn declare_var(&mut self, name: &str) -> Variable {
        let var = Variable::new(self.index);
        self.index += 1;
        self.variables.insert(name.to_string(), var);
        var
    }

    pub fn get_variable(&self, name: &str) -> Option<Variable> {
        self.variables.get(name).cloned()
    }
}
