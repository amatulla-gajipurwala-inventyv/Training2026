use chrono::{DateTime, Utc};
use rand::{Rng, distributions::Alphanumeric};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, atomic::AtomicI32};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct MultiThread {
    id: i32,
    recordAddedTime: String,
    threadId: String, // randomly generated id
}

static GLOBAL_ID: AtomicI32 = AtomicI32::new(1);
fn record_age(r: &MultiThread) -> i64 {
    let created: DateTime<Utc> = r.recordAddedTime.parse().expect("Invalid time");
    (Utc::now() - created).num_seconds()
}
fn generate_thread_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect() //to string of 8 char 
}

fn main() {
    let shared_data: Arc<Mutex<Vec<MultiThread>>> = Arc::new(Mutex::new(Vec::new()));

    {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || {
            loop {
                
                    let id = GLOBAL_ID.fetch_add(1, Ordering::SeqCst);
                    let record = MultiThread {
                        id,
                        recordAddedTime: Utc::now().to_rfc3339(),
                        threadId: generate_thread_id(),
                    };
                    {
                    data.lock().unwrap().push(record);
                    }
                    println!("Record Added  Thread1");
                    thread::sleep(Duration::from_secs(10));
            }
        });
    }
    {
        let data: Arc<Mutex<Vec<MultiThread>>> = Arc::clone(&shared_data);
        thread::spawn(move || {
            loop {
               let snapshots= {               //so that printing can be done without holding lock 
                    let records = data.lock().unwrap();
                    records.clone()
                };
                    println!("Thread2 Currrent Records len  :{}", snapshots.len());
                    for r in snapshots.iter() {
                        println!("Thread2 Record :{:?}", r);
                    }
                    println!("Thread2--------------------");
                
                thread::sleep(Duration::from_secs(5));
            }
        });
    }
    {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || {
            loop {
                {
                    let mut records = data.lock().unwrap();
                    records.retain(|r| !(r.id % 2 == 0 && record_age(r) > 20));
                    println!("Thread3 records remove even id and age >20");
                    // println!("Thraed3 records {:?}", records);
                }
                thread::sleep(Duration::from_secs(5));
            }
        });
    }
    {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || {
            loop {
                {
                    let mut records = data.lock().unwrap();
                    records.retain(|r| !(r.id % 2 != 0 && record_age(r) > 20));
                    println!("Thread4 records remove odd id and age >20");
                    // println!("Thraed4 records {:?}", records);
                }
                thread::sleep(Duration::from_secs(5));
            }
        });
    }
    {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || {
            loop {
                {
                    let records = data.lock().unwrap();
                    let count = records.iter().filter(|r| r.id % 2 == 0).count();
                    println!("Thread5  Total Records with even ids {}", count);
                }
                thread::sleep(Duration::from_secs(5));
            }
        });
    }
    {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || {
            loop {
                {
                    let records = data.lock().unwrap();
                    let count = records.iter().filter(|r| r.id % 2 != 0).count();
                    println!("Thread6  Total Records with odd ids {}", count);
                }
                thread::sleep(Duration::from_secs(5));
            }
        });
    }
    thread::sleep(Duration::from_secs(60));
}
