# Using message passing to transfer data between threads

Rust standard library provides channels [multi-producer single consumer queue](https://doc.rust-lang.org/stable/std/sync/mpsc/index.html), The module `std::sync::mpsc` provides communication over channels using the types `Sender`, `Receiver` and `SyncSender`.

Sender and SyncSender are clonable multiple senders can send messages to a single receiver. Channels come in two flavors -

- An async infinitely buffered channel. The `channel` function returns (`Sender`, `Receiver`) tuple in which all sends are async and the buffer has infinite buffer.
- A sync bounded channel - The `sync_channel` function returns (`SyncSender`, `Receiver`) tuple where storage for pending messages is a fixed size buffer that is allocated. All sends will be synchronous by blocking until the buffer space is available. Note that a bounded channel of size `0` is possible; here the channel becomes a rendezvous channel where each sender atomically hands off a message to a receiver.

## Disconnection

The send and receive operations on a channel all return `Result` indicating whether the operation succeeds or not. An unsuccessful operation is usually indicative of the other end of the channel usually hung-up by being dropped in the corresponding thread.

Once half channel has been deallocated most operations cannot proceed so `Err` is returned.

Basic channel code to send and receive a message -

```
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("got: {}", received);
}
```

## Channels and ownership transference

Ownership rules play a critical rule when using channels as they help you write safe concurrent code.

```
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
	println!("val: {}", val);
    });

    let received = rx.recv().unwrap();
    println!("got: {}", received);
}
```

The error you see when trying to access `val` after `send` is -

```
error[E0382]: borrow of moved value: `val`
  --> src/main.rs:10:29
   |
8  |         let val = String::from("hi");
   |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val: {}", val);
   |                             ^^^ value borrowed here after move
```

**NOTE** The concurrency error is caught at compile time.

## Sending multiple values and see receiver waiting

See code [here](./channel_send_multiple_values)
