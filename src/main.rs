use clap::{arg, Command};

fn cli() -> Command<'static> {
    Command::new("git")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("parse")
                .about("parse problem")
                .arg(arg!(<CID> "contest id"))
                .arg(arg!(<PID> "problem id"))
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
                .parse::<char>()
                .unwrap()
                .to_ascii_lowercase();
            println!("Parsing {} {}", cid, pid);
            let (inputs, outputs) = client.parse(cid, pid).unwrap();

            let size = inputs.len();
            for i in 1..=size {
                use std::io::Write;
                {
                    let mut file = std::fs::File::create(format!("{}{}.in", pid, i)).unwrap();
                    file.write_all(inputs[i - 1].as_ref()).unwrap();
                }
                {
                    let mut file = std::fs::File::create(format!("{}{}.ans", pid, i)).unwrap();
                    file.write_all(outputs[i - 1].as_ref()).unwrap();
                }
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
