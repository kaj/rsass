# Testing rsass

There are two groups of tests, and the groups are subdirectories to
this directory.

* `spec` is the [sass-spec](https://github.com/sass/sass-spec) test
  suite, converted to rust by [a script](../../check_spec) in this
  repository.  Any test that did not work on the last conversion is
  marked as `#[ignore]`, so all other tests are expected to succeed.

* `misc` is any tests local to this implementation.

If you want to fix a bug in rsass, there should be a test exposing the
bug.  Hopefully, there is an ignored test in `spec`.  If so, remove
the `ignore` annotation and try to fix it.  If not, you can add a test
in `misc`.  If you add a new file, don't forget to `use` it as a
module in `misc/main.rs`.
