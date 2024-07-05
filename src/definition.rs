use cucumber::{codegen::WorldInventory, Cucumber, parser, runner, Writer, World as CucumberWorld};
use std::{fmt::Debug, path::PathBuf};

type CucumberConfig<World, Writer> = Cucumber<World, parser::Basic, PathBuf, runner::Basic<World>, Writer>;

/// A trait that defines a Cucumber test.
pub trait CucumberTest: CucumberWorld + WorldInventory + Debug {
    /// The name of the test.
    ///
    /// This is used to find the feature file.
    const NAME: &'static str;

    fn config<W: Writer<Self>>(_cucumber: &mut CucumberConfig<Self, W>) {}
}
