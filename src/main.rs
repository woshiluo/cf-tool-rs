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
}

fn main() {
    let matches = cli().get_matches();
    let mut client = cf_tool::client::WebClient::new();

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
            println!("Parsing {} {}", cid, pid);
            cf_tool::util::write_sample(client.parse(cid, &pid).unwrap(), &pid, format!("./"));
        }
        Some(("race", sub_matches)) => {
            let cid = sub_matches
                .value_of("CID")
                .expect("required")
                .parse::<u32>()
                .unwrap();

            client.race(cid).unwrap();
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
