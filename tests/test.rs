use assert_cmd::Command;

#[test]
fn is_a_path_arg() {
    let mut cmd = Command::cargo_bin("wasm-cli-runner").unwrap();
    let assert = cmd.assert();
    assert.failure().stdout("Could not find file at location\n");
}

#[test]
fn is_a_reachable_path() {
    let mut cmd = Command::cargo_bin("wasm-cli-runner").unwrap();
    let assert = cmd.write_stdin("noop.wasm").assert();
    assert.failure().stdout("Could not find file at location\n");
}

#[test]
fn is_a_wasm_binary() {
    let mut cmd = Command::cargo_bin("wasm-cli-runner").unwrap();
    let assert = cmd.write_stdin("noop.wasm").assert();
    assert.failure().stdout("Could not find file at location\n");
}

#[test]
fn is_a_wasm_component() {
    let mut cmd = Command::cargo_bin("wasm-cli-runner").unwrap();
    let assert = cmd.write_stdin("noop.wasm").assert();
    assert.failure().stdout("Could not find file at location\n");
}
