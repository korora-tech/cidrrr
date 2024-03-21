use std::net::IpAddr;

use clap::{arg, command, Parser, ValueEnum};
use ipnet::IpNet;
use std::iter::Iterator;

// show only that many addresses, unless "--danger-zone" is set
const HARD_LIMIT: usize = 1_048_576;

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormats {
    Csv,
    Json,
    Plain,
}

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    /// The CIDR block to check
    cidr: String,

    /// Show all IPs in CIDR block - if set to false, we only show the first and the last ip in the block
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Additional toggle which has to be enabled - if set to false, we only show 2^20 (= 1.048.576) addresses
    #[arg(long, default_value_t = false)]
    danger_zone: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormats::Plain)]
    output: OutputFormats,
}

fn calculate_all_ips(
    cidr: &str,
    all: bool,
    danger_zone: bool,
) -> Result<impl Iterator<Item = IpAddr>, String> {
    let ip_net = cidr
        .parse::<IpNet>()
        .map_err(|e| format!("'{cidr}' is an invalid CIDR: {e}"))?;

    let ips: Box<dyn Iterator<Item = IpAddr>> = if all && danger_zone {
        Box::new(ip_net.hosts())
    } else if all {
        Box::new(ip_net.hosts().take(HARD_LIMIT))
    } else {
        let mut hosts = ip_net.hosts();

        Box::new(hosts.next().into_iter().chain(hosts.last()))
    };

    Ok(ips)
}

fn main() {
    let cli = Cli::parse();

    let cidr = &cli.cidr;

    match calculate_all_ips(cidr, cli.all, cli.danger_zone) {
        Ok(ips) => match cli.output {
            OutputFormats::Csv => {
                println!("\"ips\"");
                for ip in ips {
                    println!("\"{ip}\"");
                }
            }
            OutputFormats::Json => {
                print!("[");

                let mut peekable_ips = ips.peekable();

                while let Some(elem) = peekable_ips.next() {
                    if peekable_ips.peek().is_none() {
                        print!("\"{elem}\"");
                    } else {
                        print!("\"{elem}\",");
                    }
                }

                println!("]");
            }
            OutputFormats::Plain => {
                for ip in ips {
                    println!("{ip}");
                }
            }
        },
        Err(err) => eprintln!("Error: {err}"),
    }
}
