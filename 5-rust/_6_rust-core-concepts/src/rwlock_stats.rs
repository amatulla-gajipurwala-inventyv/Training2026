use std::sync::RwLock;

#[derive(Debug)]
struct RequestStats {
    total: u32,
}

static STATS: RwLock<RequestStats> = RwLock::new(RequestStats { total: 0 });

pub fn run() {
    let mut stats = STATS.write().unwrap();
    stats.total += 1;

    println!("{:?}", STATS.read().unwrap());
}
