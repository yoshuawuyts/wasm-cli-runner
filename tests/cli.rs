use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::process::Command;

fn cli() -> Command {
    Command::new(get_cargo_bin("wasm-cli-runner"))
}

#[test]
fn has_a_version() {
    assert_cmd_snapshot!(cli().arg("--version"));
}

// #[test]
// fn path_is_required() {
//     let mut cmd = Command::cargo_bin("wasm-cli-runner").unwrap();
//     let assert = cmd.assert();
//     let msg = "the following required arguments were not provided:";
//     assert.failure().stderr(predicates::str::contains(msg));
// }
//
// #[test]
// fn path_is_reachable_on_disk() {
//     let mut cmd = Command::cargo_bin("wasm-cli-runner").unwrap();
//     let assert = cmd.arg("noop.wasm").assert();
//     let msg = "No such file or directory";
//     assert.failure().stderr(predicates::str::contains(msg));
// }
//
// #[test]
// fn path_is_a_wasm_binary() {
//     let mut cmd = Command::cargo_bin("wasm-cli-runner").unwrap();
//     let assert = cmd.write_stdin("noop.wasm").assert();
//     assert.failure().stderr("Could not find file at location\n");
// }
//
// #[test]
// fn path_is_a_wasm_component() {
//     let mut cmd = Command::cargo_bin("wasm-cli-runner").unwrap();
//     let assert = cmd.write_stdin("noop.wasm").assert();
//     assert.failure().stderr("Could not find file at location\n");
// }
