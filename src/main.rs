use carben;

pub fn main() {
    let mut runner = carben::MainRunner::new();
    let result = runner.run();
    if result.is_err() {
        eprintln!("Error: {:?}", result);
    }
}
