use std::fs::File;
use std::io::{Write};

pub fn c(name: Option<&str>) {
    let content = "#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

int main(int argc, char *argv[]){
    return 0;
}
    ";
    let fname = match name {
        Some(s) => s,
        None => "./main.c",
    };
    let mut fil = File::create(fname).expect("ERROR: Could not open file");
    match write!(fil, "{content}") {
        Ok(()) => (),
        Err(e) => eprintln!("ERROR: Could not write to file.\n{e}"),
    }
}

pub fn cpp(name: Option<&str>) {
    let content = "#include <iostream>
#include <stdlib.h>

int main(int argc, char *argv[]){
    return 0;
}
    ";
    let fname = match name {
        Some(s) => s,
        None => "./main.cc",
    };
    let mut fil = File::create(fname).expect("ERROR: Could not open file");
    match write!(fil, "{content}") {
        Ok(()) => (),
        Err(e) => eprintln!("ERROR: Could not write to file.\n{e}"),
    }
}
