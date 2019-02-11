use std::collections::HashMap;
use std::fs::File;
use std::process::{Command, Stdio};

use crate::lib::commands;

use sysinfo::{ProcessExt, SystemExt};

pub fn run(action: &str, process: &str, ls_port: u16) {
    match action {
        "start" => commands::start(process, ls_port),
        "stop" => commands::stop(process),
        "status" => commands::status(process),
        _ => println!("invalid action: {}", action),
    }
}

fn log_cmd(exe: &str, args: &str) {
    println!("Will execute [{} {}]", exe, args);
}

pub fn run_cmd(cmd: &str, args: Vec<&str>) {
    log_cmd(cmd, &args.join(" "));

    let out = File::create(format!("/tmp/{}.log", cmd)).expect("failed to open file");
    let err = out.try_clone().expect("failed to clone file");
    
    let op = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::from(out))    
        .stderr(Stdio::from(err))
        .spawn();

    if op.is_err() {
        eprintln!("failed to execute command, please check if it's in path");
        match op.err() {
            Some(expr) => eprintln!("Error: [{}] {}", cmd, expr),
            None => unreachable!(),
        }
    }
}

pub fn check_status() -> HashMap<String, i32> {
    let mut system = sysinfo::System::new();  // TODO: find any other way to check running processes
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