use super::{result::TestResult, CucumberTest};
use futures::future::join_all;
use path_absolutize::Absolutize;
use std::{
    path::{PathBuf, Path},
    env::current_dir,
};

pub struct CucumberTrellis {
    path_base: PathBuf,
    tests: Vec<TestResult>,
}

impl CucumberTrellis {
    /// Create a new CucumberTrellis with the given path base for feature files.
    ///
    /// If the path base is not provided, the current directory is used,
    /// and the path base used will be `./tests/features`.
    pub fn new(feature_path_base: Option<&Path>) -> Self {
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
            tests: Vec::new(),
            // no-coverage:start
        }
        // no-coverage:stop
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

    /// Run all tests in the trellis.
    pub async fn run_tests(self) {
        join_all(self.tests).await;
    }
}

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;
    use cucumber::World;

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
        let trellis = CucumberTrellis::new(Some(&path_base));

        assert_eq!(trellis.path_base, path_base, "path base should be `./tests/features`");
        assert!(trellis.tests.is_empty(), "no tests should be added");
    }

    #[test]
    #[should_panic]
    fn test_new_trellis_with_nonexistent_path() {
        CucumberTrellis::new(Some(&PathBuf::from("!existent")));
    }

    #[test]
    #[should_panic]
    fn test_new_trellis_with_path_on_file() {
        CucumberTrellis::new(Some(&PathBuf::from("Cargo.toml")));
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
