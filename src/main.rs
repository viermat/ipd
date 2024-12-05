use clap::{value_parser, Arg, ArgAction, Command};
use std::{net::Ipv4Addr, str::FromStr};

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .about("CLI tool for converting IP addresses to decimal format")
        .help_template("{before-help}{about}\n{usage-heading} {usage}\n\n{all-args}")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("ip")
                .help("IPs to convert")
                .action(ArgAction::Append)
                .value_parser(value_parser!(String))
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("clean")
                .short('c')
                .help("Set to lowest verbosity level")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    if let Some(files) = matches.get_many::<String>("ip") {
        for ip in files {
            let ip_addr: Ipv4Addr = match Ipv4Addr::from_str(&ip) {
                Ok(ip) => ip,
                Err(_) => {
                    continue;
                }
            };

            let octets: [u8; 4] = ip_addr.octets();
            let mut ip_addr_dec: u64 = 0;

            for _dec in octets.iter().enumerate() {
                ip_addr_dec += (*_dec.1 as u64) * 256_u64.pow([3, 2, 1, 0][_dec.0 as usize]);
            }

            match matches.get_flag("clean") {
                true => println!("{}", ip_addr_dec),
                false => println!("{} {}", ip_addr, ip_addr_dec),
            }
        }
    }
}
