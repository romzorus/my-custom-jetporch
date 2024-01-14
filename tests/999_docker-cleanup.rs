use std::process::Command;

// This 'test' is just about running the cleanup script
// named '999_docker-cleanup-script.sh' and using its exit code as a test result.
// If the script fails, the test fails -> cleanup failed
#[test]
fn docker_cleanup() {
    let mut command = Command::new("tests/999_docker-cleanup-script.sh")
        .spawn()
        .expect("Problem launching the cleanup script");

    let results = command
        .wait()
        .expect("Something went wrong with the cleanup script itself");

    if !results.success() {
        panic!("Something went wrong with the cleanup script itself");
    }
}