use std::fs;
use std::io::Write;
use std::process::{Command, Output};

use clap::ArgMatches;
use reqwest::blocking::get;

pub fn fabric_install(arg:&ArgMatches){
    let div = &"1.0.3".to_string();
    let dlv = &"0.16.13".to_string();
    let game_version = arg.get_one::<String>("game_version").unwrap();
    let loader_version = arg.get_one::<String>("loader_version").unwrap_or(&dlv);
    let installer_version = arg.get_one::<String>("installer_version").unwrap_or(&div);
    let response = get(format!("https://meta.fabricmc.net/v2/versions/loader/{}/{}/{}/server/jar",game_version,loader_version,installer_version));
    let buffer_file = std::fs::File::create("fabric-server-launch.jar");
    buffer_file.unwrap().write_all(&response.unwrap().bytes().unwrap()).unwrap();
    let read = fs::read_to_string("fabric-server-launch.jar");
    match read{
        Ok(_)=>{
            println!("Error downloading jar.");
            fs::remove_file("fabric-server-launch.jar").unwrap();
            return
        }
        Err(_)=>{}
    }
    // REUSABLE SEGMENT
    fs::write("start.sh","#!/usr/bin/env bash\njava -Xmx2G -jar fabric-server-launch.jar nogui").unwrap();
    fs::write("start.bat","java -Xmx2G -jar fabric-server-launch.jar nogui\npause").unwrap();
    execute_shell(false,vec![
        ("java",vec!["-jar","fabric-server-launch.jar"]),
        ("chmod",vec!["+x","start.sh"]),
        ("chmod",vec!["+x","start.bat"])
    ]);
}

pub fn quilt_install(arg:&ArgMatches){
    let game_version: &String = arg.get_one("game_version").unwrap();
    let forwarder = get("https://quiltmc.org/api/v1/download-latest-installer/java-universal");
    let installer_file = std::fs::File::create("installer.jar");
    installer_file.unwrap().write_all(&forwarder.unwrap().bytes().unwrap()).unwrap();
    let installer_attempt = execute_shell(true,vec![
        ("java",vec!["-jar","installer.jar","install","server",game_version,"--download-server"])
    ]);
    if String::from_utf8(installer_attempt.unwrap()[0].stderr.clone()).unwrap() != "".to_string(){
        println!("Error installing jar.");
        fs::remove_file("installer.jar").unwrap();
    }else{
        fs::remove_file("installer.jar").unwrap();
        fs::rename("server/libraries","libraries").unwrap();
        fs::rename("server/server.jar","server.jar").unwrap();
        fs::rename("server/quilt-server-launch.jar","quilt-server-launch.jar").unwrap();
        fs::remove_dir("server").unwrap();
        // REUSABLE SEGMENT
        fs::write("start.sh","#!/usr/bin/env bash\njava -Xmx2G -jar quilt-server-launch.jar nogui").unwrap();
        fs::write("start.bat","java -Xmx2G -jar quilt-server-launch.jar nogui\npause").unwrap();
        execute_shell(false,vec![
            ("java",vec!["-jar","quilt-server-launch.jar"]),
            ("chmod",vec!["+x","start.sh"]),
            ("chmod",vec!["+x","start.bat"])
        ]);
    }
}

pub fn purpur_install(arg:&ArgMatches){
    let game_version: &String = arg.get_one("game_version").unwrap();
    let build_edition: &String = arg.get_one("build_edition").unwrap();
    let installer = get(format!("https://api.purpurmc.org/v2/purpur/{game_version}/{build_edition}/download"));
    let buffer_file = std::fs::File::create("purpur.jar");
    buffer_file.unwrap().write_all(&installer.unwrap().bytes().unwrap()).unwrap();
    match fs::read_to_string("purpur.jar"){
        Ok(_)=>{
            println!("Error downloading jar.");
            fs::remove_file("purpur.jar").unwrap();
            return
        }
        Err(_)=>{}
    }
    fs::write("start.sh","#!/usr/bin/env bash\njava -Xmx2G -jar purpur.jar nogui").unwrap();
    fs::write("start.bat","java -Xmx2G -jar purpur.jar nogui\npause").unwrap();
    execute_shell(false,vec![
        ("java",vec!["-jar","purpur.jar"]),
        ("chmod",vec!["+x","start.sh"]),
        ("chmod",vec!["+x","start.bat"])
    ]);
}
pub fn paper_install(arg:&ArgMatches){
    let game_version: &String = arg.get_one("game_version").unwrap();
    let build_edition: &String = arg.get_one("build_edition").unwrap();
    let installer = get(format!("https://api.papermc.io/v2/projects/paper/versions/{game_version}/builds/{build_edition}/downloads/paper-{game_version}-{build_edition}.jar"));
    let buffer_file = std::fs::File::create("paper.jar");
    buffer_file.unwrap().write_all(&installer.unwrap().bytes().unwrap()).unwrap();
    match fs::read_to_string("paper.jar"){
        Ok(_)=>{
            println!("Error downloading jar.");
            fs::remove_file("paper.jar").unwrap();
            return
        }
        Err(_)=>{}
    }
    fs::write("start.sh","#!/usr/bin/env bash\njava -Xmx2G -jar paper.jar nogui").unwrap();
    fs::write("start.bat","java -Xmx2G -jar paper.jar nogui\npause").unwrap();
    execute_shell(false,vec![
        ("java",vec!["-jar","paper.jar"]),
        ("chmod",vec!["+x","start.sh"]),
        ("chmod",vec!["+x","start.bat"])
    ]);
}

pub fn execute_shell(capture_output:bool,cmds:Vec<(&str,Vec<&str>)>) -> Option<Vec<Output>>{
    if capture_output{
        let mut summary: Vec<Output> = Vec::new();
        for i in cmds{
            summary.push(Command::new(i.0).args(i.1).output().unwrap());
        }
        Option::Some(summary)
    }else{
        for i in cmds{
            Command::new(i.0).args(i.1).spawn().unwrap().wait().unwrap();
        }
        Option::None
    }
}
