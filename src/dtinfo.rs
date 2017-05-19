mod dtlib;

fn main() {
  let written = dtlib::dt::get_bytes_written();
  let utime = dtlib::uptime::get_uptime();

  match (
  	  written.ok().map(sectors_to_gb),
  	  utime.ok()
  	) {
    (Some(gbs), Some(utime)) => {
    	let up_hours = utime.as_secs() as f32 / (60 * 60) as f32;

    	println!("GB written: {}", gbs);
    	println!("Uptime: {} h", up_hours);
    	println!("Result: {} GB/h", gbs / up_hours);
    },
    _ => println!("Failed to retrieve information.")
  }
}

fn sectors_to_gb(sectors : usize) -> f32 {
  return (sectors * dtlib::dt::BLOCK_SIZE) as f32 / (1024 * 1024 * 1024) as f32;
}
