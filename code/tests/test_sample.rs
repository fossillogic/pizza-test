use crate::{assert_eq, assert_false, assert_true, pizza_test, Pizza};

pizza_test!(TestAddition, {
    assert_eq(2 + 2, 4)
});

pizza_test!(TestConditionTrue, {
    assert_true(3 > 2)
});

pizza_test!(TestConditionFalse, {
    assert_false(1 > 2)
});

pizza_test!(TestFailure, {
    assert_eq(1 + 1, 3) // This test will fail
});

pub fn register_tests(pizza: &mut Pizza) {
    pizza.add_test(TestAddition);
    pizza.add_test(TestConditionTrue);
    pizza.add_test(TestConditionFalse);
    pizza.add_test(TestFailure);
}
