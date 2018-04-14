#[macro_use]
extern crate structopt;
extern crate reqwest;
extern crate failure;

use failure::Error;
use std::path::PathBuf;
use std::io::{BufReader, BufRead};
use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "hosts2unbound")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,

    /// Override hosts source
    #[structopt(short = "h", long = "hosts", default_value = "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn-social/hosts")]
    hosts_url: String,
}

fn main() {
    let opt = Opt::from_args();
    run(&opt).unwrap();
}

fn run(opt: &Opt) -> Result<(), Error> {
    let mut out = File::create(&opt.output)?;
    let resp = reqwest::get(&opt.hosts_url)?;
    for line in BufReader::new(resp).lines() {
        let line = line?;
        line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }
        if !line.starts_with("0.0.0.0") {
            continue;
        }

        let host = line.split(' ').last().unwrap_or("");
        host.trim();
        if host == "" {
            continue;
        }

        out.write(b"local-zone: \"")?;
        out.write(host.as_bytes())?;
        out.write(b"\" static\n")?;
    }
    return Ok(());
}
