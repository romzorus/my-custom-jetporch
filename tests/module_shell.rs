use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;
use assert_fs::TempDir;
use testinglib::*;


#[test]
fn test_module_shell() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    docker_init("containers-list-mod-shell.json", "shell");
    create_inventory(&tempfolder, "containers-list-mod-shell.json");

    let playbookcontent =
r#"---
- name: shell module testing
  groups:
    - all
  tasks:
    - !shell
      cmd: "cat /etc/os-release"
      save: cat_result

    - !echo
      msg: "\n/etc/os-release :\n{{ cat_result.out }}\n"
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

    docker_cleanup("containers-list-mod-shell.json", "shell");

    assert_eq!(output.status.success(), true);
    Ok(())
}
