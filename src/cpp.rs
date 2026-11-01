use std::fs;

use crate::{Config, MkboilerError};

pub fn cpp(config: &Config) -> Result<(), MkboilerError> {
    let content = "#include <iostream>
#include <stdlib.h>

int main(int argc, char *argv[]){
    return 0;
}";

    let output = config.output.unwrap_or("./main.cc"); 
    fs::write(output, content)
        .map_err(|e| MkboilerError::new(format!("could not write to file '{output}': {e}")))
}
