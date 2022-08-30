use libubootenv_rs::UBootContext;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: \"ubootenv-rs get <name>\" or \"ubootenv-rs set <name> <value>\"");
        return;
    }
    let cfg = "/etc/fw_env.config";
    let mut e = UBootContext::initialize().unwrap();
    e.read_config(cfg).unwrap();
    e.open().unwrap();
    if args[1] == "get" {
        assert!(args.len() == 3, "Format: get <name>");
        let name = &args[2];
        println!("{}", e.get_env(name.as_str()).unwrap());
    } else if args[1] == "set" {
        assert!(args.len() == 4, "Format: set <name> <value>");
        let name = &args[2];
        let val = &args[3];
        e.set_env(name.as_str(), val.as_str()).unwrap();
        e.env_store().unwrap();
    } else {
        panic!("Invalid command");
    }
}