//! List system USB buses and devices; a modern `lsusb` that attempts to maintain compatibility with, but also add new features.
//!
//! # Examples
//!
//! Profile USB devices on cross-platform systems:
//!
//! ```no_run
//! use cyme::profiler;
//! let sp_usb = profiler::get_spusb().unwrap();
//! ```
//!
//! Profile USB devices with all extra descriptor data (requires opening devices) on cross-platform systems:
//!
//! ```no_run
//! use cyme::profiler;
//! let sp_usb = profiler::get_spusb_with_extra().unwrap();
//! ```
//!
//! It's often useful to then flatten this into a list of devices ([`profiler::Device`]):
//!
//! ```no_run
//! # use cyme::profiler;
//! # let sp_usb = profiler::get_spusb().unwrap();
//! // flatten since we don't care tree/buses
//! let devices = sp_usb.flattened_devices();
//!
//! for device in devices {
//!    format!("{}", device);
//! }
//! ```
//!
//! One can then print with the cyme display module:
//!
//! ```no_run
//! # use cyme::profiler;
//! # let sp_usb = profiler::get_spusb().unwrap();
//! # let devices = sp_usb.flattened_devices();
//! use cyme::display;
//! // print with default [`display::PrintSettings`]
//! display::DisplayWriter::default().print_flattened_devices(&devices, &display::PrintSettings::default());
//! ```
//!
//! The [`profiler::SystemProfile`] struct contains system [`profiler::Bus`]s, which contain [`profiler::Device`]s as a USB tree.
#![allow(dead_code)]
#![warn(missing_docs)]

pub mod colour;
pub mod config;
pub mod display;
pub mod error;
pub mod icon;
pub mod lsusb;
pub mod profiler;
pub mod types;
#[cfg(all(target_os = "linux", feature = "udev"))]
pub mod udev;
#[cfg(all(all(target_os = "linux", feature = "udevlib"), not(feature = "udev")))]
#[path = "udev_ffi.rs"]
pub mod udev;
pub mod usb;

/// Set cyme module and binary log level
pub fn set_log_level(debug: u8) -> crate::error::Result<()> {
    let mut builder = env_logger::Builder::new();
    let builder = match debug {
        // just use env if not passed
        0 => builder
            .parse_default_env()
            .filter_module("udevrs", log::LevelFilter::Off)
            .filter_module("nusb", log::LevelFilter::Off),
        1 => builder
            .filter_module("udevrs", log::LevelFilter::Warn)
            .filter_module("nusb", log::LevelFilter::Warn)
            .filter_module("cyme", log::LevelFilter::Info),
        2 => builder
            .filter_module("udevrs", log::LevelFilter::Info)
            .filter_module("nusb", log::LevelFilter::Info)
            .filter_module("cyme", log::LevelFilter::Debug),
        3 => builder
            .filter_module("udevrs", log::LevelFilter::Debug)
            .filter_module("nusb", log::LevelFilter::Debug)
            .filter_module("cyme", log::LevelFilter::Trace),
        // all modules at Trace level
        _ => builder.filter_level(log::LevelFilter::Trace),
    };

    builder.try_init().map_err(|e| {
        crate::error::Error::new(
            crate::error::ErrorKind::Other("logger"),
            &format!("Failed to set log level: {}", e),
        )
    })?;

    Ok(())
}

// run any Rust code as doctest
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
