use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;
use assert_fs::TempDir;
use testinglib::*;


#[test]
fn test_module_packages() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    docker_init("containers-list-mod-packages.json", "git");
    create_inventory(&tempfolder, "containers-list-mod-packages.json");

    // First we get facts then we install git. After that we clone a repo and list its files.
    let playbookcontent =
r#"---
- name: package modules testing
  groups:
    - all
  tasks:
    - !facts

    - !apt
      package: git
      beforetask:
        checkcondition: (eq jet_os_flavor "Debian")
    
    - !dnf
      package: git
      beforetask:
        checkcondition: (eq jet_os_flavor "Fedora")
    
    - !pacman
      package: git
      beforetask:
        checkcondition: (eq jet_os_flavor "Arch")
    
    - !zypper
      package: git
      beforetask:
        checkcondition: (eq jet_os_flavor "Suse")
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
    println!("Exit Status : {:?}", output.status);

    docker_cleanup("containers-list-mod-packages.json", "packages");

    assert_eq!(output.status.success(), true);
    Ok(())
}
