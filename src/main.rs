// This file is part of hosts2unbound.
//
// hosts2unbound is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// hosts2unbound is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with hosts2unbound.  If not, see <http://www.gnu.org/licenses/>.
use failure::Error;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};
use structopt::StructOpt;

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
    #[structopt(
        short = "h",
        long = "hosts",
        default_value = "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn-social/hosts"
    )]
    hosts_url: String,
}

fn main() {
    let opt = Opt::from_args();
    run(&opt).unwrap();
}

fn run(opt: &Opt) -> Result<(), Error> {
    let mut out = File::create(&opt.output)?;
    let resp = reqwest::blocking::get(&opt.hosts_url)?;
    for line in BufReader::new(resp).lines() {
        let line = line?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }
        if !line.starts_with("0.0.0.0") {
            continue;
        }

        let line = line.split('#').next().unwrap_or("");

        let host = line.split(' ').last().unwrap_or("").trim();
        if host == "" {
            continue;
        }

        out.write(b"local-zone: \"")?;
        out.write(host.as_bytes())?;
        out.write(b"\" static\n")?;
    }
    return Ok(());
}
