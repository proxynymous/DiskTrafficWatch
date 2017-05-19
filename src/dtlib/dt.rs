use std::io::Result;
use std::io::{Error, ErrorKind};

use dtlib::shared;

const STAT_PATH : &str = "/sys/block/sda/stat";
const SECTOR_IDX : usize = 6;
#[allow(dead_code)]
pub const BLOCK_SIZE : usize = 512;

pub fn get_bytes_written() -> Result<usize> {
  return shared::read_file(STAT_PATH)
	         .and_then(
	           |s| extract_sectors_written(&s)
	               .ok_or(
	                 Error::new(ErrorKind::Other, "Could not parse the drive stats")
	                )
	          )
}

pub fn extract_sectors_written(input : &String) -> Option<usize> {
  return input.split_whitespace()
              .nth(SECTOR_IDX)
              .and_then(|s| s.parse::<usize>().ok());
}
