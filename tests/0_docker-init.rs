use std::process::Command;

// This 'test' is just about running the initialization script
// named '0_docker-init-script.sh' and using its exit code as a test result.
// If the script fails, the test fails -> initialization failed
#[test]
fn docker_init() {
    let _ = Command::new("tests/0_docker-init-script.sh")
        .output()
        .expect("Problem launching the init script");
}