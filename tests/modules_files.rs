use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;
use assert_fs::TempDir;
use testinglib::*;


#[test]
fn test_module_copy_file_directory_stat() -> Result<(), Box<dyn std::error::Error>> {
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
        - !directory
          path: /tmp/workdir
          attributes:
            owner: root
            group: root
            mode: 0o777
    
        - !copy
          src: /etc/hostname
          dest: /tmp/workdir/hostname
          attributes:
            owner: root
            group: root
            mode: 0o777
        
        - !stat
          path: /tmp/workdir/hostname
          save: stat_result
    
        - !debug
          vars:
          - stat_result
    
        - !file
          path: /tmp/workdir/hostname
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
