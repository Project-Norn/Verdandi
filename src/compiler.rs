use crate::frontend::lexer::tokenize;
use crate::frontend::parser::parse;
use std::fs;

pub fn compile_to_file(input_file: String, output_file: String) -> Result<(), String> {
    match fs::read_to_string(input_file) {
        Ok(source) => {
            compile(source)?;
            Ok(())
            // let output = compile(source);
            // match fs::write(output_file, output) {
            //     Ok(_) => Ok(()),
            //     Err(err) => Err(format!("failed to compile: {}", err)),
            // }
        }
        Err(err) => Err(format!("failed to compile: {}", err)),
    }
}

pub fn compile(source: String) -> Result<(), String> {
    let tokens = tokenize(source)?;
    let nodes = parse(tokens)?;
    println!("{:?}", nodes);
    Ok(())
}
