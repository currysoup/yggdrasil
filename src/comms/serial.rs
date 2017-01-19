use std::ffi::OsString;
use std::io::{BufRead, Cursor, Read, Write};
use std::mem;
use std::str;
use std::time::Duration;

use bufstream::BufStream;
use byteorder::{LittleEndian, ReadBytesExt};
use rustc_serialize::json;
use serial;
use serial::prelude::*;

use comms::{Comms, MoistureLevel, PlantId};

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud57600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

// Arduinos use 16 bit integers
type DeviceInt = i16;

// TODO: Seriously? No constexpr yet?
const SIZE_OF_REQUEST: usize = 6;
const SIZE_OF_RESPONSE: usize = 6 + RESPONSE_MAX_LEN;
const RESPONSE_MAX_LEN: usize = 256;

#[repr(packed)]
#[derive(Clone)]
struct SerialRequest {
    seq: DeviceInt,
    req_type: DeviceInt,
    plant_id: DeviceInt,
}

#[repr(packed)]
#[derive(Debug, RustcDecodable)]
struct SerialResponse {
    seq: DeviceInt,
    plant_id: DeviceInt,
    text: String,
}

impl SerialResponse {
    fn to_moisture_level(self) -> MoistureLevel {
        MoistureLevel {
            plant_id: PlantId(self.plant_id),
            level: i16::from_str_radix(&self.text, 10).unwrap(),
        }
    }
}

pub struct SerialComms {
    stream: BufStream<serial::SystemPort>,
    seq: DeviceInt,
}

impl SerialComms {
    pub fn new(port: &str) -> serial::Result<SerialComms> {
        let mut port = serial::open(port)?;
        port.configure(&SETTINGS)?;
        port.set_timeout(Duration::from_secs(2))?;

        Ok(SerialComms {
            stream: BufStream::new(port),
            seq: 1,
        })
    }

    fn send_request(&mut self, req: SerialRequest) -> SerialResponse {
        let text = format!("{} {} {}\n", req.seq, req.req_type, req.plant_id);
        self.stream.write_all(text.as_bytes()).unwrap();
        self.stream.flush().unwrap();

        let mut buf = String::new();
        self.stream.read_line(&mut buf);
        json::decode(&buf).unwrap()
    }
}

impl Comms for SerialComms {
    fn req_moisture(&mut self, plant_id: PlantId) -> MoistureLevel {
        let PlantId(id) = plant_id;

        let req = SerialRequest {
            seq: self.seq,
            req_type: 0,
            plant_id: id as DeviceInt,
        };

        self.seq += 1;

        self.send_request(req).to_moisture_level()
    }
}
