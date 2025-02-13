//! Personal crate for leetcode problem solutions.

/// Generate a set of parametrized test cases for a target function.
///
/// Usage:
///
/// ```no_run
/// # use leetcode::param_test;
/// # mod path { pub mod to { pub fn target_fn(a: &[i32], b: Vec<&str>) -> i32 { loop {} } } }
/// param_test! {
///     // Test target specification:
///     // - Name for the generated test driver function (can be arbitrary)
///     // - Name (or path) of the target function (the function under test)
///     // - Parameters to each test case (must have the same number and order
///     //   as parameters to the target function)
///     // - Type of test case expected values
///     test_driver<path::to::target_fn>(
///         target_param1: &[i32],
///         target_param2: &str => {
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
/// `test_runner` can be an arbitrary identifier; it will be used as the name of
/// the parameterized function that handles the various test cases.
///
/// The expected value can also have an optional conversion, given by the
/// following syntax:
///
/// ```no_run
/// # use leetcode::param_test;
/// # fn target_fn(a: &[i32]) -> Vec<i32> { loop {} }
/// param_test! {
///     test_driver<target_fn>(param: &[i32]) -> &[i32] => |expect| {
///         Vec::from(expect)
///     } {
///         case1(&[45, 46, 47]) => &[18],
///     }
/// }
/// ```
///
/// If provided, the expected value conversion will be evaluated on the expected
/// value as specified in a test case before the result of the target function is
/// compared to it. Note that in the above example this conversion is entirely
/// unneccessary, since `&[i32]` can be compared in an `assert_eq!` with a
/// `Vec<i32>` with no need for conversion.
///
/// Each test case is executed as follows:
/// - Evaluate the optional conversion for each parameter that has one;
/// - Call the target function, passing as arguments the results of calling
///   `.into()` on each of the given parameters (or on the result of their
///   optional conversion);
/// - If an expected value conversion was given, evaluate it on the given
///   expected value;
/// - Evaluate `assert_eq!` between the return value of the target function and
///   the (possibly converted) expected value.
#[macro_export]
macro_rules! param_test {
    (
        $test_fn:ident < $target_fn:path > (
            $($param:ident : $paramty:ty $(=> $paraminto:expr)?),* $(,)?
        ) -> $expectty:ty $(=> |$expect:ident| $expectinto:block)? {
            $(
                $case:ident($($arg:expr),* $(,)?) => $expectval:expr
            ),* $(,)?
        }
    ) => {
        fn $test_fn($($param: $paramty),* , expect: $expectty) {
            $(
                $(let $param = $paraminto;)?
            )*
            let res = $target_fn(
                $($param.into()),*
            );
            $(let expect = (|$expect: $expectty| $expectinto)(expect);)?
            assert_eq!(
                res,
                expect,
            );
        }

        $(
            #[test]
            fn $case() {
                $test_fn($($arg),*, $expectval);
            }
        )*
    };
}

pub mod problem1910;
pub mod problem2342;
pub mod problem2430;
pub mod problem3066;
pub mod problem3151;
pub mod problem3160;
pub mod problem3174;
