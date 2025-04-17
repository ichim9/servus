use std::collections::HashMap;
use maplit::hashmap;
use crate::server::*;

pub fn inject_cmd(mut cmd:clap::Command) -> clap::Command{
    let server_types: HashMap<&str,(&str,HashMap<&str,bool>)> = hashmap! {
        "fabric" => ("fabricmc.net",hashmap!{
            "game_version" => false,
            "loader_version" => true,
            "installer_version" => true
        }),
        "quilt" => ("quiltmc.org",hashmap!{
            "game_version" => false,
        }),
        "purpur" => ("purpurmc.org",hashmap!{
            "game_version" => false,
            "build_edition" => false
        })
    };

    for (k,v) in server_types{
        let mut summary: Vec<clap::Arg> = Vec::new();
        for (sk,sv) in v.1{
            let per = clap::Arg::new(sk);
            summary.push(option_if_bool(per,sv))
        }
        cmd = cmd.subcommand(clap::Command::new(k).about(v.0).arg_required_else_help(true).args(summary));
    }
    cmd
}

pub fn inject_match(arg:&clap::ArgMatches){
    match arg.subcommand(){
        Some(("fabric",args)) => {
            fabric_install(args);
        }
        Some(("quilt",args)) =>{
            quilt_install(args);
        }
        Some(("purpur",args)) =>{
            purpur_install(args)
        }
        _=>{}
    }
}

fn option_if_bool(arg:clap::Arg,v:bool) -> clap::Arg{
    if v{
        return arg.default_missing_value("")
    }
    return arg
}
