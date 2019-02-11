use std::collections::HashMap;

use crate::lib::executor::{check_status, run_cmd};
use crate::lib::config;

pub fn status(_proc: &str) {
    let processes = check_status();

    if processes.is_empty() {
        println!("no ls/pk stubs are running!");
        return;
    }

    for (k, v) in processes.iter() {
        println!("{:15} ... {:5}", k, v);
    }
}

pub fn start(process: &str, ls_port: u16) {
    let mut proc_map = HashMap::new();

    let ls_port_str = ls_port.to_string();
    let pk_port_str = (ls_port + 1).to_string();
    let host = "0.0.0.0";

    if process == "ls" {
        proc_map.insert(config::LS_HNDL, vec!["--host", host, "--port", &ls_port_str]);
    } else if process == "pk" {
        proc_map.insert(config::PK_HNDL, vec!["--host", host, "--port", &pk_port_str]);
    } else {
        proc_map.insert(config::LS_HNDL, vec!["--host", host, "--port", &ls_port_str]);
        proc_map.insert(config::PK_HNDL, vec!["--host", host, "--port", &pk_port_str]);
    }

    for (p, v) in proc_map.iter() {
        run_cmd(p, v.to_vec());
    }
}

pub fn stop(process: &str) {
    let running_proc = check_status();

    if running_proc.is_empty() {
        println!("no stubs are running locally!");
        return;
    }

    let mut proc_map = HashMap::new();

    for (k, v) in running_proc.iter() {
        if k.contains(config::LS_HNDL) {
            proc_map.insert("ls", v);
        } else if k.contains(config::PK_HNDL) {
            proc_map.insert("pk", v);
        }
    }

    if process == "ls" {
        if proc_map.get("ls").is_some() {
            run_cmd("kill", vec![&proc_map["ls"].to_string()]);
        }
    } else if process == "pk" {
        if proc_map.get("pk").is_some() {
            run_cmd("kill", vec![&proc_map["pk"].to_string()]);
        }
    } else {
        if proc_map.get("ls").is_some() {
            run_cmd("kill", vec![&proc_map["ls"].to_string()]);
        }
        if proc_map.get("pk").is_some() {
            run_cmd("kill", vec![&proc_map["pk"].to_string()]);
        }
    }
}
