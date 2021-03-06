use clap::{arg, Command};

fn cli() -> Command<'static> {
    Command::new("cf")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("parse")
                .about("parse problem")
                .arg(arg!(<CID> "contest id"))
                .arg(arg!(<PID> "problem id"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("race")
                .about("race contest")
                .arg(arg!(<CID> "contest id"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("submit").about("Submit Code"))
}

fn main() {
    let matches = cli().get_matches();
    let config = cf_tool::config::Config::from_file(
        dirs::config_dir().unwrap().join("cf-tool-rs/config.toml"),
    );
    let mut client = cf_tool::client::WebClient::new(config);

    match matches.subcommand() {
        Some(("parse", sub_matches)) => {
            let cid = sub_matches
                .value_of("CID")
                .expect("required")
                .parse::<u32>()
                .unwrap();
            let pid = sub_matches
                .value_of("PID")
                .expect("required")
                .parse::<String>()
                .unwrap()
                .to_ascii_lowercase();
            client.login().unwrap();
            cf_tool::util::write_sample(client.parse(cid, &pid).unwrap(), &pid, "./");
        }
        Some(("race", sub_matches)) => {
            let cid = sub_matches
                .value_of("CID")
                .expect("required")
                .parse::<u32>()
                .unwrap();

            client.login().unwrap();
            client.race(cid).unwrap();
        }
        Some(("submit", _)) => {
            let current_path = std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let mut dirs: Vec<&str> = current_path.rsplit('/').collect();
            dirs.reverse();
            let pid = dirs.pop().unwrap();
            let cid = dirs.pop().unwrap().parse::<u32>().unwrap();

            client.login().unwrap();
            client.submit(cid, pid).unwrap();
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
