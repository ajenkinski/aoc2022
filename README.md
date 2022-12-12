# Advent of Code solutions for 2022

Solutions for Advent of Code challenges for 2022, implemented in Rust

https://adventofcode.com/2022

# Running

Install rust if you haven't already.  Follow the instructions here to install the Rust toolchain:

https://www.rust-lang.org/tools/install

Make sure input files are downloaded by running 

```
AOC_SESSION=<session_id> ./download-inputs.sh
```

where `<session_id>` is the value of the "session" cookie for your logged in adventofcode.com session.  You can
find the cookie value by looking in the developer tools for your browser for a cookie named "session".

To run the solution for a specific day, run

```
cargo run -r --bin day1
```

See the contents of src/bin to see which days have solutions implemented.
