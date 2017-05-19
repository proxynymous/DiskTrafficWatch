use std::io::Result;
use std::io::{Error, ErrorKind};
use std::time::Duration;

use dtlib::shared;

const UPTIME_PATH : &str = "/proc/uptime";
const UPTIME_WS_IDX : usize = 0;
const UPTIME_C_IDX : usize = 0;

pub fn get_uptime() -> Result<Duration> {
  return shared::read_file(UPTIME_PATH)
	       .and_then(|s| extract_uptime(&s).ok_or(Error::new(ErrorKind::Other, "Could not parse the uptime")))
	       .map(|secs| Duration::from_secs(secs))
}

pub fn extract_uptime(input : &String) -> Option<u64> {
  return input.split_whitespace()
              .nth(UPTIME_WS_IDX)
              .and_then(|s| s.split('.').nth(UPTIME_C_IDX))
              .and_then(|s| s.parse::<u64>().ok());
}
