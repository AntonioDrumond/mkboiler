use std::fs::File;
use std::io::{Write};

pub fn c() {
    let content = "#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

int main(int argc, char *argv[]){
    return 0;
}
    ";
    let mut fil = File::create("./main.c").expect("ERROR: Could not open file");
    match write!(fil, "{content}") {
        Ok(()) => (),
        Err(e) => eprintln!("ERROR: Could not write to file.\n{e}"),
    }
}

pub fn cpp() {
    let content = "#include <iostream>
#include <stdlib.h>

int main(int argc, char *argv[]){
    return 0;
}
    ";
    let mut fil = File::create("./main.cc").expect("ERROR: Could not open file");
    match write!(fil, "{content}") {
        Ok(()) => (),
        Err(e) => eprintln!("ERROR: Could not write to file.\n{e}"),
    }
}
