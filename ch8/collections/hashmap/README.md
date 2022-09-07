## Hashmap

Generic key-value map.

## Ownership

For types that implement the `Copy` trait the values are copied into the hashmap. For those that don't the values are moved and the HashMap becomes the owner of the value.

If we insert references to values into HashMaps the values won't be moved. But the references must be valid for at least as long as the hashmap is valid.

## Updating a HashMap

### Overwriting a Value

When a value is inserted for a key more than once the value is overwritten.

```
    // Updating a HashMap
    println!("Updating a HashMap:");
    let mut h = HashMap::new();
    h.insert("Blue", 50);
    h.insert("Blue", 100);
    println!("{:?}", h);
```

Prints `{"Blue": 100}`.

### Adding a key and value only if the key is not present

For this the HashMap provides entry method. This [entry](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) method returns the corresponding [Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) in the map for in-place mutation.

`Entry` is a view into a single entry in the map that can either be "Occupied" or "Vacant".



