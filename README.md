[![Crates.io]](https://crates.io/crates/cucumber-trellis)
[![Docs.rs]](https://docs.rs/cucumber-trellis/)
[![CircleCI]](https://circleci.com/gh/corebreaker/cucumber-trellis/tree/main)
[![Coverage Status]](https://coveralls.io/github/corebreaker/cucumber-trellis?branch=main)

# Cucumber Trellis
Create a test "trellis" for Cucumber.

You can create a test suite for Cucumber, with each test implemented in a file,
linked to a Gherkin feature file.

Each test implements a trait `CucumberTest` and this test is registered in the "trellis".

Finally, all tests are executed in parallel, and the trellis waits for all tests to finish.

## Installation
```bash
cargo add cucumber cucumber-trellis
```

## Usage
First, allows Cucumber to print output instead of libtest
by adding these lines in your `Cargo.toml`:
```toml
[[test]]
name = "cucumber"
harness = false
```

Then, put feature files in `tests/features` directory,
and put the following code in `tests/cucumber.rs`:
```rust
mod tests;

fn main() {
    let mut trellis = cucumber_trellis::CucumberTrellis::new(None);

    trellis.add_test::<tests::example::SimpleTest>();

    trellis.run_tests();
}
```

After that , in `tests/tests/example.rs`,
implements the trait `cucumber_trellis::CucumberTest`, like this:
```rust
use cucumber_trellis::CucumberTest;
use cucumber::World;

#[derive(World, Debug)]
pub(in super::super) struct SimpleTest;

impl CucumberTest for SimpleTest {
    const NAME: &'static str = "simple-test";
}

// Implement here, the steps according the file `tests/features/simple-test.feature`
// ...
```

Don't forget the file `tests/tests/mod.rs`:
```rust
pub(super) mod example;
```

Finally, run the tests:
```bash
cargo test --test cucumber
```

## Example
You have an example in the `tests` directory.

## Macro
You can use the macro `cucumber_trellis::cucumber_test` to simplify the code.

The macro `cucumber_test` will decorate your function that will add Cucumber tests.
That function will receive a mutable reference to a `CucumberTrellis` object:
```rust
use cucumber_trellis::{CucumberTrellis, cucumber_test};

#[cucumber_test(features="tests/features", executor="futures::executor::block_on", use_tokio)]
fn my_tests(trellis: &mut CucumberTrellis) {
    trellis.add_test::<tests::example::SimpleTest>();
}
```

Currently, the macro `cucumber_test` can receive the following parameters:
- `features`: The path to the features directory, here for example `tests/features`.
- `executor`: The path of a function to execute asyncs.
- `use_tokio`: The generated function `main` will use the `tokio` runtime;
Tokio should be added in `dev-dependencies`.

These parameters are optional, and the parameters `use_tokio` and `executor` are mutually exclusive,
as `use_tokio` forces the use of the `tokio` as executor.
So **the example above won't compile**, it's just to show the usage.

The default executor is `futures::executor::block_on`,
therefore the dependency `futures` must be added in `Cargo.toml` if no executor is specified in parameters.


[Docs.rs]: https://img.shields.io/docsrs/cucumber-trellis?style=for-the-badge
[Crates.io]: https://img.shields.io/crates/v/cucumber-trellis?style=for-the-badge
[CircleCI]: https://img.shields.io/circleci/build/github/corebreaker/cucumber-trellis/main?style=for-the-badge
[Coverage Status]: https://img.shields.io/coveralls/github/corebreaker/cucumber-trellis?style=for-the-badge
