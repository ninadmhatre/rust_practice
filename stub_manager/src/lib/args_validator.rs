use clap::{App, Arg};

pub fn parse_and_validate() -> (String, String) {
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
