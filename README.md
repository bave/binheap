# binheap

binheap (rust library test)

```
[/tmp/test_binheap]$ tree ./
./
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```
---

## Cargo.toml

```markdown
[package]
name = "test_binheap"
version = "0.1.0"
authors = ["bave <inoue.tomoya@gmail.com>"]
edition = "2018"
[dependencies]
binheap = { git = "https://github.com/bave/binheap.git" }
```

---

## src/main.rs
```rust
extern crate binheap;

use std::process::exit;

fn main() {

    // min heap
    let mut bh = binheap::BinHeap::new() as binheap::BinHeap<u64>;
    for i in vec![1,8,2,7,3,6,4,5] {
        bh.push(i);
    }
    print!("\nmin-heap\n");
    while !bh.is_empty() {
        print!("{:?}\n", bh.pop());
    }
    print!("{:?}\n", bh.pop());

    // max heap
    fn is_greater(a: &u64, b: &u64) -> bool {
        if a.cmp(b) == std::cmp::Ordering::Greater {
            true
        } else {
            false
        }
    }
    print!("\nmax-heap\n");
    let mut bh = binheap::BinHeap::new_with_cmp(is_greater) as binheap::BinHeap<u64>;
    for i in vec![1,8,2,7,3,6,4,5] {
        bh.push(i);
    }
    while !bh.is_empty() {
        print!("{:?}\n", bh.pop());
    }
    print!("{:?}\n", bh.pop());

    // my-entry
    print!("\nown structure's min-heap\n");
    struct MyEntry {
        a: u64,
        b: String

    }
    impl std::fmt::Debug for MyEntry {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MyEntry").field("a", &self.a).field("b", &self.b).finish()
        }
    }
    impl Ord for MyEntry {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.a.cmp(&other.a)
        }
    }
    impl PartialOrd for MyEntry {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl PartialEq for MyEntry {
        fn eq(&self, other: &Self) -> bool {
            self.a == other.a
        }
    }
    impl Eq for MyEntry { }
    let mut bh = binheap::BinHeap::new() as binheap::BinHeap<MyEntry>;
    for i in vec![1,8,2,7,3,6,4,5] {
        bh.push(MyEntry{a:i, b:i.to_string()});
    }
    while !bh.is_empty() {
        print!("{:?}\n", bh.pop());
    }
    print!("{:?}\n", bh.pop());

    // into_vec
    let mut bh = binheap::BinHeap::new() as binheap::BinHeap<u64>;
    for i in vec![1,8,2,7,3,6,4,5] {
        bh.push(i);
    }
    print!("\nVec<u64> {:?}\n", bh.into_vec());

    exit(0);
}
```

---

## shell

```shell
% cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `/private/tmp/test_binheap/target/debug/test_binheap`

min-heap
Some(1)
Some(2)
Some(3)
Some(4)
Some(5)
Some(6)
Some(7)
Some(8)
None

max-heap
Some(8)
Some(7)
Some(6)
Some(5)
Some(4)
Some(3)
Some(2)
Some(1)
None

own structure's min-heap
Some(MyEntry { a: 1, b: "1" })
Some(MyEntry { a: 2, b: "2" })
Some(MyEntry { a: 3, b: "3" })
Some(MyEntry { a: 4, b: "4" })
Some(MyEntry { a: 5, b: "5" })
Some(MyEntry { a: 6, b: "6" })
Some(MyEntry { a: 7, b: "7" })
Some(MyEntry { a: 8, b: "8" })
None

Vec<u64> [1, 3, 2, 5, 7, 6, 4, 8]
```

