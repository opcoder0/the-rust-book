## Collections

Template based collections are defined in the standard library (`std::collections`); The data here is stored on the heap and hence the size of the data can be grown / shrunk at runtime. Some of the most common collections are `Vec` available in `std::vec` which allows you to store data contiguously and `String` which is a sequence of characters and `HashMap` a key-value pair accessed by hashing. There are others like `VecDequeue`, `LinkedList`, `BTreeMap`, `HashSet` etc (in `std::collections`).

## Vectors

Vectors (`std::vec`) -

- Is a generic container type
- Store elements contiguously on the heap
- All elements must be of the same type
- When a vector is dropped all its elements are dropped as well

_NOTE_: To store different element types in a vector use enum with associated values.


