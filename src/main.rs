use std::net::IpAddr;

use clap::{arg, command, Parser, ValueEnum};
use ipnet::IpNet;

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

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormats::Plain)]
    output: OutputFormats,
}

fn calculate_all_ips(cidr: &str, all: bool) -> Result<Vec<IpAddr>, String> {
    let ip_net = cidr
        .parse::<IpNet>()
        .map_err(|e| format!("'{cidr}' is an invalid CIDR: {e}"))?;

    let ips = match ip_net {
        IpNet::V4(ipv4_net) => {
            if all {
                let mut ips = vec![];
                for host in ipv4_net.hosts() {
                    ips.push(IpAddr::V4(host));
                }

                ips
            } else {
                let mut hosts = ipv4_net.hosts();

                match (hosts.next(), hosts.last()) {
                    (None, None) => vec![],
                    (None, Some(last)) => vec![IpAddr::from(last)],
                    (Some(first), None) => vec![IpAddr::from(first)],
                    (Some(first), Some(last)) => vec![IpAddr::from(first), IpAddr::from(last)],
                }
            }
        }
        IpNet::V6(_) => vec![],
    };

    Ok(ips)
}

fn main() {
    let cli = Cli::parse();

    let cidr = &cli.cidr;

    match calculate_all_ips(cidr, cli.all) {
        Ok(ips) => match cli.output {
            OutputFormats::Csv => {
                println!("\"ips\"");
                for ip in ips {
                    println!("\"{ip}\"");
                }
            }
            OutputFormats::Json => {
                print!("[");

                ips.iter()
                    .take(ips.len() - 1)
                    .for_each(|ip| print!("\"{ip}\","));

                if let Some(last_ip) = ips.last() {
                    print!("\"{last_ip}\"");
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
