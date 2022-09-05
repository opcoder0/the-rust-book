## Enum Basics

Enums -

- Used for representing type that can have different values (variants).
- In Rust enums are different. They can have data associated with them that is stored with the enum.

Instead of storing the IP Address in a struct this can be reversed and stored in an enum. Example -

```
struct IpAddr {
  kind: IpAddrKindV1,
  address: String,
}
```

This can instead be stored in an Enum like `IpAddrKindV2` -

```
enum IpAddrKindV2 {
   V4(String),
   V6(String),
}
```
The Enum variants can have different types of values.

```
enum IpAddrKindV3 {
  V4(u8, u8, u8, u8),
  V6(String),
}
```

Standard library does -

```
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Enums like structs can have methods defined in the `impl` block.
