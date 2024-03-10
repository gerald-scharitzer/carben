use carben;

pub fn main() {
	let mut runner = carben::MainRunner::new();
	if let Err(msg) = runner.run() {
		eprintln!("Error: {msg}");
	}
}
