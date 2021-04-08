# WIP!!!

# libseat-rs

```rust
let seat = Seat::open(
    |seat| {
        println!("Enable");
        println!("Name: {}", seat.name());
    },
    |seat| {
        println!("Disable");
        seat.disable().unwrap();
    },
);
```
