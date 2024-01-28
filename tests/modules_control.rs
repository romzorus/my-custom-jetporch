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
            condition: (eq jet_os_flavor "Debian")
      
        - !echo
          msg: "This OS if Fedora flavored."
          beforetask:
            condition: (eq jet_os_flavor "Fedora")
        
        - !echo
          msg: "This OS if Arch flavored."
          beforetask:
            condition: (eq jet_os_flavor "Arch")
        
        - !echo
          msg: "This OS if Suse flavored."
          beforetask:
            condition: (eq jet_os_flavor "Suse")
      
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
