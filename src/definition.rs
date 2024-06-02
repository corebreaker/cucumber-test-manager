use cucumber::{codegen::WorldInventory, World as CucumberWorld};
use std::fmt::Debug;

/// A trait that defines a Cucumber test.
pub trait CucumberTest: CucumberWorld + WorldInventory + Debug {
    /// The name of the test.
    ///
    /// This is used to find the feature file.
    const NAME: &'static str;
}
