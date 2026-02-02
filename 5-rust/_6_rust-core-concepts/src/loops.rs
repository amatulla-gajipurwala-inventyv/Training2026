pub fn run() {
    println!("--- loop: retry until success ---");

    let mut attempts = 0;

    loop {
        attempts += 1;
        println!("Trying attempt {}", attempts);

        if attempts == 3 {
            println!("Operation successful!");
            break;
        }
    }

    println!("\n--- while: countdown timer ---");

    let mut time_left = 5;
    while time_left > 0 {
        println!("Time left: {}", time_left);
        time_left -= 1;
    }

    println!("\n--- for: iterate over array ---");

    let scores = [90, 85, 78, 92, 88];
    for score in scores {
        println!("Score = {}", score);
    }

    println!("\n--- for: fixed retries ---");

    for i in 1..=3 {
        println!("Retry {}", i);
    }

    println!("\n--- loop returning value ---");

    let result = loop {
        let value = 7;
        if value % 2 == 0 {
            break value;
        } else {
            break value + 1;
        }
    };

    println!("Result = {}", result);
}
