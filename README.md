# Num Stream
![Latest Version](https://img.shields.io/crates/v/num_stream)
![License](https://img.shields.io/crates/l/num_stream)
![Downloads](https://img.shields.io/crates/d/num_stream)

This crate provides a [Stream](https://docs.rs/futures/0.3.4/futures/stream/trait.Stream.html) that yields
numeric values at a specifed interval and that increments at a specifed rate.

[API Documentation](https://docs.rs/num_stream/0.1.1/num_stream/)

[crates.io](https://crates.io/crates/num_stream)

---

This crate was born out of the desire for a simple configurable stream that would yield ever changing, 
yet predictable values. 

The [num_stream](https://docs.rs/num_stream/0.1.0/num_stream/fn.num_stream.html) method can be used to
acquire an instance of a [NumStream](https://docs.rs/num_stream/0.1.0/num_stream/struct.NumStream.html) 
struct which implements a Futures 0.3 Stream. 

#### Example
```rust
use futures::stream::StreamExt;
use num_stream::num_stream;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    let mut nums = num_stream(0, 3, Duration::from_millis(500));
    loop {
        println!("Got: {:?}", nums.next().await);
    }
}
```

#### License

<sup>
Licensed under <a href="LICENSE">Apache License, Version
2.0</a>
</sup>
