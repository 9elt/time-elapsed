# time-elapsed

A Rust crate that provides a concise and handy way to benchmark **elapsed time inside functions**.

# features:
* Name a benchmark
* Set a timestamp
* Print coloured messages
* Auto adjusts the unit of measurement

# example

```rust
use std::thread;
use std::time::Duration;

use time_elapsed;

fn main() {
    let mut time = time_elapsed::start("test");
    // running test...

    thread::sleep(Duration::from_millis(200));

    time
        .log("log() prints a message and the time elapsed")
        .timestamp();
    // (test) log() prints a message and the time elapsed -> 200 ms

    thread::sleep(Duration::from_millis(2));

    time.log("this is an offset from the previous timestamp()");
    // (test) this is an offset from the previous timestamp() -> 2103 μs

    time.log_overall("log_overall() ignores timestamps");
    // (test) log_overall() ignores timestamps -> 202 ms

    time.end();
    // test finished in 202 ms (202271 μs)
}
```
