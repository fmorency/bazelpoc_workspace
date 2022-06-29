# Requirements

- [cargo-raze](https://github.com/google/cargo-raze)
- [Bazel](https://bazel.build/)

# Build

```
$ cd bazelpoc_workspace
$ cargo raze
$ bazel build //...:all-targets
```

# Tests

```
$ cd bazelpoc_workspace
$ cargo raze
$ bazel test //...:all-targets
```
