use cucumber::{given, then, when, World};
use cucumber_trellis::CucumberTest;

#[derive(World, Debug)]
#[world(init = Self::new)]
pub(in super::super) struct SimpleTest {
    number1: Option<i32>,
    number2: Option<i32>,
    result: Option<i32>,
}

impl SimpleTest {
    fn new() -> Self {
        Self {
            number1: None,
            number2: None,
            result: None,
        }
    }
}

impl CucumberTest for SimpleTest {
    const NAME: &'static str = "simple-test";
}

#[given(expr = "we have the number `{int}` as the first number")]
fn i_have_the_first_number(world: &mut SimpleTest, num: i32) {
    world.number1.replace(num);
}

#[given(expr = "we have the number `{int}` as the second number")]
fn i_have_the_second_number(world: &mut SimpleTest, num: i32) {
    world.number2.replace(num);
}

#[when(expr = "we add them together")]
fn i_add_the_numbers_together(world: &mut SimpleTest) {
    match (world.number1, world.number2) {
        (Some(num1), Some(num2)) => {
            world.result.replace(num1 + num2);
        }
        r => panic!("numbers not set: {r:?}"),
    }
}

#[then(expr = "we should get `{int}` as the result")]
fn i_should_get(world: &mut SimpleTest, num: i32) {
    assert_eq!(world.result, Some(num));
}
