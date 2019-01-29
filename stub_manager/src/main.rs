use std::process::{Command, Stdio};
use std::fs::File;
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

fn run_cmd(cmd: &str, args: Vec<&str>) {
    let out = File::create(format!("/tmp/{}.log", cmd)).expect("failed to open file");
    let err = out.try_clone().expect("failed to clone file");

    let mut op = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::from(out))
        .stderr(Stdio::from(err))
        .spawn()
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
    let mut proc_map = HashMap::new();

    if process.as_str() == "ls" {
        proc_map.insert("l-handler", vec!["--host", "localhost", "--port", "21000"]);
    } else if process.as_str() == "pk" {
        proc_map.insert("p-handler", vec!["--host", "localhost", "--port", "21001"]);
    } else {
        proc_map.insert("l-handler", vec!["--host", "localhost", "--port", "21000"]);
        proc_map.insert("p-handler", vec!["--host", "localhost", "--port", "21001"]);
    }

    for (p, v) in proc_map.iter() {
        // python3 l-handler --host 0.0.0.0 --port 21001 > /tmp/ls.out 2>&1 &
        // | cmd  | arg-1 |   arg2-3       | arg4-5     | 
        run_cmd(p, v.to_vec());
    }    
}

fn stop(process: String) {
    let running_proc = check_status();

    if running_proc.len() == 0 {
        println!("no stubs are running locally!");
        return;
    } 

    let mut proc_map = HashMap::new();

    for (k, v) in running_proc.iter() {
        if k.contains("l-handler") {
            proc_map.insert("ls", v);         
        } else if k.contains("p-handler") {
            proc_map.insert("pk", v);
        }
    }

    if process.as_str() == "ls" {
        if proc_map.get("ls").is_some() {
            run_cmd("kill", vec![
                    proc_map["ls"],
                ]
            );
        }
    } else if process.as_str() == "pk" {
        if proc_map.get("pk").is_some() {
            run_cmd("kill", vec![proc_map["pk"]]);
        }
    } else {
        if proc_map.get("ls").is_some() {
            run_cmd("kill", vec![proc_map["ls"]]);
        }
        if proc_map.get("pk").is_some() {
            run_cmd("kill", vec![proc_map["pk")]]);
        }
    }

    // for (p, v) in proc_map.iter() {
    //     // python3 l-handler --host 0.0.0.0 --port 21001 > /tmp/ls.out 2>&1 &
    //     // | cmd  | arg-1 |   arg2-3       | arg4-5     | 
    //     run_cmd(p, v.to_vec());
    // }
    // run_cmd(cmd, false);
}

fn main() {
    let (action, process) = parse_and_validate();
    println!("action : {}, process: {}", action, process);
    run(action, process);
}
