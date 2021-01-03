mod store;

fn main() {
    // create a dictionary of strings -> (Maybe byte)[] (actually strings)

    // once dictionary gets 10k keys, write to disk in sorted order as a segment

    // keep an array, one element for each segment file, of a sparse dictionary where the keys map to addresses in the
    // file, can start will having a full index

    // can add bloom filters to reduce necessary lookups for keys that don't exist in the store

    // let mut kv_store = store::Store::new();
    // kv_store.put("adam".to_string(), "sushi".to_string());
    // kv_store.put("bob".to_string(), "burgers".to_string());
    // kv_store.put("jill".to_string(), "ice cream".to_string());
    // kv_store.put("tim".to_string(), "candy".to_string());
    // kv_store.put("sally".to_string(), "meatloaf".to_string());
    // kv_store.put("justin".to_string(), "pancakes".to_string());

    match store::segment::Segment::new_from_file("test.seg".to_string()) {
        Ok(segment) => {
            print!("Segment Store: ");
            for key in segment.keys.iter() {
                print!("{},  ", key);
            }
            println!();

            let value = match segment.lookup("jill".to_string()) {
                Some(v) => v,
                None => "Couldn't lookup value".to_string(),
            };
            println!("value: {}", value);
        }
        Err(_) => println!("Error"),
    }

    // print!("Store: ");
    // for (key, value) in kv_store.iter() {
    //     print!("{} -> {}, ", key, value);
    // }
    // println!();

    // println!("Hello, world!");
}
