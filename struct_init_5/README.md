## Ownership of struct data

In the `User` struct definition all the fields were owned by the struct. If we specified this instead -

```
struct User {
    username: &str,
    email: &str,
    active: bool,
    signin_count: u64,
}

fn main() {
	let user = User{
	    email: "someone@example.com",
	    username: "someone",
	    active: true,
	    signin_count: 3,
	};
}
```

The above won't work because it is missing lifetime parameter. Check Chapter 10 for lifetimes.

```
struct User<'a> {
    username: &'a str,
    email: &'a 'str,
    active: bool,
    signin_count: u64,
}
```
