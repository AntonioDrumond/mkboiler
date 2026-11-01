use std::{
    env,
    error::Error,
};

use mkboiler::{Config, IndentStyle, MkboilerError, c, nix, python, rust};

#[derive(Default)]
struct Options<'a> {
    pub mkboiler_target: Option<&'a str>,
    pub help: Option<&'a str>,
    pub output: Option<&'a str>,
    pub use_tabs: bool,
    pub spaces: Option<u32>,
}

enum ArgState {
    Arg,
    Value(ValueState),
    Either(ValueState),
}

enum ValueState {
    Help,
    Output,
    Spaces,
}

impl<'a> From<Options<'a>> for Config<'a> {
    fn from(opts: Options<'a>) -> Self {
        let target = opts.mkboiler_target.unwrap_or_else(|| {
            unreachable!("invariant broken: the target must be known at this point")
        });

        let indent = if opts.use_tabs {
            IndentStyle::Tabs
        } else if let Some(spaces) = opts.spaces {
            IndentStyle::Spaces(spaces)
        } else {
            unreachable!("invariant broken: spaces must be populated when use_tabs is false");
        };

        Config {
            target,
            output: opts.output,
            indent,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut arg_state = ArgState::Arg;

    let mut opts = Options::default();

    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            arg_state = match arg_state {
                ArgState::Arg => process_arg(arg, &mut opts),
                ArgState::Value(value_state) => process_value(arg, value_state, &mut opts),
                ArgState::Either(value_state) => {
                    if !matches!(value_state, ValueState::Help) && arg.starts_with("-") {
                        apply_either_default(value_state, &mut opts);
                        process_arg(arg, &mut opts)
                    } else {
                        process_value(arg, value_state, &mut opts)
                    }
                }
            }?;
        }

        let config = Config::from(opts);

        match config.target {
            "default" | "default.nix" => Ok(nix::default(&config)?),
            "shell" | "shell.nix" => Ok(nix::shell(&config)?),
            "nix_module" | "mod.nix" | "module.nix" => Ok(nix::module(&config)?),
            "c" => Ok(c::c(&config)?),
            "cc" | "c++" | "cpp" => Ok(c::cpp(&config)?),
            "python" | "python3" => Ok(python::python3(&config)?),
            "rust"  => Ok(rust::rust(&config)?),
            "iced" => Ok(rust::iced(&config)?),
            _ => Err(MkboilerError::new(format!(
                "Unknown boilerplate: {}",
                config.target
            ))),
        }?;
    } else {
        println!("Usage:");
        println!("mkboiler <boiler type> [<args>]")
    }

    Ok(())
}

fn apply_either_default(value_state: ValueState, opts: &mut Options) {
    match value_state {
        ValueState::Spaces => {
            if opts.spaces.is_none() {
                opts.spaces = Some(4);
            }
        }
        _ => {}
    }
}

fn process_arg<'a>(arg: &'a str, opts: &mut Options<'a>) -> Result<ArgState, MkboilerError> {
    if !arg.starts_with("-") {
        opts.mkboiler_target = Some(arg);
        Ok(ArgState::Arg)
    } else {
        match arg {
            "-h" | "--help" => Ok(ArgState::Either(ValueState::Help)),
            "-o" | "--output" => Ok(ArgState::Value(ValueState::Output)),
            "-t" | "--tabs" => {
                if opts.spaces.is_some() {
                    Err(MkboilerError::new(
                        "the argument '--tabs' cannot be used with '--spaces [<num>]'".to_owned(),
                    ))
                } else {
                    opts.use_tabs = true;
                    Ok(ArgState::Arg)
                }
            }
            "-s" | "--spaces" => {
                if opts.use_tabs {
                    Err(MkboilerError::new(
                        "the argument '--spaces [<num>]' cannot be used with '--tabs'".to_owned(),
                    ))
                } else {
                    Ok(ArgState::Either(ValueState::Spaces))
                }
            }
            _ => Err(MkboilerError::new(format!("Unknown argument: {}", arg))),
        }
    }
}

fn process_value<'a>(
    value: &'a str,
    value_state: ValueState,
    opts: &mut Options<'a>,
) -> Result<ArgState, MkboilerError> {
    match value_state {
        ValueState::Help => {
            opts.help = Some(value);
            Ok(ArgState::Arg)
        }
        ValueState::Output => {
            opts.output = Some(value);
            Ok(ArgState::Arg)
        }
        ValueState::Spaces => {
            if let Ok(space_amount) = value.parse::<u32>() {
                opts.spaces = Some(space_amount);
                Ok(ArgState::Arg)
            } else {
                Err(MkboilerError::new(
                    "Invalid value (spaces): expected positive integer".to_owned(),
                ))
            }
        }
    }
}
