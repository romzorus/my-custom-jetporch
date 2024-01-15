use assert_cmd::prelude::*;
use std::process::Command;
use predicates::prelude::*;
use assert_fs::TempDir;
use testinglib::*;


// This file checks that the proper CLI modes are run when selected :
// - unset
// - show-inventory
// - check-local
// - local
// - check-ssh
// - ssh



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

    // Running command : $ jetp show-inventory -i <path to temp inventory>
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("show-inventory")
        .arg("-i")
        .arg(format!("{}/inventory", tempfolder.path().display()));

    // Expecting to find our group and hosts
    cmd.assert()
        .code(predicate::eq(0));

    Ok(())
}

#[test]
fn test_cli_local_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    create_playbook(&tempfolder);

    // Running command : $ jetp local -p <path to temp playbook>
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

    // Running command : $ jetp check-local -p <path to temp playbook>
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("check-local")
        .arg("-p")
        .arg(format!("{}/playbooks/play.yml", tempfolder.path().display()));

    cmd.assert()
        .stdout(predicate::str::contains("play complete: show facts"))
        .code(predicate::eq(0));
        
        Ok(())
}

#[test]
fn test_cli_check_ssh_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    create_inventory(&tempfolder);
    create_playbook(&tempfolder);

    // Running command : $ jetp check-ssh -p <path to temp playbook> -i <path to temp inventory> -u root
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("check-ssh")
        .arg("-p")
        .arg(format!("{}/playbooks/play.yml", tempfolder.path().display()))
        .arg("-i")
        .arg(format!("{}/inventory", tempfolder.path().display()))
        .arg("-u")
        .arg("root");

    cmd.spawn()
        .expect("Failure to launch check-ssh command")
        .wait()
        .expect("Failure during the check-ssh test");
        
    Ok(())
}

#[test]
fn test_cli_ssh_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a temporary folder to work in
    let tempfolder = TempDir::new()?;

    create_inventory(&tempfolder);
    create_playbook(&tempfolder);

    // Running command : $ jetp check-ssh -p <path to temp playbook> -i <path to temp inventory> -u root
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("ssh")
        .arg("-p")
        .arg(format!("{}/playbooks/play.yml", tempfolder.path().display()))
        .arg("-i")
        .arg(format!("{}/inventory", tempfolder.path().display()))
        .arg("-u")
        .arg("root");

    cmd.spawn()
        .expect("Failure to launch ssh command")
        .wait()
        .expect("Failure during the ssh test");
        
    Ok(())
}
