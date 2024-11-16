mod test_cases;

use test_cases::register_tests;
use std::time::Instant;

// Test Framework Core
pub struct Pizza {
    tests: Vec<Box<dyn PizzaTest>>,
}

impl Pizza {
    pub fn new() -> Self {
        Self { tests: Vec::new() }
    }

    pub fn add_test<T: PizzaTest + 'static>(&mut self, test: T) {
        self.tests.push(Box::new(test));
    }

    pub fn run(&self) {
        println!("🍕 Pizza Test Framework 🍕");
        let start_time = Instant::now();
        let mut passed = 0;
        let mut failed = 0;

        for (index, test) in self.tests.iter().enumerate() {
            let test_start = Instant::now();
            println!("Running test {}: {}", index + 1, test.name());

            test.setup();
            let result = test.run();
            test.teardown();
            let duration = test_start.elapsed();

            match result {
                TestResult::Pass => {
                    println!("\x1b[32m✅ {}: PASSED in {:?}\x1b[0m", test.name(), duration);
                    passed += 1;
                }
                TestResult::Fail(reason) => {
                    println!(
                        "\x1b[31m❌ {}: FAILED in {:?} - {}\x1b[0m",
                        test.name(),
                        duration,
                        reason
                    );
                    failed += 1;
                }
            }
        }

        let total_duration = start_time.elapsed();
        println!("\nTest Summary (total time: {:?}):", total_duration);
        println!(
            "Total: {}, \x1b[32mPassed: {}\x1b[0m, \x1b[31mFailed: {}\x1b[0m",
            self.tests.len(),
            passed,
            failed
        );
    }
}

// PizzaTest Trait
pub trait PizzaTest {
    fn name(&self) -> &'static str;

    fn setup(&self) {
        // Optional setup logic
    }

    fn run(&self) -> TestResult;

    fn teardown(&self) {
        // Optional teardown logic
    }
}

// Test Result Enum
pub enum TestResult {
    Pass,
    Fail(String),
}

// Assertion Helpers
pub fn assert_eq<T: PartialEq + std::fmt::Debug>(left: T, right: T) -> TestResult {
    if left == right {
        TestResult::Pass
    } else {
        TestResult::Fail(format!("Assertion failed: {:?} != {:?}", left, right))
    }
}

pub fn assert_true(condition: bool) -> TestResult {
    if condition {
        TestResult::Pass
    } else {
        TestResult::Fail("Assertion failed: condition is false".to_string())
    }
}

pub fn assert_false(condition: bool) -> TestResult {
    if !condition {
        TestResult::Pass
    } else {
        TestResult::Fail("Assertion failed: condition is true".to_string())
    }
}

// Test Registration Macro
#[macro_export]
macro_rules! pizza_test {
    ($test_name:ident, $test_block:block) => {
        pub struct $test_name;

        impl PizzaTest for $test_name {
            fn name(&self) -> &'static str {
                stringify!($test_name)
            }

            fn run(&self) -> TestResult {
                let result = std::panic::catch_unwind(|| $test_block);
                match result {
                    Ok(TestResult::Pass) => TestResult::Pass,
                    Ok(TestResult::Fail(reason)) => TestResult::Fail(reason),
                    Err(_) => TestResult::Fail("Panic occurred".into()),
                }
            }
        }
    };
}

fn main() {
    let mut pizza = Pizza::new();
    register_tests(&mut pizza);
    pizza.run();
}
