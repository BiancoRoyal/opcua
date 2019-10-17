use futures::{
    future::{self, FutureResult},
};
use std::{thread, time};
use tokio_service::Service;

use tokio_modbus::prelude::*;

struct MbServer {
    start_time: time::Instant,
}

impl Service for MbServer {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req {
            Request::ReadInputRegisters(_addr, cnt) => {
                let mut registers = vec![0; cnt as usize];

                let now = time::Instant::now();
                let elapsed = now - self.start_time;
                let elapsed = elapsed.as_secs();

                // Set the register values on their index + the start time duration to now
                for i in 0..(cnt as usize) {
                    let register_value = (i as u64 + elapsed) % std::u16::MAX as u64;
                    registers[i] = register_value as u16;
                }

                let rsp = Response::ReadInputRegisters(registers);
                future::ok(rsp)
            }
            _ => unimplemented!(),
        }
    }
}

pub fn run_modbus_slave(address: &str) {
    let socket_addr = address.parse().unwrap();

    println!("Starting up slave...");
    let _server = thread::spawn(move || {
        tcp::Server::new(socket_addr).serve(|| Ok(MbServer {
            start_time: time::Instant::now()
        }));
    });
}