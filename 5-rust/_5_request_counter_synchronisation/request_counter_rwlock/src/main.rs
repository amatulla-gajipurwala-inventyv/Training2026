use std::sync::RwLock;

#[derive(Debug)]
enum Request {
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32),
}

#[derive(Debug)]
struct RequestStats {
    total: u32,
    get: u32,
    post: u32,
    delete: u32,
}

static REQUEST_STATS: RwLock<RequestStats> = RwLock::new(RequestStats {
    total: 0,
    get: 0,
    post: 0,
    delete: 0,
});

fn handle_request(req: Request) -> Result<String, String> {
    let mut stats = match REQUEST_STATS.write() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("⚠ RwLock was poisoned, recovering...");
            poisoned.into_inner()
        }
    };

    stats.total += 1;

    let response = match req {
        Request::Get { endpoint } => {
            stats.get += 1;
            format!("GET request received for {}", endpoint)
        }
        Request::Post {
            endpoint,
            payload_size,
        } => {
            stats.post += 1;
            format!(
                "POST request received for {} with payload size {} bytes",
                endpoint, payload_size
            )
        }
        Request::Delete(id) => {
            stats.delete += 1;
            format!("DELETE request received for resource id {}", id)
        }
    };

    Ok(response)
}

fn main() {
    let requests = vec![
        Request::Get {
            endpoint: "/users".into(),
        },
        Request::Post {
            endpoint: "/upload".into(),
            payload_size: 512,
        },
        Request::Delete(42),
    ];

    for req in requests {
        match handle_request(req) {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    let stats = REQUEST_STATS.read().map_err(|e| e.into_inner()).unwrap();
    println!("\n Stats (RwLock)");
    println!("{stats:?}");
     println!("\n Request Statistics");
    println!("Total   : {}", stats.total);
    println!("GET     : {}", stats.get);
    println!("POST    : {}", stats.post);
    println!("DELETE  : {}", stats.delete);
}
