use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;
use assert_fs::TempDir;
use testinglib::*;


#[test]
fn test_module_echo_facts_debug() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    docker_init("containers-list-mod-file.json", "file");
    create_inventory(&tempfolder, "containers-list-mod-file.json");

    // Create a directory on the remote host, copy a localhost file into the remote directory,
    // get the remote file's information then delete it
    let playbookcontent =
    r#"---
    - name: file module testing
      groups:
        - all
    
      tasks:

        - !facts
        
        - !echo
          msg: "This OS if Debian flavored."
          beforetask:
            checkcondition: (eq jet_os_flavor "Debian")
      
        - !echo
          msg: "This OS if Fedora flavored."
          beforetask:
            checkcondition: (eq jet_os_flavor "Fedora")
        
        - !echo
          msg: "This OS if Arch flavored."
          beforetask:
            checkcondition: (eq jet_os_flavor "Arch")
        
        - !echo
          msg: "This OS if Suse flavored."
          beforetask:
            checkcondition: (eq jet_os_flavor "Suse")
      
        - !debug
    
    "#;

    create_playbook(&tempfolder, playbookcontent);
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("ssh")
        .arg("-p")
        .arg(format!("{}/playbooks/play.yml", tempfolder.path().display()))
        .arg("-i")
        .arg(format!("{}/inventory", tempfolder.path().display()))
        .arg("-u")
        .arg("root");

    let output = cmd.output().unwrap();

    println!("{}", String::from_utf8_lossy(&output.stdout));

    docker_cleanup("containers-list-mod-file.json", "file");

    assert_eq!(output.status.success(), true);
    Ok(())
}

#[test]
fn test_module_fail() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    docker_init("containers-list-mod-fail.json", "fail");
    create_inventory(&tempfolder, "containers-list-mod-fail.json");

    // Fails
    let playbookcontent =
r#"---
- name: fail module testing
  groups:
    - all

  tasks:

  - !fail
    name: this will not fail
    aftertask:
      ignore_errors: true

  - !fail
    name: this will fail
"#;

    create_playbook(&tempfolder, playbookcontent);
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("ssh")
        .arg("-p")
        .arg(format!("{}/playbooks/play.yml", tempfolder.path().display()))
        .arg("-i")
        .arg(format!("{}/inventory", tempfolder.path().display()))
        .arg("-u")
        .arg("root");

    let output = cmd.output().unwrap();

    println!("{}", String::from_utf8_lossy(&output.stdout));

    docker_cleanup("containers-list-mod-fail.json", "fail");

    assert_eq!(output.status.success(), false);
    Ok(())
}

#[test]
fn test_module_assert() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    docker_init("containers-list-mod-assert.json", "assert");
    create_inventory(&tempfolder, "containers-list-mod-assert.json");

    // Fails
    let playbookcontent =
r#"---
- name: assert module testing
  groups:
    - all

  defaults:
    dog: scooby
    ghost: "blinky"

  tasks:

  - !facts

  - !assert
    name: test1
    msg: the OS must be Linux
    true: (eq jet_os_type "Linux")
       
  - !assert
    name: test2
    msg: the OS must not be MacOS
    false: (eq jet_os_type "MacOS")

  - !assert
    name: test3
    msg: various things must all be true
    all_true:
      - (eq dog "scooby")
      - (eq ghost "blinky")
          
  - !assert
    name: test4
    msg: none of these things may be true
    all_false:
      - (eq jet_os_type "Atari")
      - (eq ghost "Slimer")
          
  - !assert
    name: test5
    msg: one of these things must be true
    some_true:
      - (eq ghost "Slimer")
      - (eq jet_os_type "Linux")
"#;

    create_playbook(&tempfolder, playbookcontent);
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("ssh")
        .arg("-p")
        .arg(format!("{}/playbooks/play.yml", tempfolder.path().display()))
        .arg("-i")
        .arg(format!("{}/inventory", tempfolder.path().display()))
        .arg("-u")
        .arg("root");

    let output = cmd.output().unwrap();

    println!("{}", String::from_utf8_lossy(&output.stdout));

    docker_cleanup("containers-list-mod-assert.json", "assert");

    assert_eq!(output.status.success(), true);
    Ok(())
}
