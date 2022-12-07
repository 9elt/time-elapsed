# time-elapsed

A Rust crate that provides a concise and handy way to benchmark **elapsed time inside functions**.

# features:
* the benchmark is named
* can set a timestamp
* prints coloured messages
* auto adjusts the unit of measurement

# example
### code
```rust
use std::thread;
use std::time::Duration;

use time_elapsed;

fn main() {
    let mut time = time_elapsed::start("test");

    // sleep 200 ms
    thread::sleep(Duration::from_millis(200));

    time
        .log("log() prints a message and the time elapsed")
        .timestamp();

    // sleep 2 ms
    thread::sleep(Duration::from_millis(2));

    time.log("this is an offset from the previous timestamp()");

    time.log_overall("log_overall() ignores timestamps");

    time.end();
}
```
### output
<pre>
running test...
(<b>test</b>) log() prints a message and the time elapsed -> <b>200 ms</b>
(<b>test</b>) this is an offset from the previous timestamp() -> <b>2103 μs</b>
(<b>test</b>) log_overall() ignores timestamps -> <b>202 ms</b>
<b>test finished</b> in <b>202 ms</b> (202271 μs)
</pre>
