# `rstest` example on how to deal with static cache fixture

Lot of time peaploe need a `fixture` that provide some static value that we want to 
build just one time for each tests. In other words you are looking for a `lazy` `static`
fixture. Ok we should just add an `_` and we are done :).

Sometime is just this and sometime not. If we need also a cache you should add also a 
`Mutex` (tests by default is running in parallel mode) and `Arc` to deal with lifetime because
nobody can give some warrant to compiler that the cache entry will not removed till test 
is running.

So this crate provide an example where `simple_session` is the simple case and `logged_in` is the cache one.

If you run test you can see that `Session::new()` is called just one time for each username in all 8 tests:

```textmichele@DartVader:~/sviluppo/rstest_fixture_cache_example$ cargo test -- --nocapture 2>&1 | grep "user.to_owned()"
[src/lib.rs:17] user.to_owned() = "user"
[src/lib.rs:17] user.to_owned() = "other_simple"
[src/lib.rs:17] user.to_owned() = "three"
```

Let me know if it is clear or not.