use std::ffi::OsString;
use std::io::{Cursor, Read, Write};
use std::mem;
use std::str;
use std::time::Duration;

use byteorder::{LittleEndian, ReadBytesExt};
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
    seq_number: DeviceInt,
    req_type: DeviceInt,
    plant_id: DeviceInt,
}

#[repr(packed)]
struct SerialResponse {
    /// The sequence number this is a response to
    seq_number: DeviceInt,
    plant_id: DeviceInt,
    /// Length of the data segment
    len: DeviceInt,
    buf: [u8; RESPONSE_MAX_LEN],
}

impl SerialResponse {
    // TODO: Make result type
    fn to_moisture_level(self) -> Option<MoistureLevel> {
        let id = PlantId(self.plant_id as u32);
        let mut rdr = Cursor::new(&self.buf[..]);
        let level = MoistureLevel::new(id, rdr.read_i16::<LittleEndian>().unwrap());

        Some(level)
    }
}

pub struct SerialComms {
    port: serial::SystemPort,
    seq_number: DeviceInt,
}

impl SerialComms {
    pub fn new(port: &str) -> serial::Result<SerialComms> {
        let mut port = serial::open(port)?;
        port.configure(&SETTINGS)?;
        port.set_timeout(Duration::from_secs(5))?;

        Ok(SerialComms {
            port: port,
            seq_number: 0,
        })
    }

    fn send_request(&mut self, req: SerialRequest) -> SerialResponse {
        // Send request
        self.port.write(unsafe {
          &mem::transmute::<SerialRequest, [u8; SIZE_OF_REQUEST]>(req)
        });

        // Process response
        let mut buf = [0_u8; SIZE_OF_RESPONSE];
        self.port.read(&mut buf[..]).unwrap();

        println!("Read: {}", self.port.read(&mut buf[..]).unwrap());
        let response = unsafe {
            mem::transmute::<[u8; SIZE_OF_RESPONSE], SerialResponse>(buf)
        };

        response
    }
}

impl Comms for SerialComms {
    fn req_moisture(&mut self, plant_id: PlantId) -> MoistureLevel {
        let PlantId(id) = plant_id;

        let req = SerialRequest {
            seq_number: self.seq_number,
            req_type: 0,
            plant_id: id as DeviceInt,
        };

        self.seq_number += 1;

        self.send_request(req).to_moisture_level().unwrap()
    }
}
