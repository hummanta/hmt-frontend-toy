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

use lalrpop_util::lalrpop_mod;

use crate::{ast::Program, lexer::Lexer};

lalrpop_mod!(grammar);

pub fn parse(source: &str) -> Result<Program, String> {
    let lexer = Lexer::new(source);
    let parser = grammar::ProgramParser::new();

    parser.parse(lexer).map_err(|e| format!("Parse error: {:?}", e))
}

#[cfg(test)]
mod test {
    use crate::{ast::*, parser};

    #[test]
    fn test_parse_var() -> Result<(), String> {
        let ast = parser::parse("var x = 42;")?;

        assert_eq!(ast.0.len(), 1);

        let statement = ast.0.first().unwrap();
        assert_eq!(
            statement,
            &Statement::Variable(VarStatement {
                name: String::from("x"),
                value: Box::new(Expression::Integer(42))
            })
        );

        Ok(())
    }
}
