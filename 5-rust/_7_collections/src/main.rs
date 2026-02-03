use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let mut users: HashMap<u32, User> = HashMap::new();

    users.insert(
        1,
        User {
            id: 1,
            name: "Ama".to_string(),
        },
    );
    users.insert(
        1,
        User {
            id: 3,
            name: "Amsbdb".to_string(),
        },
    );

    users.insert(
        2,
        User {
            id: 2,
            name: "John".to_string(),
        },
    );

    println!("Original map: {:?}", users);

    let cloned_users = users.clone();
    println!("Cloned map: {:?}", cloned_users);
    println!("capcaity {}", users.capacity());
    println!("len  {}", users.len());

    match users.try_reserve(10) {
        Ok(_) => println!("Successfully reserved capacity"),
        Err(e) => println!("Failed to reserve capacity: {}", e),
    }
    println!("capcaity {}", users.capacity()); //buc count

    let mut removed_user: Option<User> = users.get_mut(&1).map(|u| u.clone());

    if let Some(user) = removed_user.take() {
        ///take out some(value)
        println!("Taken user safely: {:?}", user);
    } else {
        println!("No user to take");
    }

    // Remove key completely
    let removed = users.remove(&1);
    println!("Removed from map: {:?}", removed);
    println!("Original map: {:?}", users);

    let mut new_users: HashMap<u32, User> = HashMap::new();

    new_users.insert(
        3,
        User {
            id: 3,
            name: "Abc".to_string(),
        },
    );

    new_users.insert(
        4,
        User {
            id: 4,
            name: "Bob".to_string(),
        },
    );

    users.extend(new_users); //into_iter move
    println!("After extend(): {:?}", users);
    // println!("After extend(): {:?}", new_users)   move value

    users.retain(|_id, user| user.name.starts_with('J'));
    println!("After retain(): {:?}", users);

    println!();
    //HashSet
    let mut user_set: HashSet<User> = HashSet::new();

    user_set.insert(User {
        id: 1,
        name: "Ama".to_string(),
    });
    user_set.insert(User {
        id: 1,
        name: "Ama".to_string(),
    });

    user_set.insert(User {
        id: 1,
        name: "cknascn".to_string(),
    });

    user_set.insert(User {
        id: 2,
        name: "John".to_string(),
    });
    user_set.insert(User {
        id: 3,
        name: "Jake".to_string(),
    });

    println!("\nOriginal HashSet: {:?}", user_set);

    // clone
    let cloned_set = user_set.clone();
    println!("Cloned HashSet: {:?}", cloned_set);

    // try_reserve
    println!("capacity {}", user_set.capacity());
    match user_set.try_reserve(5) {
        Ok(_) => println!("HashSet reserved capacity successfully"),
        Err(e) => println!("HashSet failed to reserve: {}", e),
    }
    println!("after capacity {}", user_set.capacity());

    let mut taken_user: Option<User> = user_set.take(&User {
        id: 1,
        name: "Ama".to_string(),
    });
    if let Some(u) = taken_user.take() {
        println!("Taken user safely from HashSet: {:?}", u);
    }

    println!("HashSet after take(): {:?}", user_set);

    // extend
    let mut new_set: HashSet<User> = HashSet::new();
    new_set.insert(User {
        id: 4,
        name: "Alice".to_string(),
    });
    new_set.insert(User {
        id: 5,
        name: "Bob".to_string(),
    });

    user_set.extend(new_set);
    println!("HashSet after extend(): {:?}", user_set);

    // retain
    user_set.retain(|user| user.name.starts_with('J'));
    println!("HashSet after retain(): {:?}", user_set);
}
