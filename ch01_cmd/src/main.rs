mod cmd;

use crate::cmd::{parse_cmd, Cmd};

fn main() {

    let cmd = parse_cmd();
    // dbg!(&cmd);

    if cmd.version_flag {
        print!("{}", "version 0.0.1");
    }else if cmd.help_flag  || cmd.class == "" {
        cmd.print_usage();
    }else{
        start_jvm(cmd);
    }
    
}

fn start_jvm(cmd: Cmd) {
    println!("classpath: {} class: {} args: {:?}", cmd.cp_option, cmd.class, cmd.args);
}
