pub mod ip;

use std::io::Read;

use clap::{crate_description, crate_name, crate_version, value_parser, Arg, ArgAction, Command};
use ip::IPAddress;

#[derive(clap::ValueEnum, Clone)]
enum Types {
    Dec = 0,
    Hex = 1,
    Oct = 2,
    All = 3,
}

fn main() {
    let matches = Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .help_template("{before-help}\n{about}\n\n{usage-heading} {usage}\n\n{all-args}")
        .arg(
            Arg::new("ip")
                .help("IPs to convert.")
                .action(ArgAction::Append)
                .value_parser(value_parser!(String))
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Choose output format.")
                .action(ArgAction::Set)
                .value_parser(clap::builder::EnumValueParser::<Types>::new())
                .required(false),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .help("Reduce output (will not print input).")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("full")
                .long("full")
                .short('f')
                .help("Display hex/octal as separate bytes (e.g. 0x7f.0x0.0x0.0x1).")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let format = matches.get_one::<Types>("output").unwrap_or(&Types::Dec);
    let full: bool = *matches.get_one("full").unwrap();

    let mut args: Vec<String>;
    let mut buffer = String::new();
    let mut input: bool = false;

    if let Some(args) = matches.get_many::<String>("ip") {
        for arg in args {
            if arg.eq("-") {
                input = true;
                break;
            }
        }
    }

    if input {
        std::io::stdin().read_to_string(&mut buffer).unwrap();

        args = buffer
            .split("\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.replace(" ", ""))
            .collect();
        args.retain(|arg| !arg.is_empty());
    } else {
        args = matches
            .get_many::<String>("ip")
            .unwrap()
            .map(|arg| arg.as_str())
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.replace(" ", ""))
            .collect();
    }

    for arg in args {
        let ip_type = match arg.parse::<u64>() {
            Ok(parsed) => IPAddress::try_from(parsed),
            Err(_) => IPAddress::try_from(arg.to_owned()),
        };

        let ip: IPAddress;

        match ip_type {
            Ok(ip_address) => {
                ip = ip_address;
            }

            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        }

        let result = match format {
            Types::Dec => ip.to_dec().to_string(),
            Types::Hex => ip.to_hex(full).to_string(),
            Types::Oct => ip.to_oct(full).to_string(),
            Types::All => {
                format!("{} {} {}", ip.to_dec(), ip.to_hex(full), ip.to_oct(full))
            }
        };

        if matches.get_flag("quiet") {
            println!("{}", format!("{}", result));
        } else {
            println!("{}", format!("{} {}", ip.ip, result));
        }
    }
}
