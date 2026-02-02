use std::sync::Mutex;

#[derive(Debug)]
struct RequestStats {
    total: u32,
}

static STATS: Mutex<RequestStats> = Mutex::new(RequestStats { total: 0 });

pub fn run() {
    let mut stats = STATS.lock().unwrap();
    stats.total += 1;

    println!("{:?}", *stats);
}
