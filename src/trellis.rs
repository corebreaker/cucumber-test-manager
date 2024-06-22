use super::{
    result::TestResult,
    spawners::{DefaultSpawner, TestSpawner},
    CucumberTest,
};

use futures::future::join_all;
use path_absolutize::Absolutize;
use std::{env::current_dir, path::PathBuf, rc::Rc};

pub struct CucumberTrellis {
    path_base: PathBuf,
    spawner: Rc<dyn TestSpawner>,
    tests: Vec<TestResult>,
}

impl CucumberTrellis {
    /// Create a new CucumberTrellis with the given path base for feature files.
    ///
    /// If the path base is not provided, the current directory is used,
    /// and the path base used will be `./tests/features`.
    pub fn new(feature_path_base: Option<PathBuf>) -> Self {
        let path_base = match feature_path_base {
            None => current_dir().unwrap().join("tests").join("features"),
            Some(p) => p.absolutize().unwrap().to_path_buf(),
        };

        if !path_base.exists() {
            panic!("Path does not exist: {}", path_base.display());
        }

        if !path_base.is_dir() {
            panic!("Path is not a directory: {}", path_base.display());
        }

        Self {
            path_base,
            spawner: Rc::new(DefaultSpawner),
            tests: Vec::new(),
            // no-coverage:start
        }
        // no-coverage:stop
    }

    /// Set the spawner for the trellis.
    pub fn with_spawner(mut self, spawner: impl TestSpawner + 'static) -> Self {
        self.spawner = Rc::new(spawner);
        self
    }

    /// Add a test to the trellis.
    pub fn add_test<T: CucumberTest>(&mut self) {
        let name = format!("{}.feature", T::NAME);
        let feature_path = self.path_base.join(&name);

        if !feature_path.exists() {
            panic!("Feature file does not exist: {}", feature_path.display());
        }

        self.tests.push(TestResult::new(T::run(feature_path)));
    }

    async fn all(self) {
        join_all(self.tests).await;
    }

    /// Run all tests in the trellis.
    pub fn run_tests(self) {
        let spawner = Rc::clone(&self.spawner);
        let result = TestResult::new(self.all());

        spawner.spawn(Box::pin(result));
    }
}

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;
    use cucumber::World;
    use std::{cell::Cell, rc::Rc};

    #[test]
    fn test_new_trellis_with_current_path() {
        let path_base = current_dir().unwrap().join("tests").join("features");
        let trellis = CucumberTrellis::new(None);

        assert_eq!(trellis.path_base, path_base, "path base should be `./tests/features`");
        assert!(trellis.tests.is_empty(), "no tests should be added");
    }

    #[test]
    fn test_new_trellis_with_any_path() {
        let path_base = current_dir().unwrap().join("tests").join("features");
        let trellis = CucumberTrellis::new(Some(path_base.clone()));

        assert_eq!(trellis.path_base, path_base, "path base should be `./tests/features`");
        assert!(trellis.tests.is_empty(), "no tests should be added");
    }

    #[test]
    fn test_with_spawner() {
        let trellis = CucumberTrellis::new(None).with_spawner(DefaultSpawner);
        let value = Rc::new(Cell::new(0_usize));

        {
            let value = Rc::clone(&value);
            let func = || async move {
                value.replace(42);
            };

            trellis.spawner.spawn(Box::pin(TestResult::new(func())));
        }

        assert_eq!(value.get(), 42);
    }

    #[test]
    #[should_panic]
    fn test_new_trellis_with_nonexistent_path() {
        CucumberTrellis::new(Some(PathBuf::from("!existent")));
    }

    #[test]
    #[should_panic]
    fn test_new_trellis_with_path_on_file() {
        CucumberTrellis::new(Some(PathBuf::from("Cargo.toml")));
    }

    #[test]
    fn test_add_test() {
        #[derive(World, Debug, Default)]
        pub(in super::super) struct SimpleTest;

        impl CucumberTest for SimpleTest {
            const NAME: &'static str = "simple-test";
        }

        let mut trellis = CucumberTrellis::new(None);
        trellis.add_test::<SimpleTest>();
    }

    #[test]
    #[should_panic]
    fn test_add_test_with_nonexistent_feature_file() {
        #[derive(World, Debug, Default)]
        pub(in super::super) struct NoFeatureTest;

        impl CucumberTest for NoFeatureTest {
            const NAME: &'static str = "!existent";
        }

        let mut trellis = CucumberTrellis::new(None);
        trellis.add_test::<NoFeatureTest>();
    }
}
// no-coverage:stop
