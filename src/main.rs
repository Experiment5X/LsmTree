use std::io;
use std::io::Write;

mod store;

fn main() {
    // create a dictionary of strings -> (Maybe byte)[] (actually strings)

    // once dictionary gets 10k keys, write to disk in sorted order as a segment

    // keep an array, one element for each segment file, of a sparse dictionary where the keys map to addresses in the
    // file, can start will having a full index

    // can add bloom filters to reduce necessary lookups for keys that don't exist in the store

    let mut kv_store = store::Store::new();
    kv_store.put("adam".to_string(), "sushi".to_string());
    kv_store.put("bob".to_string(), "burgers".to_string());
    kv_store.put("jill".to_string(), "ice cream".to_string());
    kv_store.put("tim".to_string(), "candy".to_string());
    kv_store.put("sally".to_string(), "meatloaf".to_string());
    kv_store.put("justin".to_string(), "pancakes".to_string());

    for i in 0..25 {
        kv_store.put(format!("ronny-{}", i), format!("grease-{}", i));
    }

    println!("Welcome to Adam's Key Value Store");
    loop {
        print!("> ");
        io::stdout().flush();

        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(_) => (),
        };

        if line.starts_with("get ") {
            let key = line.replace("get ", "").replace("\n", "");
            match kv_store.lookup(key) {
                Some(value) => println!("{}", value),
                None => println!("* Not found *"),
            };
        }

        if line.starts_with("del ") {
            let key = line.replace("del ", "").replace("\n", "");
            kv_store.delete(key)
        }

        if line.starts_with("put ") {
            let line_cleaned = line.replace("put ", "").replace("\n", "");
            let components: Vec<&str> = line_cleaned.split(" ").collect();

            if components.len() != 2 {
                println!("")
            }

            let key = components[0];
            let value = components[1];

            kv_store.put(key.to_string().clone(), value.to_string().clone());
        }
    }
}
