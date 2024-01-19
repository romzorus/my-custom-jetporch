use std::io::Write;
use std::fs::File;
use assert_fs::TempDir;
use config::{self, Config, File as ConfigFile, FileFormat};
use serde_derive::Deserialize;
use std::env::current_dir;
use std::process::Command;

// This function launches the script responsible for creating containers as well
// as container list in JSON format.
pub fn docker_init(containerslistname: &str, moduletested: &str) {
    let _ = Command::new("tests/docker-scripts/docker-init.sh")
        .arg(containerslistname)
        .arg(moduletested)
        .output()
        .expect("Problem launching the init script");
}

// This function launches the script responsible for cleaning up containers (and list)
// at the end of a test.
pub fn docker_cleanup(containerslistname: &str, moduletested: &str) {
    let _ = Command::new("tests/docker-scripts/docker-cleanup.sh")
        .arg(containerslistname)
        .arg(moduletested)
        .output()
        .expect("Problem launching the cleanup script");
}

// This function simplifies the way to write absolute path based on the temporary folder created by each test
pub fn temp_absolute_path(tempfolder: &TempDir, relative_path: &str) -> String {
    format!("{}/{}", tempfolder.path().display(), relative_path)
}

// This function creates a simple working inventory pointing at the containers
pub fn create_inventory(tempfolder: &TempDir, containerslistname: &str) {

    let config_builder = Config::builder()
        .add_source(ConfigFile::new(format!("tests/{}", containerslistname).as_str(), FileFormat::Json))
        .build()
        .unwrap();

    let containers_info = config_builder.try_deserialize::<ContainersInfo>().expect(format!("Problem with deserialization of {}", containerslistname).as_str());

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

pub fn create_playbook(tempfolder: &TempDir, playbookcontent: &str) {
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "playbooks"));
    let mut tempplaybookfile = File::create(temp_absolute_path(tempfolder, "playbooks/play.yml")).unwrap();
    // let _ = tempplaybookfile.write_all(b"- name: show facts\n  groups:\n    - all\n  tasks:\n    - !facts\n    - !debug");
    let _ = tempplaybookfile.write_all(playbookcontent.as_bytes());
}

pub fn create_role(tempfolder: &TempDir) {
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "roles"));
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "roles/webserver"));
    let _ = std::fs::create_dir(temp_absolute_path(tempfolder, "roles/webserver/tasks"));
    let mut temprolefile = File::create(temp_absolute_path(tempfolder, "roles/webserver/webserver.yml")).unwrap();
    let _ = temprolefile.write_all(b"name: webserver\ntasks:\n  - webserver.yml");
    let mut temptasksfile = File::create(temp_absolute_path(tempfolder, "roles/webserver/tasks/webserver.yml")).unwrap();
    let _ = temptasksfile.write_all(b"- !facts\n  {}");
}


#[derive(Deserialize)]
pub struct ContainersInfo {
    pub containers_list: Vec<ContainerSpec>
}

#[derive(Deserialize)]
pub struct ContainerSpec {
    pub container_name: String,
    pub container_id: String,
    pub container_ip: String,
    pub container_pubkey: String
}