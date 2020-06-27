#![feature(prelude_import)]
#![cfg(test)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rstest::*;
struct Session {
    user: String,
}
impl Session {
    fn new(user: String) -> Self {
        Self { user }
    }
}
#[allow(non_camel_case_types)]
struct logged_in {}
impl logged_in {
    #[allow(unused_mut)]
    pub fn get() -> &'static Session {
        logged_in()
    }
    pub fn default() -> &'static Session {
        Self::get()
    }
}
#[allow(dead_code)]
fn logged_in() -> &'static Session {
    let user = "user".to_owned();
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    #[doc = r" This is an example for using doc comment attributes"]
    struct SESSIONS {
        __private_field: (),
    }
    #[doc(hidden)]
    static SESSIONS: SESSIONS = SESSIONS {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for SESSIONS {
        type Target = Mutex<HashMap<String, Session>>;
        fn deref(&self) -> &Mutex<HashMap<String, Session>> {
            #[inline(always)]
            fn __static_ref_initialize() -> Mutex<HashMap<String, Session>> {
                Default::default()
            }
            #[inline(always)]
            fn __stability() -> &'static Mutex<HashMap<String, Session>> {
                static LAZY: ::lazy_static::lazy::Lazy<Mutex<HashMap<String, Session>>> =
                    ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for SESSIONS {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    let s = SESSIONS
        .lock()
        .unwrap()
        .entry(user)
        .or_insert_with(Session::new(user));
    return s;
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const first_test: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("first_test"),
        ignore: false,
        allow_fail: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::UnitTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(first_test())),
};
fn first_test() {
    fn first_test(logged_in: &Session) {
        {
            match (&logged_in.user, &"user") {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&&*left_val, &&*right_val) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                    ],
                                },
                            ))
                        }
                    }
                }
            }
        };
    }
    let logged_in = logged_in::default();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
            &["", "\n"],
            &match (&" TEST START ",) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
            &[::core::fmt::rt::v1::Argument {
                position: 0usize,
                format: ::core::fmt::rt::v1::FormatSpec {
                    fill: '-',
                    align: ::core::fmt::rt::v1::Alignment::Center,
                    flags: 0u32,
                    precision: ::core::fmt::rt::v1::Count::Implied,
                    width: ::core::fmt::rt::v1::Count::Is(40usize),
                },
            }],
        ));
    };
    first_test(logged_in)
}
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&first_test])
}
