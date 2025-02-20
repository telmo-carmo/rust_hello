use pico_args::Arguments;

/*
parsing command line args example app

cargo add pico-args
*/


const HELP_MSG: &str = "\
App

USAGE:
  app [OPTIONS] --number NUMBER [INPUT]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --num NUMBER       Sets a number
  --opt-number NUMBER   Sets an optional number
  --width WIDTH         Sets width [default: 10]
  --output PATH         Sets an output path

ARGS:
  <INPUT>
";

fn main() {
    let mut pargs = Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP_MSG);
        std::process::exit(0);
    }

    let nv: i32 = pargs.opt_value_from_str("--num").unwrap().unwrap_or(0);

    let name: String = pargs
        .opt_value_from_str("--str")
        .unwrap()
        .unwrap_or_else(|| "None".to_string());

    let vf: bool = pargs.contains("-v");
    let remaining_args = pargs.finish();

    println!("Integer argument: {}", nv);
    println!("String argument: {}", name);
    println!("Bool argument: {}", vf);
    if !remaining_args.is_empty() {
        println!("Remaining arguments: {:?}", remaining_args);
    }
}
