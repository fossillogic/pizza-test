dir = include_directories('.')

test_code = ['testing.rs']

pizza_test_lib = library('pizza_test',
    test_code,
    install: true,
    include_directories: dir)

pizza_test_dep = declare_dependency(
    link_with: pizza_test_lib,
    include_directories: dir)