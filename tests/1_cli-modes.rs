use assert_cmd::prelude::*;
use std::io::Write;
use std::process::Command;
use std::fs::File;
use predicates::prelude::*;
use assert_fs::TempDir;
use config::{self, Config, File as ConfigFile, FileFormat};
use serde_derive::Deserialize;
use std::env::current_dir;

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

// This function creates a simple working inventory pointing at the containers
fn create_inventory(tempfolder: &TempDir) {

    let config_builder = Config::builder()
        .add_source(ConfigFile::new("tests/containers-info.json", FileFormat::Json))
        .build()
        .unwrap();

    let containers_info = config_builder.try_deserialize::<ContainersInfo>().expect("Problem with deserialization of containers-info.json");

    let mut inventory_content = String::from("hosts:\n");

    for container in containers_info.containers_list.into_iter() {
        inventory_content.push_str(
            format!("  - {}\n", container.container_ip).as_str()
        )
    }

    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "inventory"));
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "inventory/groups"));
    let mut tempgroupfile = File::create(temp_absolute_path(tempfolder, "inventory/groups/containers")).expect("File creation failed");
    let _ = tempgroupfile.write_all(inventory_content.as_bytes());
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "inventory/group_vars"));
    let mut tempgroupvarfile = File::create(temp_absolute_path(tempfolder, "inventory/group_vars/containers")).expect("File creation failed");

    let privatekeyconf = format!("jet_ssh_private_key_file: {}/tests/controller_key", current_dir().unwrap().display());
    let _ = tempgroupvarfile.write_all(privatekeyconf.as_bytes());

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


#[derive(Deserialize)]
struct ContainersInfo {
    containers_list: Vec<ContainerSpec>
}

#[derive(Deserialize)]
struct ContainerSpec {
    container_name: String,
    container_id: String,
    container_ip: String,
    container_pubkey: String
}