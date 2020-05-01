# 要格式解决方案

在解决方案目录中使用 

    cargo fmt

要查看解决方案是否包含一些常见的无效用例，在解决方案目录中使用

    cargo clippy --all -targets

# rust 中的测试

Execute the tests with:

    cargo test

All but the first test have been ignored. After you get first test to pass,
open the tests source file which is located in the tests directory and remove
the #[ignore] flag from he next test and get the test to pass again. Each separate test
is a function with #[test] flag above it. Continue, until you pass every test.

if you wish to run all ignored tests without editing the test source file, use:

    cargo test -- --ignored

To run a specifi test, for example some_test, you can use:

    cargo test some_test

If the specific test is ignored use:

    cargo test some_test -- --ignored

To learn more about Rust tests refer to the online test documentation 
Make sure to read the Modules chapter if you haven't already, it will 
help you with originizing you files.

# Leap

Given a year, report if it a leap year.

The tricky thing here is that a leap year in the Gregorian calendar occurs:

on every year that is evenly divisiable by 4
    except every year that is evenly divisible by 100
        unless the year is also evenly divisible by 400


For exaple. 1997 is not a leap year, but 1996 is, 1900 is not a leap year, but 2000 is.
