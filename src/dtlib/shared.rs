use std::io::Result;
use std::io::Read;
use std::fs::File;
//use std::io::Write;

pub fn read_file(path : &str) -> Result<String> {
  let mut file = File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  return Ok(contents);
}

//pub fn require(in : &[Result], op : ) ->

//from http://stackoverflow.com/questions/27588416/how-to-send-output-to-stderr
#[macro_export]
macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);
