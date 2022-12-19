//! A Rust crate that provides a concise and handy way to benchmark **elapsed time inside functions**.
//! > time-elapsed brings a small overhead, however, if you are trying to measure very small durations (in the order of *nanoseconds* or few *microseconds*), please consider using something else.
//!
//! ### installation
//! Add the following to Cargo.toml
//! ```
//! [dependencies]
//! time-elapsed = "0.1.0"
//! ```
//! 
//! # features
//! * named benchmark
//! * timestamps
//! * coloured messages
//! * auto unit of measurement
//! 
//! # example
//! 
//! ### code
//! 
//! ```
//! use std::thread;
//! use std::time::Duration;
//! 
//! use time_elapsed;
//! 
//! fn main() {
//!     let mut time = time_elapsed::start("test");
//! 
//!     // sleep 200 ms
//!     thread::sleep(Duration::from_millis(200));
//! 
//!     time
//!         .log("log() prints a message and the time elapsed")
//!         .timestamp();
//! 
//!     // sleep 2 ms
//!     thread::sleep(Duration::from_millis(2));
//! 
//!     time.log("this is an offset from the previous timestamp()");
//! 
//!     time.log_overall("log_overall() ignores timestamps");
//! 
//!     time.end();
//! }
//! ```
//! 
//! ### output
//! 
//! ```console
//! running test...
//! (test) log() prints a message and the time elapsed -> 200ms
//! (test) this is an offset from the previous timestamp() -> 2103 μs
//! (test) log_overall() ignores timestamps -> 202 ms
//! test finished in 202 ms (202271 μs)
//! ```
//!

use std::time::Instant;

/// Starts the benchmark by returning an initialized instance of **TimeElpased**.
/// 
/// # example
/// 
/// ```
/// let mut time = time_elapsed::start("test");
/// // output: running test...
/// ```
pub fn start<S: AsRef<str>>(name: S) -> TimeElapsed {
    TimeElapsed::new(name.as_ref())
}

fn get_unit_of_measurement(nanos: u128) -> &'static str {
    match nanos / 4000000 {
        0 => "μs",
        _ => match nanos / 15000000000 {
            0 => "ms",
            _ => match nanos / 300000000000 {
                0 => "s",
                _ => match nanos / 540000000000 {
                    0 => "min",
                    _ => "hrs",
                },
            },
        },
    }
}

fn get_units_of_measurement(nanos: u128) -> [&'static str; 2] {
    match get_unit_of_measurement(nanos) {
        "μs" => ["μs", "ns"],
        "ms" => ["ms", "μs"],
        "s" => ["s", "ms"],
        "min" => ["min", "s"],
        "hrs" => ["hrs", "min"],
        _ => ["ns", "ns"],
    }
}

fn nanos_to_unit_of_msr(nanos: u128, unit_of_msr: &str) -> u128 {
    match unit_of_msr {
        "μs" => nanos / 1000,
        "ms" => nanos / 1000000,
        "s" => nanos / 1000000000,
        "min" => nanos / 60000000000,
        "hrs" => nanos / 3600000000000,
        _ => nanos,
    }
}

fn nanos_to_units_of_msr(nanos: u128, unit_of_msr: &str) -> [u128; 2] {
    match unit_of_msr {
        "μs" => [nanos / 1000, nanos],
        "ms" => [nanos / 1000000, nanos / 1000],
        "s" => [nanos / 1000000000, nanos / 1000000],
        "min" => [nanos / 60000000000, nanos / 1000000000],
        "hrs" => [nanos / 3600000000000, nanos / 60000000000],
        _ => [nanos, nanos],
    }
}

/// Stores the benchmark state and provides methods (timestamp method needs a mutable reference).
/// 
/// To create an initialized instance use the **time_elapsed::start** function.
/// 
/// # example
/// 
/// ```
/// let mut time = time_elapsed::start("test");
/// // output: running test...
/// 
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TimeElapsed {
    name: String,
    start_timestamp: Instant,
    last_timestamp: Instant,
}

impl TimeElapsed {

    fn new(name: &str) -> Self {
        println!("running {}...", name);
        Self {
            name: name.to_string(),
            start_timestamp: Instant::now(),
            last_timestamp: Instant::now(),
        }
    }

    fn print_message(&mut self, msg: &str, nanos: u128) -> &Self {
        let unit = get_unit_of_measurement(nanos);
        let time = nanos_to_unit_of_msr(nanos, unit);
        println!(
            "(\x1b[32m\x1b[1m{}\x1b[0m) \x1b[1m{} \x1b[0m-> \x1b[35m\x1b[1m{} {} \x1b[0m",
            self.name, msg, time, unit
        );
        self
    }

    /// Ends the benchmark. Outputs the total elapsed time from the start
    /// of the benchmark.
    /// 
    /// # example
    /// 
    /// ```
    /// let mut time = time_elapsed::start("test");
    /// // output: running test...
    /// 
    /// time.end();
    /// // output: test finished in 1 μs (1204 ns)
    /// 
    /// ```
    pub fn end(self) {
        let nanos = self.start_timestamp.elapsed().as_nanos();
        let units = get_units_of_measurement(nanos);
        let times = nanos_to_units_of_msr(nanos, units[0]);
        println!(
            "\x1b[32m\x1b[1m{} finished\x1b[0m in \x1b[35m\x1b[1m{} {} \x1b[0m({} {})",
            self.name, times[0], units[0], times[1], units[1],
        );
    }

    /// Outputs a message followed by the **elapsed time** from the **previous timestamp**.
    /// 
    /// Returns a mutable reference of self.
    /// 
    /// # example
    /// 
    /// ```
    /// let mut time = time_elapsed::start("test");
    /// // output: running test...
    /// 
    /// time.log("My message");
    /// // output: (test) My message -> 1 μs
    /// 
    /// ```
    pub fn log<S: AsRef<str>>(&mut self, msg: S) -> &mut Self {
        let nanos = self.last_timestamp.elapsed().as_nanos();
        self.print_message(msg.as_ref(), nanos);
        self
    }

    /// Outputs a message followed by the **elapsed time** from the **start**, ignoring timestamps.
    /// 
    /// Returns a mutable reference of self.
    /// 
    /// # example
    /// 
    /// ```
    /// use std::thread;
    /// use std::time::Duration;
    /// 
    /// let mut time = time_elapsed::start("test");
    /// // output: running test...
    /// 
    /// thread::sleep(Duration::from_millis(200));
    /// 
    /// time.timestamp();
    /// time.log_overall("The elapsed time from the start");
    /// // output: (test) The elapsed time from the start -> 200 ms
    /// 
    /// ```
    pub fn log_overall<S: AsRef<str>>(&mut self, msg: S) -> &mut Self {
        let nanos = self.start_timestamp.elapsed().as_nanos();
        self.print_message(msg.as_ref(), nanos);
        self
    }

    /// Updates and returns the last timestamp.
    /// 
    /// # example
    /// 
    /// ```
    /// use std::thread;
    /// use std::time::Duration;
    /// let mut time = time_elapsed::start("test");
    /// // output: running test...
    /// 
    /// thread::sleep(Duration::from_millis(200));
    /// 
    /// time.timestamp();
    /// 
    /// time.log("Elapsed time from the prev timestamp");
    /// // output: (test) Elapsed time from the prev timestamp -> 1 μs
    /// 
    /// ```
    pub fn timestamp(&mut self) -> Instant {
        self.last_timestamp = Instant::now();
        self.last_timestamp
    }
}
