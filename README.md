# time-elapsed

A Rust crate that provides a concise and handy way to benchmark **elapsed time inside functions**.
> time-elapsed brings a small overhead, however, if you are trying to measure very small durations (in the order of *nanoseconds* or few *microseconds*), please consider something else.

## installation
Add the following to Cargo.toml
```
[dependencies]
time-elapsed = "0.1.0"
```

# features
* named benchmark
* timestamps
* coloured messages
* auto unit of measurement

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
(<a href="#output">test</a>) <b>log() prints a message and the time elapsed</b> -> <b><a href="#output">200 ms</a></b>
(<a href="#output">test</a>) <b>this is an offset from the previous timestamp()</b> -> <b><a href="#output">2103 μs</a></b>
(<a href="#output">test</a>) <b>log_overall() ignores timestamps</b> -> <b><a href="#output">202 ms</a></b>
<b><a href="#output">test finished</a></b> in <b><a href="#output">202 ms</a></b> (202271 μs)
</pre>
