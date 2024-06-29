mod tests;

fn main() {
    let mut trellis = cucumber_trellis::CucumberTrellis::new(None);

    trellis.add_test::<tests::example::SimpleTest>();

    futures::executor::block_on(async {
        trellis.run_tests().await;
    });
}
