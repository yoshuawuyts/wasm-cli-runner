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
fn path_ends_with_wasm() {
    assert_cmd_snapshot!(cli().arg("tests/binaries/time.wasm"));
}

// #[test]
// fn path_is_a_wasm_component() {
//     let mut cmd = Command::cargo_bin("wasm-cli-runner").unwrap();
//     let assert = cmd.write_stdin("noop.wasm").assert();
//     assert.failure().stderr("Could not find file at location\n");
// }
