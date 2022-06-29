load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("//remote/cargo_workspace/cargo:crates.bzl", "crate_deps")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "p1",
    srcs = ["src/main.rs"],
    deps = crate_deps([
        "minicbor",
    ]),
)

rust_test(
    name = "foo",
    srcs = ["tests/foo.rs"],
)
