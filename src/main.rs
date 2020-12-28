pub mod ast;
pub mod lexer;
pub mod parser;
pub mod run;
pub mod token;

use crate::lexer::get_tokens;
use crate::parser::parse_tokens;
use crate::run::run;

fn main() {
    let input = "
        PRINT \"H         D\"
        PRINT \" E       L \"
        PRINT \"  L     R  \"
        PRINT \"   L   O   \"
        PRINT \"    O W    \"
    ";
    let tokens = get_tokens(input);
    let program = parse_tokens(tokens);

    run(program);
}
