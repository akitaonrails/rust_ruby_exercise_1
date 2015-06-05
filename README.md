## Simple Rust lib consumed by Ruby via FFI

This test is intended as a simple exercise. The intentions:

* to make Rust consume a text file and parse through it, line by line
* to not load the entire file in memory before parsing (I/O streaming, line by line)
* to test the currently available Regex support
* to test how to expose such a function for FFI comsumption
* to benchmark a similar Ruby solution

So far, be careful:

* the Regex library seems to be slow so far
* it's so slow that the Rust solution is actually a bit slower than the Ruby one (!!)
* I might be making basic mistakes in the Rust code (still just 5 days into studying)

Running Rust 1.0 and Ruby 2.2.2, this is the benchmark with a 1_000_000 slice
from the actors.list file:

```
RUST: 7.270000   0.070000   7.340000 (  7.363527)
RUBY: 6.020000   0.030000   6.050000 (  6.082596)
```

I believe that the regex lib is the culprit at this point, but not sure how to profile.

If I'm making obvious mistakes, a pull request will be really appreciated.

To run the test:

```
cargo build --release
cargo run --release
```

Or to run via Ruby+FFI:

```
RUST=1 ruby actors.rb
```
