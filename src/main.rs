#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate byteorder;
extern crate docopt;
extern crate serial;
extern crate rustc_serialize;

use comms::Comms;

mod comms;
#[macro_use]
mod logging;

docopt!(Args derive Debug, "
yggdrasil

The world tree withers.

Usage:
    yggdrasil [options] <port>
    yggdrasil (--help | --version)
Options:
    -p --port=PORT    Set to a nonstandard port [default: 8508]
    -h --help         Show this screen.
    -V --version      Show version information.
", flag_port: u16);

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
    let mut serial = comms::serial::SerialComms::new(&args.arg_port).unwrap();

    loop {
        let id = comms::PlantId(4);
        let mlevel = serial.req_moisture(id);

        println!("{:?}", mlevel);
        std::thread::sleep_ms(1000);
    }
}
