// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! This simple OPC UA client will do the following:
//!
//! 1. Create a client configuration
//! 2. Connect to an endpoint specified by the url with security None
//! 3. Subscribe to values and loop forever printing out their values
use std::fmt::Formatter;
use std::fs::File;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender, SendError, sync_channel, SyncSender};
use std::{thread, time};
use std::time::Instant;

use opcua::client::prelude::*;
use opcua::server::prelude::Variable;
use opcua::sync::RwLock;



#[derive(Debug, Copy, Clone)]
struct ProfilingData {

    runs: usize,
    min: Option<time::Duration>,
    current: Option<time::Duration>,
    max: Option<time::Duration>,
    average: Option<time::Duration>,
    data_points: [Option<time::Duration>; 32],
}

pub struct ProfileTimer {
    start: Instant,
}

impl ProfileTimer {
    pub fn elapsed(&self, interval_timer: time::Duration) -> time::Duration {
        self.start.elapsed() - interval_timer
    }
}

impl ProfilingData {
    fn new() -> Self {
        ProfilingData { runs: 0, min: None, current: None, max: None, average: None, data_points: [None; 32] }
    }

    fn add_round(mut self, elapsed: time::Duration) -> Self {
        if self.runs == 0 {
            self.min = Some(elapsed);
            self.current = Some(elapsed);
            self.max = Some(elapsed);
        }

        if elapsed < self.min.unwrap() {
           self.min = Some(elapsed);
        }

        if elapsed > self.max.unwrap() {
            self.max = Some(elapsed);
        }

        self.data_points[self.runs] = Some(elapsed);
        if self.average.is_none() {
            self.average = Some(elapsed);
        } else {
            self.average = Some(self.average.unwrap() + self.data_points[self.runs].unwrap());
        }

        if !(self.runs == 32) {
            self.runs += 1;
        }

        if self.data_points[31] != None {
            self.average = self.average.unwrap().checked_div(self.runs as u32);
            println!("Calculating the average");
        }
        self
    }

    fn csv_record_writer(&self, wtr: &mut csv::Writer<File>, id: usize) {
        wtr.write_field(&format!("{}", id)).unwrap();
        wtr.write_field(&format!("{:.2?}", self.min.unwrap())).unwrap();
        //wtr.write_field(format!("{:.2?}", self.current.unwrap())).unwrap();
        wtr.write_field(format!("{:.2?}", self.max.unwrap())).unwrap();
        wtr.write_field(format!("{:.2?}", self.average.unwrap())).unwrap();

        for data in self.data_points {
            if data.is_some() {
                wtr.write_field(format!("{:.2?}", data.unwrap())).unwrap();
            }
        }


        wtr.write_record(None::<&[u8]>).unwrap();
        wtr.flush().unwrap();
    }
}

impl std::fmt::Display for ProfilingData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ProfileData")?;
        writeln!(f, "\t Round: {}", self.runs);
        writeln!(f, "\t Min: {:.2?}", self.min.unwrap())?;
        writeln!(f, "\t Last: {:.2?}", self.current.unwrap())?;
        write!(f, "\t Data Points: [")?;
        for data in self.data_points {
            if data.is_some() {
                write!(f, "{:.2?}", data)?;
            }
        }
        write!(f, "]\n")?;
        write!(f, "\t average: {:.2?}", self.average)?;
        Ok(())
    }
}


struct Args {
    help: bool,
    url: String,
}

impl Args {
    pub fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();
        Ok(Args {
            help: args.contains(["-h", "--help"]),
            url: args
                .opt_value_from_str("--url")?
                .unwrap_or_else(|| String::from(DEFAULT_URL)),
        })
    }

    pub fn usage() {
        println!(
            r#"Simple Client
Usage:
  -h, --help   Show help
  --url [url]  Url to connect to (default: {})"#,
            DEFAULT_URL
        );
    }
}

const DEFAULT_URL: &str = "opc.tcp://127.0.0.1:4855";

fn main() -> Result<(), ()> {
    let profile = ProfilingData::new();
    // Read command line arguments
    let args = Args::parse_args().map_err(|_| Args::usage())?;
    if args.help {
        Args::usage();
    } else {
        // Optional - enable OPC UA logging
        opcua::console_logging::init();

        let (sender, recv): (SyncSender<(core::time::Duration, time::Instant)>, Receiver<(core::time::Duration, time::Instant)>) = sync_channel(10);
        let writer =

        thread::spawn(move || {
            let mut runs = 0;
            let mut wtr = csv::Writer::from_path("foo.csv").unwrap();
            let mut headers = vec!["ID".to_string(), "dt_min".to_string(), "dt_max".to_string(), "dt_average".to_string()];

            let mut base_name = "dt_datapoint_".to_string();
            for id in 1..=32 {
                let id = id.to_string();
                let mut data_point_name = base_name.clone();
                data_point_name.push_str(&id);
                headers.push(data_point_name);
            }

            wtr.write_record(&headers).unwrap();

            let mut data_runs = vec![];
            let mut profiling_data = ProfilingData::new();

            loop {
                match recv.recv() {
                    Ok((timestamp_stop, timestamp_start)) => {
                        println!("DURATION: {:?}", timestamp_start.elapsed() - timestamp_stop);
                        profiling_data = profiling_data.add_round(timestamp_start.elapsed() - timestamp_stop);

                        println!("{}",profiling_data);

                        if profiling_data.data_points[31] != None {
                            data_runs.push(profiling_data);
                            profiling_data.csv_record_writer(&mut wtr, runs);
                            profiling_data = ProfilingData::new();
                            runs += 1;
                        }
                    }
                    Err(e) => panic!("ERROR: {}", e)
                }
            }
        });

        // Make the client configuration
        let mut client = ClientBuilder::new()
            .application_name("Simple Client")
            .application_uri("urn:SimpleClient")
            .product_uri("urn:SimpleClient")
            .trust_server_certs(true)
            .create_sample_keypair(true)
            .session_retry_limit(3)
            .client()
            .unwrap();

        if let Ok(session) = client.connect_to_endpoint(
            (
                args.url.as_ref(),
                SecurityPolicy::None.to_str(),
                MessageSecurityMode::None,
                UserTokenPolicy::anonymous(),
            ),
            IdentityToken::Anonymous,
        ) {
            if let Err(result) = subscribe_to_variables(session.clone(), 2, sender) {
                println!(
                    "ERROR: Got an error while subscribing to variables - {}",
                    result
                );
            } else {
                // Loops forever. The publish thread will call the callback with changes on the variables

                let _ = Session::run(session);
            }
        }
    }
    Ok(())
}

pub fn data_changed(session: Arc<RwLock<Session>>, sender: SyncSender<(time::Duration, time::Instant)>, start: ProfileTimer) -> Result<u32, StatusCode> {
    let session = session.read();

    let mut profile_data = ProfilingData::new();
    let publishing_interval = 10.0;

    // Creates a subscription with a data change callback
    let subscription_id = session.create_subscription(
        publishing_interval,
        10,
        30,
        0,
        0,
        true,
        DataChangeCallback::new(move |changed_monitored_items| {
            sender.clone().send((start.elapsed(time::Duration::from_millis(publishing_interval as u64)), start.start)).unwrap_or_else(|e| println!("Error received {}", e));
            changed_monitored_items
                .iter()
                .for_each(move |item| print_value(item));
            //println!("{}", profile_data);
        }),
    )?;

    Ok(subscription_id)
}

fn subscribe_to_variables(session: Arc<RwLock<Session>>, ns: u16, sender: SyncSender<(time::Duration, time::Instant)>) -> Result<(), StatusCode> {
    //let session = session.read();
    // Creates a subscription with a data change callback
    let timer = ProfileTimer { start: Instant::now() };
    let subscription_id = data_changed(session.clone(), sender, timer)?;
    println!("Created a subscription with id = {}", subscription_id);
    /*let subscription_id = session.create_subscription(
        50.0,
        10,
        30,
        0,
        0,
        true,
        DataChangeCallback::new(|changed_monitored_items| {
            let mut profile = ProfilingData::new();
            let before = Instant::now();
            println!("Data change from server:");
            changed_monitored_items
                .iter()
                .for_each(move |item| {
                    profile.add_round(before.elapsed());
                    print_value(item)
                });
        }),
    )?;*/


    // Create some monitored items
    let session = session.read();
    let mut monitor_list = vec![];
    let base = String::from("v");
    for n in 0..1000 {
        let id = n.to_string();
        let mut display_name = base.clone();
        display_name.push_str(&id);
        monitor_list.push(display_name);
    }

    let items_to_create: Vec<MonitoredItemCreateRequest> = monitor_list
        .iter()
        .map(|v| NodeId::new(ns, v.clone()).into())
        .collect();

    let _ = session.create_monitored_items(
        subscription_id,
        TimestampsToReturn::Both,
        &items_to_create,
    )?;

    Ok(())
}

fn print_value(item: &MonitoredItem) {
    let node_id = &item.item_to_monitor().node_id;
    let data_value = item.last_value();
    if let Some(ref value) = data_value.value {
        println!("Item \"{}\", Value = {:?}", node_id, value);
    } else {
        println!(
            "Item \"{}\", Value not found, error: {}",
            node_id,
            data_value.status.as_ref().unwrap()
        );
    }
}
