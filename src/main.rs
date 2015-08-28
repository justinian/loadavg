extern crate libc;

#[link(name = "c")]
extern {
	fn getloadavg(loadavg: &[f64], nelem: i32) -> i32;
}

fn main() {
	let loadavgs: [f64; 3] = [0.0; 3];
	let n = unsafe { getloadavg(&loadavgs, 3) };
	if n > 0 {
		println!("{:.2} {:.2} {:.2}",
			   loadavgs[0],
			   loadavgs[1],
			   loadavgs[2],
			  );
	} else {
		std::process::exit(1);
	}
}
