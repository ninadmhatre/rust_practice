use std::collections::HashMap;
use std::fs::File;
use std::process::{Command, Stdio};

// use lib;

use sysinfo::{ProcessExt, SystemExt};

pub fn run(action: String, process: String) {
    match action.as_str() {
        "start" => super::commands::start(process),
        "stop" => super::commands::stop(process),
        "status" => super::commands::status(process),
        _ => println!("invalid action: {}", action),
    }
}

pub fn run_cmd(cmd: &str, args: Vec<&str>) {
    let out = File::create(format!("/tmp/{}.log", cmd)).expect("failed to open file");
    let err = out.try_clone().expect("failed to clone file");

    // TOOD: figure out way to log the command executed!
    let op = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::from(out))    # TOFO: Out put is still buffered!
        .stderr(Stdio::from(err))
        .spawn();

    if op.is_err() {
        println!("failed to execute command");
        println!("Error: {:?}", op.err());  // TODO: Only print actual error message
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
