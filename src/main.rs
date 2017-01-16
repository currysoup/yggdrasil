#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate serial;
extern crate rustc_serialize;

use std::io::Read;
use std::str;
use std::time::Duration;

use serial::prelude::*;

#[macro_use]
mod logging;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud9600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

docopt!(Args derive Debug, "
yggdrasil

The world tree withers.

Usage:
    yggdrasil [options] <port>
    yggdrasil (--help | --version)
Options:
    -h --help         Show this screen.
    -V --version      Show version information.
");

fn get_version() -> Option<String> {
    let (maj, min, pat) = (option_env!("CARGO_PKG_VERSION_MAJOR"),
                           option_env!("CARGO_PKG_VERSION_MINOR"),
                           option_env!("CARGO_PKG_VERSION_PATCH"));
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => Some(format!("yggdrasil {}.{}.{}", maj, min, pat)),
        _ => None,
    }
}

fn main() {
    let args: Args = Args::docopt()
        .options_first(true)
        .version(get_version())
        .decode()
        .unwrap_or_else(|e| e.exit());

    let mut port = match serial::open(&args.arg_port) {
        Ok(p) => p,
        Err(e) => {
            println_stderr!("Failed to connect on \"{}\": {}", &args.arg_port, e);
            return;
        },
    };
    port.configure(&SETTINGS);
    port.set_timeout(Duration::from_secs(5));

    let mut buf = vec![0; 4];

    println!("Reading some bytes");

    loop {
        port.read(&mut buf[..]).unwrap();
        let num = unsafe { std::mem::transmute::<[u8; 4], i32>(buf[0..4]) };
        println!("{:?}", num);
    }
}
