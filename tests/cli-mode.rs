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

#[test]
fn test_unset_mode() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    
    cmd.assert()
        .stdout(predicate::str::contains("usage: jetp <MODE> [flags]"));

    Ok(())
}

#[test]
fn test_show_inventory_mode() -> Result<(), Box<dyn std::error::Error>>{
    // Creating a temporary inventory
    let tempfolder = TempDir::new()?;
    let _ = std::fs::create_dir(format!("{}/groups", tempfolder.path().display()));
    let mut tempgroupfile = File::create(format!("{}/groups/webservers", tempfolder.path().display()))?;
    tempgroupfile.write_all(b"hosts:\n  - svr123.company.local\n  - svr954.company.local")?;

    // Running command : $ jetp show-inventory --inventory <path to temporary inventory>
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("show-inventory")
        .arg("--inventory")
        .arg(tempfolder.path());

    // Expecting to find our group and hosts
    cmd.assert()
        .stdout(predicate::str::contains("webservers"))
        .stdout(predicate::str::contains("svr123.company.local"))
        .stdout(predicate::str::contains("svr954.company.local"));

    Ok(())
}

// TODO : similar tests for check-local, local, check-ssh and ssh modes
// Create temp folder with inventory, role and playbook in it
// Run the command