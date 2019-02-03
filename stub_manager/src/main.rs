// Name      : stub_handler
// 
// Usage     : stub_handler action:<start|stop|status> process:<ls|pk|all>
//             action   : what action to do?
//             process  : name of process or all
// 
// ** Totally un-necessary port of shell script to Rust (just for learning) **

mod lib;

fn main() {
    let (action, process) = lib::args_validator::parse_and_validate();
    lib::executor::run(action, process);
}
