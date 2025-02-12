/// Usage:
///
/// ```
/// param_test! {
///     // Test target signature:
///     // - Name for the generated test driver function (can be arbitrary)
///     // - Name (or path) of the target function
///     // - Parameters to each test case (should match up with parameters of target function)
///     // - Type of test case expected values (should be comparable to return value of target function)
///     test_driver<path::to::target_fn>(
///         target_param1: &[i32],
///         target_param2: &str {
///             // Optional conversion from ParamType to the actual type expected by the target function
///             let split = target_param2.split_whitespace();
///             split.collect::<Vec<_>>()
///         },
///     ) -> i32 {
///         // Individual test cases:
///         // - Test name
///         // - Test parameters
///         // - Expected result
///         test_case1(&[1, 2, 3], "lol hey") => 4,
///         test_case2(&[], "nope!") => 0,
///     }
/// }
/// ```
///
/// The parameters given for each test case should match up one-to-one with
/// the parameters of the target function. They do not need to have the exact same
/// types however; before passing the test case parameters to the target function,
/// the generated tests will call `.into()` on each of them. If a more sophisticated
/// type conversion is required, each parameter can be followed by a block
/// expression, making use of the parameter name, to map the test case parameter
/// into the type expected by the target function.
///
/// Each test case is executed as follows:
/// - Evaluating the optional conversion for given parameter that has one;
/// - Calling the target function, passing `param.into()` for each of the given
///   parameters (or the result of their optional conversion);
/// - Evaluating `assert_eq!` between the result, and the expected value given
///   after the `=>` token for that case.
///
/// `test_runner` can be an arbitrary identifier, and will be used as the name of the
/// parameterized function that handles the various test cases.
#[cfg(test)]
macro_rules! param_test {
    (
        $test_fn:ident < $target_fn:path > (
            $($param:ident : $paramty:ty $($paraminto:block)?),* $(,)?
        ) -> $retty:ty {
            $(
                $case:ident($($arg:expr),* $(,)?) => $expect:expr
            ),* $(,)?
        }
    ) => {
        fn $test_fn($($param: $paramty),* , expect: $retty) {
            $(
                $(let $param = $paraminto;)?
            )*
            assert_eq!(
                $target_fn(
                    $(
                        ($param).into()
                    ),*
                ),
                expect,
            );
        }

        $(
            #[test]
            fn $case() {
                $test_fn($($arg),*, $expect);
            }
        )*
    };
}

pub mod problem1910;
pub mod problem2342;
pub mod problem2430;
pub mod problem3151;
pub mod problem3160;
pub mod problem3174;
