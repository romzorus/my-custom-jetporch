use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;
use assert_fs::TempDir;
use testinglib::*;


#[test]
fn test_module_file() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    docker_init("containers-list-mod-file.json", "file");
    create_inventory(&tempfolder, "containers-list-mod-file.json");

    let playbookcontent =
r#"---
- name: file module testing
  groups:
    - all
  tasks:
    - !file
      path: /root/myfile
      attributes:
        owner: root
        group: root
        mode: 0o777

    - !file
      path: /root/myfile
      remove: true
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
fn test_module_git() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    docker_init("containers-list-mod-git.json", "git");
    create_inventory(&tempfolder, "containers-list-mod-git.json");

    // First we get facts then we install git. After that we clone a repo and list its files.
    let playbookcontent =
r#"---
- name: git module testing
  groups:
    - all
  tasks:
    - !facts

    - !apt
      package: git
      beforetask:
        condition: (eq jet_os_flavor "Debian")
    
    - !dnf
      package: git
      beforetask:
        condition: (eq jet_os_flavor "Fedora")
    
    - !pacman
      package: git
      beforetask:
        condition: (eq jet_os_flavor "Arch")
    
    - !zypper
      package: git
      beforetask:
        condition: (eq jet_os_flavor "Suse")

    - !git
      repo: https://github.com/romzorus/my-custom-jetporch.git
      path: /opt/mycustomjetporch
      branch: main
    
    - !shell
      cmd: "ls /opt/mycustomjetporch/*/"
      save: ls_result

    - !echo
      msg: "\n/opt/mycustomjetporch (depth 1) :\n{{ ls_result.out }}\n"
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

    docker_cleanup("containers-list-mod-git.json", "git");

    assert_eq!(output.status.success(), true);
    Ok(())
}
