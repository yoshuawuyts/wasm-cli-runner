use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::process::Command;

fn cli() -> Command {
    Command::new(get_cargo_bin("wasm-cli-runner"))
}

#[test]
fn has_a_version() {
    assert_cmd_snapshot!(cli().arg("--version"));
}

#[test]
fn path_is_required() {
    assert_cmd_snapshot!(cli());
}

#[test]
fn path_is_reachable_on_disk() {
    assert_cmd_snapshot!(cli().arg("noop.wasm"));
}

#[test]
fn path_points_to_a_wasm_binary() {
    assert_cmd_snapshot!(cli().arg("tests/binaries/time.wasm"));
}
