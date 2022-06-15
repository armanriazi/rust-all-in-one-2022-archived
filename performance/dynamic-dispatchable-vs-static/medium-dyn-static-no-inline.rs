use std::time::SystemTime;

struct PositiveBackend;

impl PositiveBackend{
    fn compute(&self, number: u64) -> u64{
        number+1
    }
}

fn main() {
    let backend = Box::new(PositiveBackend);
    let mut res= 0 as u64;
    let start_time = SystemTime::now();
    let total = 20_000_000 as u64;
    
    // our main loop
    for i in 0 .. total {
        res += backend.compute(i);
    }

    println!("Result: {}",res);
    println!("Elapsed_ms: {}", start_time.elapsed().unwrap().as_millis());
}
