mod tests;

fn main() {
    let mut trellis = cucumber_trellis::CucumberTrellis::new(None);

    trellis.add_test::<tests::example::SimpleTest>();

    trellis.run_tests();
}
