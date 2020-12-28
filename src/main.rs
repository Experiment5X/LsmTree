mod store;

fn main() {
    // create a dictionary of strings -> (Maybe byte)[] (actually strings)

    // once dictionary gets 10k keys, write to disk in sorted order as a segment

    // keep an array, one element for each segment file, of a sparse dictionary where the keys map to addresses in the
    // file, can start will having a full index

    // can add bloom filters to reduce necessary lookups for keys that don't exist in the store
    let mut kv_store = store::Store::new();
    kv_store.put("hello".to_string(), "world".to_string());

    print!("Store: ");
    for (key, value) in kv_store.iter() {
        print!("{} -> {}, ", key, value);
    }
    println!();

    println!("Hello, world!");
}
