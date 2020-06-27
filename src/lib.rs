#![cfg(test)]

use std::collections::HashMap;
use std::sync::Arc;
use std::{borrow::Borrow, sync::Mutex};

use lazy_static::lazy_static;
use rstest::*;

struct Session {
    user: String,
}

impl Session {
    fn new(user: &str) -> Self {
        Self {
            user: dbg!(user.to_owned()),
        }
    }
}

#[fixture]
fn simple_session() -> &'static Session {
    lazy_static! {
        /// This is an example for using doc comment attributes
        static ref SESSIONS: Session = Session::new("other_simple");
    }
    &SESSIONS
}

#[fixture(user = "user")]
fn logged_in(user: &str) -> Arc<Session> {
    lazy_static! {
        /// This is an example for using doc comment attributes
        static ref SESSIONS: Mutex<HashMap<String, Arc<Session>>> = Default::default();
    }
    SESSIONS
        .lock()
        .unwrap()
        .entry(user.to_owned())
        .or_insert_with(move || Session::new(user).into())
        .clone()
}

#[rstest]
fn first_test(logged_in: impl Borrow<Session>) {
    assert_eq!(logged_in.borrow().user, "user");
}

#[rstest]
fn first_test2(logged_in: impl Borrow<Session>) {
    assert_eq!(logged_in.borrow().user, "user");
}

#[rstest(
    logged_in("three"),
    step => [1, 2, 3]
)]
fn three_times_uno_user(logged_in: impl Borrow<Session>, step: u32) {
    dbg!(step);
    assert_eq!(logged_in.borrow().user, "three");
}

#[rstest(logged_in("three"))]
fn use_user_three_again(logged_in: impl Borrow<Session>) {
    assert_eq!(logged_in.borrow().user, "three");
}

#[rstest]
fn other_test(simple_session: impl Borrow<Session>) {
    assert_eq!(simple_session.borrow().user, "other_simple");
}

#[rstest]
fn other_test2(simple_session: impl Borrow<Session>) {
    assert_eq!(simple_session.borrow().user, "other_simple");
}
