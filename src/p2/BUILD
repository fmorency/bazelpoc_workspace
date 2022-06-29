load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//remote/cargo_workspace/cargo:crates.bzl", "crate_deps")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "p2",
    srcs = ["src/main.rs"],
    deps = crate_deps([
        "tracing",
        "tracing-subscriber",
    ]),
)
