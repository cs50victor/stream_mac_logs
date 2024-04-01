use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

use log::{error, info};

pub fn exec_stream<P: AsRef<Path>>(binary: P, args: Vec<&'static str>) {
    let mut cmd = Command::new(binary.as_ref())
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);

        for line in stdout_reader.lines() {
            match line {
                Ok(l) => info!("{l}"),
                Err(e) => error!("{e}"),
            }
        }
    }

    cmd.wait().unwrap();
}

fn main() {
    pretty_env_logger::formatted_builder()
    .filter_module("stream_mac_logs", log::LevelFilter::Info)
    .init();

    exec_stream("log", vec!["stream"]);
}
