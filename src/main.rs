use clap::Parser;
use cli::{Cli, OutputFormats};
use ips::calculate_all_ips;
use std::iter::Iterator;

mod cli;
mod ips;

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
