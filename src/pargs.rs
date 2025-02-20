use pico_args::Arguments;

/*
parsing command line args example app

cargo add pico-args
*/

fn main() {
    let mut pargs = Arguments::from_env();

    let nv: i32 = pargs.opt_value_from_str("--num").unwrap().unwrap_or(0);

    let name: String = pargs
        .opt_value_from_str("--str")
        .unwrap()
        .unwrap_or_else(|| "None".to_string());

    let vf: bool = pargs.contains("-v");

    println!("Integer argument: {}", nv);
    println!("String argument: {}", name);
    println!("Bool argument: {}", vf);
}
