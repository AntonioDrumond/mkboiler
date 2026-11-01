mod c;
mod nix;
mod python;
mod rust;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let fname = if args.len() > 2 {
            Some(args[2].as_str())
        } else {
            None
        };

        match args[1].as_str() {
            "default" | "default.nix" => nix::default(fname),
            "shell" | "shell.nix" => nix::shell(fname),
            "nix_module" | "mod.nix" | "module.nix" => nix::module(fname),
            "c" => c::c(fname),
            "cc" | "cplusplus" | "c++" | "cpp" => c::cpp(fname),
            "python3" => python::python3(fname),
            "rust" | "rustshell" | "rustShell" | "rust_shell" => rust::rust(fname),
            "icedshell" | "icedShell" | "iced_shell" => rust::iced(fname),
            _ => eprintln!("Unknown boilerplate: {}", args[1]),
        };
    } else {
        println!("Usage:");
        println!("mkboiler [boiler type] [file name (optional)]")
    }
}
