use std::collections::HashMap;

// mod lib::executor;

pub fn status(_proc: String) {
    let processes = super::executor::check_status();

    if processes.len() == 0 {
        println!("no ls/pk stubs are running!");
        return;
    }

    for (k, v) in processes.iter() {
        println!("{:15} ... {:5}", k, v);
    }
}

fn is_proc_running(proc: String) {
    
}

pub fn start(process: String) {
    let mut proc_map = HashMap::new();

    if process.as_str() == "ls" {
        proc_map.insert(super::config::LS_HNDL, vec!["--host", "localhost", "--port", super::config::L_PORT]);
    } else if process.as_str() == "pk" {
        proc_map.insert(super::config::PK_HNDL, vec!["--host", "localhost", "--port", super::config::P_PORT]);
    } else {
        proc_map.insert(super::config::LS_HNDL, vec!["--host", "localhost", "--port", super::config::L_PORT]);
        proc_map.insert(super::config::PK_HNDL, vec!["--host", "localhost", "--port", super::config::P_PORT]);
    }

    for (p, v) in proc_map.iter() {
        super::executor::run_cmd(p, v.to_vec());
    }
}

pub fn stop(process: String) {
    let running_proc = super::executor::check_status();

    if running_proc.len() == 0 {
        println!("no stubs are running locally!");
        return;
    }

    let mut proc_map = HashMap::new();

    for (k, v) in running_proc.iter() {
        if k.contains(super::config::LS_HNDL) {
            proc_map.insert("ls", v);
        } else if k.contains(super::config::PK_HNDL) {
            proc_map.insert("pk", v);
        }
    }

    if process.as_str() == "ls" {
        if proc_map.get("ls").is_some() {
            super::executor::run_cmd("kill", vec![&proc_map["ls"].to_string()]);
        }
    } else if process.as_str() == "pk" {
        if proc_map.get("pk").is_some() {
            super::executor::run_cmd("kill", vec![&proc_map["pk"].to_string()]);
        }
    } else {
        if proc_map.get("ls").is_some() {
            super::executor::run_cmd("kill", vec![&proc_map["ls"].to_string()]);
        }
        if proc_map.get("pk").is_some() {
            super::executor::run_cmd("kill", vec![&proc_map["pk"].to_string()]);
        }
    }
}
