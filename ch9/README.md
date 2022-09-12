## Recoverable Errors

[Here](./recoverable_errors/README.md)

## To panic or not panic

- For cases like example code, prototypes, tests it is a good idea to panic.
- For cases in which you have more information than the compiler. Example -

	```
	fn main() {
	    use std::net::IpAddr;
	    let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");
	}
	```

	Here the compiler would make us handle the `Result` returned by the `parse` method. 
	Since we know the value is always `Ok`. It's okay to panic via expect for an error case.

## Guidelines for error handling

It is advisable to have your code panic when it's possible that your code could end up in a bad state. _Bad state_ is is when some _assumption, guarantee, contract, invariant_ has been broken. 

- A _bad state_ is something that is unexpected as opposed to something that will likely happen occassionally.
- Your code after this point needs to rely on being in a good state rather than check for the problem in every step.
- There is no good way to encode this information in types.

However when a failure is expected it is more appropriate to return a `Result` than to `panic!`.

## Creating custom types for validation

Having lot of error checks can make the code verbose and annoying. Instead we can use a custom type that allows us to validate during instantiation and use it without checking further.



