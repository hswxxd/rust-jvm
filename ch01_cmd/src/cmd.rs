use std::env;
use getopts::{Options, ParsingStyle};

#[derive(Debug)]
pub struct Cmd{
    pub help_flag: bool,
    pub version_flag: bool,
    pub cp_option: String,
    pub class: String,
    pub args: Vec<String>

}

impl Cmd{
    pub fn print_usage(&self) {
        let args: Vec<String> = env::args().collect();
        println!("Usage: {} [-options] class [args...]", args[0])
    }
}

pub fn parse_cmd() -> Cmd{
    let mut cmd = Cmd{
        help_flag: false,
        version_flag: false,
        cp_option: "".to_string(),
        class: "".to_string(),
        args: vec![],
    };

    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();

    let mut opts = Options::new();
    let opts = opts.parsing_style(ParsingStyle::StopAtFirstFree).long_only(true);
    opts.optflag("h", "help", "Print help message");
    opts.optflag("", "version", "Print version and exit");
    opts.optopt("", "classpath", "Specify the classpath", "classpath");
    opts.optopt("", "cp", "Specify the classpath", "classpath");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print_usage(&program, opts);
            panic!("{}", f.to_string());
        }
    };

    if matches.opt_present("help"){
        cmd.help_flag = true;
    }

    if matches.opt_present("version") {
        cmd.version_flag = true;
    }

    match matches.opt_str("classpath") {
        Some(classpath) => {
            cmd.cp_option = classpath;
        },
        None => {
            match matches.opt_str("cp") {
                Some(cp) => {
                    cmd.cp_option = cp;
                }
                None => {}
            }
        }
    }

    if !matches.free.is_empty() {
        cmd.class = matches.free[0].clone();
        cmd.args = matches.free[1..].to_vec();
    }

    cmd


}

fn print_usage(program: &str, opts: &mut Options){
    let brief = format!("Usage: {} [-options] class [args...]", program);
    println!("{}", opts.usage(&brief));
}