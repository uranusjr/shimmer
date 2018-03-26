use std::env::current_dir;
use std::error::Error;
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;

use clap::{Arg, ArgMatches};

macro_rules! bail {
    ( $key:expr, $error:expr ) => {
        eprintln!("{}: {}", $key, $error.description());
        use std::process::exit;
        exit($error.raw_os_error().unwrap_or(1));
    }
}

pub fn parse<'a>() -> ArgMatches<'a> {
    let app = app_from_crate!()
        .arg(Arg::with_name("source")
            .required(true)
            .help("Executable to shim"))
        .arg(Arg::with_name("target")
            .required(true)
            .help("Where to put the shim"));
    app.get_matches()
}

fn resolve(s: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(s);
    if path.is_absolute() {
        return Ok(path);
    }
    current_dir().map(|d| {
        let mut path = PathBuf::from(d);
        path.push(s);
        path
    })
}

pub fn resolve_source(s: &str) -> PathBuf {
    let result = resolve(s).and_then(|path| {
        if path.is_dir() {
            Err(io::Error::new(io::ErrorKind::PermissionDenied, format!(
                "{:?} is a directory", path,
            )))
        } else if !path.exists() {
            Err(io::Error::new(io::ErrorKind::PermissionDenied, format!(
                "{:?} does not exist", path,
            )))
        } else {
            Ok(path)
        }
    });
    match result {
        Ok(path) => path,
        Err(e) => { bail!("source", e); },
    }
}

pub fn resolve_target(s: &str, default_name: &OsStr) -> PathBuf {
    let result = resolve(s).and_then(|mut path| {
        if path.is_dir() {
            path.push(default_name);
        }
        if path.exists() {
            Err(io::Error::new(io::ErrorKind::AlreadyExists, format!(
                "{:?} exists", path,
            )))
        } else {
            Ok(path)
        }
    });
    match result {
        Ok(path) => path,
        Err(e) => { bail!("target", e); },
    }
}
