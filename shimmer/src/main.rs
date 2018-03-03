mod cli;

use std::env::current_exe;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;

fn build_shim_data(shim_dir: &str, source: &PathBuf, buf: &mut Vec<u8>) {
    let mut path = PathBuf::from(shim_dir);
    if path.is_relative() {
        path = current_exe().unwrap().parent().unwrap().to_path_buf();
        path.push(shim_dir);
    }
    path.push("shim.exe");
    File::open(path).unwrap().read_to_end(buf).expect("could not read shim");
    
    let mut data = source.to_str().map_or_else(|| {
        eprintln!("source: {:?} does not decode", source);
        exit(1);
    }, |s| {
        let mut data = s.as_bytes().to_vec();
        data.append(&mut vec![0x0a, 0x0a]); // Two line feeds ends the command.
        data.reverse();
        data
    });
    buf.append(&mut data);
}

fn main() {
    let options = cli::parse();

    let source = cli::resolve_source(options.value_of("source").unwrap());
    let mut shim_data = vec![];
    build_shim_data(
        option_env!("SHIM_DIR").unwrap_or(""),
        &source,
        &mut shim_data,
    );

    let target = cli::resolve_target(
        options.value_of("target").unwrap(),
        source.file_name().unwrap(),
    );
    let mut shim = File::create(target).unwrap_or_else(|e| {
        eprintln!("target: {}", e.description());
        exit(1);
    });
    shim.write_all(&shim_data).expect("could not write shim");
}
