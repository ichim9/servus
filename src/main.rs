use clap::{Command,ArgMatches};
mod wrapper;
mod server;
use server::execute_shell;
use wrapper::*;

fn main() {
    let matches = command().get_matches();
    eval_matches(matches);
}

fn command() -> Command{
    Command::new("servus")
        .about("Minecraft server manager written in rust 🌲")
        .subcommand_required(true)
        .subcommand(wrapper::inject_cmd(Command::new("new")
            .about("Initialize a new server in CWD.")
            .arg_required_else_help(true))
        )
        .subcommand(Command::new("run")
            .about("Starts the server.")
        )
}

fn eval_matches(matches:ArgMatches) {
    let os = std::env::consts::OS;
    match matches.subcommand(){
        Some(("new",args))=>{inject_match(args);}
        Some(("run",_))=>{
            if os == "windows"{
                execute_shell(false, vec![
                    ("cmd",vec![
                        "/C","start.bat"
                    ])
                ]);
            }else{
                execute_shell(false, vec![
                    ("bash",vec![
                        "start.sh"
                    ])
                ]);
            }
        }
        _=>{}
    }
}
