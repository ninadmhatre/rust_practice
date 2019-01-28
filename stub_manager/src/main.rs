use std::process::Command;
use std::collections::HashMap;

use clap::{App, Arg};
use sysinfo::{ProcessExt, SystemExt};

fn parse_and_validate() -> (String, String) {
    let matches = App::new("stub_manager")
        .about("manage locally running stubs")
        .version("1.0")
        .author("Ninad Mhatre")
        .arg(
            Arg::with_name("action")
                .help("action can be <start|stop|status>")
                .index(1)
                .required(true)
                .validator(validate_action),
        )
        .arg(
            Arg::with_name("process")
                .help("which stub process to manage? <ls|pk|all>")
                .index(2)
                .required(true)
                .validator(validate_process),
        )
        .get_matches();

    let action = matches.value_of("action").unwrap();
    let process = matches.value_of("process").unwrap();

    (action.to_string(), process.to_string())
}

fn validate_action(val: String) -> Result<(), String> {
    match val.as_str() {
        "start" | "stop" | "status" => Ok(()),
        _ => Err(String::from("action must be <start|stop|status>")),
    }
}

fn validate_process(val: String) -> Result<(), String> {
    match val.as_str() {
        "ls" | "pk" | "all" => Ok(()),
        _ => Err(String::from("process must be <ls|pk|all>")),
    }
}

fn run(action: String, process: String) {
    match action.as_str() {
        "start" => start(process),
        "stop" => stop(process),
        "status" => status(process),
        _ => println!("invalid action: {}", action),
    }
}

fn run_cmd(cmd: String, args: Vec[<String>]) {
    if !dry {
        dry = false;
    }

    let mut op = Command::new(cmd)
        .args(&args)
        .output()
        .expect("failed to run command");

    dbg!(op);
}

fn check_status() -> HashMap<String, i32> {
    let mut system = sysinfo::System::new();
    let mut processes = HashMap::new();

    system.refresh_all();

    for (pid, proc_) in system.get_process_list() {
        if proc_.name().starts_with("python3") && proc_.cmd()[1].ends_with("-handler") {
            let abs_cmd = proc_.cmd().join(" ");
            processes.insert(abs_cmd, *pid);
        }
    }

    processes
}

fn cmd_generator(prefix: String, process: String) -> String {
    let what = &format!("{}_{}", prefix, process);
    what.to_string()
}

fn status(_proc: String)  {

    let processes = check_status();

    if processes.len() == 0 {
        println!("no ls/pk stubs are running!");
        return 
    }

    for (k, v) in processes.iter() {
        println!("{:15} ... {:5}", k, v);
    }
}

fn start(process: String) {
    // let cmd = cmd_generator("start".to_string(), process);
    let mut p_list = Vec[<String>];

    match process.as_str() {
        "ls" => p_list.push("l-handler"),
        "pk" => p_list.push("p-handler"),
        "all" => p_list.push("l-handler", "p-handler"),
        _ => p_list.push("---")
    }

    p_list.push("--host 0.0.0.0");
    p_list.push("--port 2479");

    for p in p_list.iter() {
        run_cmd("python3", p_list);
    }
    
}

fn stop(process: String) {
    let cmd = cmd_generator("stop".to_string(), process);
    run_cmd(cmd, false);
}

fn main() {
    let (action, process) = parse_and_validate();
    println!("action : {}, process: {}", action, process);
    run(action, process);
}
