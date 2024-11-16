if get_option('with_test').enabled()
    test_cases = [
        'test_sample.rs',
    ]

    pizza_rs = executable('testbed_rs', test_cases, include_directories: dir, dependencies: [pizza_test_dep])

    test('pizza testing Rust', pizza_rs)
endif