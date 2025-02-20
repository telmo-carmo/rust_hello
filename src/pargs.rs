use pico_args::Arguments;

/*
parsing command line args example app

cargo add pico-args

OR

[dependencies]
pico-args = { version = "0.5", features = ["eq-separator"] } 

*/

const HELP_MSG: &str = "\
App XX version 1.0.1

USAGE:
  app [OPTIONS] -t NUM  [INPUT]

FLAGS:
  -h, --help           Prints help information

OPTIONS:
  -t NUMBER            Sets a number
  -n. --num NUMBER     Sets an optional number [default: 0]
  -s, --str STRING     Sets an optional str
  -v                   Sets verbose flag

ARGS:
  <INPUT>
";

fn main() {
    let mut pargs = Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP_MSG);
        std::process::exit(0);
    }

    match pargs.value_from_str::<&str, i32>("-t") {
        Ok(t ) => println!("arg -t = {}", t),
        Err(e) => {
            eprintln!("{} - Missing -t arg", e);
            std::process::exit(1);
        }
    }

    let nv: i32 = pargs
        .opt_value_from_str(["-n", "--num"])
        .unwrap()
        .unwrap_or(0);

    let name: String = pargs
        .opt_value_from_str(["-s", "--str"])
        .unwrap()
        .unwrap_or_else(|| "None".to_string());

    let vf: bool = pargs.contains("-v");
    let remaining_args = pargs.finish();

    println!("Integer argument: {}", nv);
    println!("String argument: {}", name);
    println!("Bool argument: {}", vf);
    if !remaining_args.is_empty() {
        println!("Remaining arguments: {:?}", remaining_args);
        println!("INPUT = {}", remaining_args[0].to_str().unwrap());
    }
}
