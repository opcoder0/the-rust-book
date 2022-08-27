# Ownership

Refer to [Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).

```
(gdb) list
5           // That way only one owner
6           let s2 = s1;
7           // cannot use s1 after the line above
8           // compiler error - value borrowed/used after
9           // move.
10          println!("s2 is {}", s2);
11      }
(gdb) print s1                                                                                       
$3 = "hello"
(gdb) print s2
$4 = "hello"
(gdb) print &s1
$5 = (*mut alloc::string::String) 0x7fffffffd928
(gdb) print &s2
$6 = (*mut alloc::string::String) 0x7fffffffd980
```

`s1` and `s2` are stored on the stack with the structure which holds the `length, capacity, pointer to data on heap`.

![String Structure](https://doc.rust-lang.org/book/img/trpl04-01.svg)
