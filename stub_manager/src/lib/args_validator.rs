use clap::{App, Arg};

pub fn parse_and_validate() -> (String, String, u16) {
    let matches = App::new("stub_manager")
        .about("manage locally running stubs")
        .version("1.1")
        .author("Ninad Mhatre")
        .arg(
            Arg::with_name("action")
                .help("what action to perform?")
                .index(1)
                .required(true)
                .possible_values(&["start", "stop", "status"])
        )
        .arg(
            Arg::with_name("process")
                .help("on which stub above action needs to be performed?")
                .index(2)
                .possible_values(&["ls", "pk", "all"])
                .default_value("all")
        )
        .arg(
            Arg::with_name("port")
                .help("choose LS port, PK port will be LS+1")
                .index(3)
                .default_value("21000")
                .validator(validate_port)
        )
        .get_matches();

    let action = matches.value_of("action").unwrap();
    let process = matches.value_of("process").unwrap();
    let port = matches.value_of("port").unwrap();

    let port_int = match port.parse::<u16>() {
        Ok(v) => v,
        Err(_) => unreachable!()
    };

    (action.to_string(), process.to_string(), port_int)
}

fn validate_port(val: String) -> Result<(), String> {
    match val.parse::<i32>() {
        Ok(v) => {
          if ( 4000 < v) && (v < 65430 ) {
            Ok(())
          } else {
              Err(String::from("port must be between 4000 - 65364"))
          }  
        },
        _ => Err(format!("failed to parse port! {}", val))
    }
}