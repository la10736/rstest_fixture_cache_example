# `rstest` example on how to deal with static cache fixture

Lot of time people need a `fixture` that provide some static value that we want to 
build just one time for each tests. In other words you are looking for a `lazy` `static`
fixture. Ok we should just add an `_` and we are done :).

Sometimes is just this and sometimes not. If you need also a cache you should use a 
`Mutex` to deal with concurrency (tests by default is running in parallel mode) and 
`Arc` to deal with lifetime because nobody can give some warrant to compiler that the 
cache entry will not removed till test is running.

So this crate provide three fixture example

1. `simple_session` that just return a reference to a static allocated value: all test that use 
this fixture will share the same reference and the value is allocated just one time at the first
request
2. `logged_in` that cache a `Session` for each `user` and return a reference to it: all tests that
ask a session for the same use will share the same reference
3. `mutable_session` that return _something that can be referenced as mutable `Session`_: all tests 
that use this fixture just run in sequence because all of them need a mutable reference to the same
`Session`

If you run test you can see that `Session::new()` is called just one time for each username in all 
11 tests and `visit` is called 3 times on `mutable`:

```
michele@DartVader:~/sviluppo/rstest_fixture_cache_example$ cargo test -- --nocapture 2>&1 | grep -e "user.to_owned()" -e "self.visit"
[src/lib.rs:18] user.to_owned() = "user"
[src/lib.rs:18] user.to_owned() = "mutable"
[src/lib.rs:25] self.visit = 1
[src/lib.rs:25] self.visit = 2
[src/lib.rs:25] self.visit = 3
[src/lib.rs:18] user.to_owned() = "other_simple"
[src/lib.rs:18] user.to_owned() = "three"
```

Let me know if it is clear or not.