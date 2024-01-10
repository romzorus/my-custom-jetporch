use assert_cmd::prelude::*;
use std::io::Write;
use std::process::Command;
use std::fs::File;
use predicates::prelude::*;
use assert_fs::TempDir;

// This file checks that the proper CLI modes are run when selected :
// - unset
// - show-inventory
// - check-local
// - local
// - check-ssh
// - ssh

// This function simplifies the way to write absolute path based on the temporary folder created by each test
fn temp_absolute_path(tempfolder: &TempDir, relative_path: &str) -> String {
    format!("{}/{}", tempfolder.path().display(), relative_path)
}

// This function creates a simple working inventory
fn create_inventory(tempfolder: &TempDir) {
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "inventory"));
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "inventory/groups"));
    let mut tempgroupfile = File::create(temp_absolute_path(tempfolder, "inventory/groups/webservers")).unwrap();
    let _ = tempgroupfile.write_all(b"hosts:\n  - svr123.company.local\n  - svr954.company.local");
}

fn create_playbook(tempfolder: &TempDir) {
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "playbooks"));
    let mut tempplaybookfile = File::create(temp_absolute_path(tempfolder, "playbooks/play.yml")).unwrap();
    let _ = tempplaybookfile.write_all(b"- name: show facts\n  groups:\n    - all\n  tasks:\n    - !facts\n    - !debug");
}

fn create_role(tempfolder: &TempDir) {
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "roles"));
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "roles/webserver"));
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "roles/webserver/tasks"));
    let mut temprolefile = File::create(temp_absolute_path(tempfolder, "roles/webserver/webserver.yml")).unwrap();
    let _ = temprolefile.write_all(b"name: webserver\ntasks:\n  - webserver.yml");
    let mut temptasksfile = File::create(temp_absolute_path(tempfolder, "roles/webserver/tasks/webserver.yml")).unwrap();
    let _ = temptasksfile.write_all(b"- !facts\n  {}");
}

#[test]
fn test_cli_unset_mode() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    
    cmd.assert()
        .stdout(predicate::str::contains("usage: jetp <MODE> [flags]"))
        .code(predicate::eq(0));

    Ok(())
}

#[test]
fn test_cli_show_inventory_mode() -> Result<(), Box<dyn std::error::Error>>{
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;
    
    create_inventory(&tempfolder);

    // Running command : $ jetp show-inventory --inventory <path to temporary inventory>
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("show-inventory")
        .arg("--inventory")
        .arg(format!("{}/inventory", tempfolder.path().display()));

    // Expecting to find our group and hosts
    cmd.assert()
        .stdout(predicate::str::contains("webservers"))
        .stdout(predicate::str::contains("svr123.company.local"))
        .stdout(predicate::str::contains("svr954.company.local"))
        .code(predicate::eq(0));

    Ok(())
}

#[test]
fn test_cli_local_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    create_playbook(&tempfolder);

    // Running command : $ jetp local -p <path to temporary playbook>
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("local")
        .arg("-p")
        .arg(format!("{}/playbooks/play.yml", tempfolder.path().display()));

    cmd.assert()
        .stdout(predicate::str::contains("play complete: show facts"))
        .code(predicate::eq(0));
        
        Ok(())
}

// TODO : find a better way to distinguish local and check-local because at the moment,
// with !facts, the results are identical
#[test]
fn test_cli_check_local_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    create_playbook(&tempfolder);

    // Running command : $ jetp check-local -p <path to temporary playbook>
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("check-local")
        .arg("-p")
        .arg(format!("{}/playbooks/play.yml", tempfolder.path().display()));

    cmd.assert()
        .stdout(predicate::str::contains("play complete: show facts"))
        .code(predicate::eq(0));
        
        Ok(())
}

// TODO : similar tests for local, check-ssh and ssh modes
// Create temp folder with inventory, role and playbook in it
// Run the command