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

#[test]
fn test_module_fetch() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    docker_init("containers-list-mod-fetch.json", "fetch");
    create_inventory(&tempfolder, "containers-list-mod-fetch.json");

    // Copy the /etc/os-release file in the /root folder then fetch this folder
    // in a local folder named as the remote host hostname
    let playbookcontent = format!(
r#"---
- name: fetch module testing
  groups:
    - all
  tasks:
    - !shell
      cmd: "cat /etc/hostname"
      save: remote_hostname

    - !shell
      cmd: cp /etc/os-release /root

    - !fetch
      is_folder: true
      remote_src: /root
      local_dest: {}/root{{ remote_hostname.out }}
"#, tempfolder.path().display());

    create_playbook(&tempfolder, playbookcontent.as_str());
    
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
    // Assert that the files have been successfully fetched according to jet
    assert_eq!(output.status.success(), true);

    // First we make sure we fetched the right number of files.
    // Count the number of hosts in inventory (aka number of containers/Dockerfiles) -> '- 1' because the first line is 'hosts:' and not an actual host
    // Count the number of 'os-release' files retrieved by the play
    // Compare these numbers and exit 1 if discripency
    let checking_number_cmd = Command::new("sh")
        .arg("-c")
        .arg(format!("EXPECTED=$(($(wc -l < {}/inventory/groups/containers) - 1)); REAL=$(find {}/root-* -type f -name \"os-release\"| wc -l); if ! [ $EXPECTED -eq $REAL ]; then echo \"Wrong number of fetched files\"; exit 1; fi"
            , tempfolder.path().display()
            , tempfolder.path().display()))
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&checking_number_cmd.stdout));
    // Assert that we retrieved one 'os-release' file per container
    assert_eq!(checking_number_cmd.status.success(), true);

    // Then we explore each fetched file, look for 'VERSION' keyword, and exit 1 if one the files doesn't have this keyword.
    let checking_content_cmd = Command::new("sh")
      .arg("-c")
      .arg(format!("LIST=$(find {}/root* -type f -name \"os-release\"); for file in $LIST; do if ! grep -q VERSION $file; then exit 1; else echo \"$file contains VERSION\"; fi; done", tempfolder.path().display()))
      .output()
      .unwrap();
    println!("{}", String::from_utf8_lossy(&checking_content_cmd.stdout));
    // Assert that all fetched files contains 'VERSION' keyword, meaning they are not empty and are likely to contain informations about the OS as expected
    assert_eq!(checking_content_cmd.status.success(), true);

    docker_cleanup("containers-list-mod-fetch.json", "fetch");


    Ok(())
}