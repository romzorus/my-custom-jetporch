// Jetporch
// Copyright (C) 2023 - Michael DeHaan <michael@michaeldehaan.net> + contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::tasks::*;
use crate::handle::handle::TaskHandle;
use crate::tasks::fields::Field;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use std::process::Command;
use std::sync::Arc;
use std::vec::Vec;
use std::fs;

const MODULE: &str = "fetch";

#[derive(Deserialize,Debug)]
#[serde(deny_unknown_fields)]
pub struct FetchTask {
    pub name: Option<String>,
    pub is_folder: Option<String>,
    pub remote_src: String,
    pub local_dest: String,
    pub attributes: Option<FileAttributesInput>,
    pub beforetask: Option<PreLogicInput>,
    pub aftertask: Option<PostLogicInput>
}

struct FetchAction {
    pub is_folder: bool,
    pub remote_src: String,
    pub local_dest: PathBuf,
}

impl IsTask for FetchTask {

    fn get_module(&self) -> String { String::from(MODULE) }
    fn get_name(&self) -> Option<String> { self.name.clone() }
    fn get_with(&self) -> Option<PreLogicInput> { self.beforetask.clone() }

    fn evaluate(&self, handle: &Arc<TaskHandle>, request: &Arc<TaskRequest>, tm: TemplateMode) -> Result<EvaluatedTask, Arc<TaskResponse>> {

        return Ok(
            EvaluatedTask {
                action: Arc::new(FetchAction {
                    is_folder: handle.template.boolean_option_default_false(&request, tm, &String::from("is_folder"), &self.is_folder)?,
                    remote_src: handle.template.string(&request, tm, &String::from("src"), &self.remote_src)?,
                    local_dest: PathBuf::from(&handle.template.path(&request, tm, &String::from("dest"), &self.local_dest)?),
                }),
                beforetask: Arc::new(PreLogicInput::template(&handle, &request, tm, &self.beforetask)?),
                aftertask: Arc::new(PostLogicInput::template(&handle, &request, tm, &self.aftertask)?),
            }
        );
    }

}

impl IsAction for FetchAction {

    fn dispatch(&self, handle: &Arc<TaskHandle>, request: &Arc<TaskRequest>) -> Result<Arc<TaskResponse>, Arc<TaskResponse>> {
    
        match request.request_type {

            TaskRequestType::Query => {

                if self.is_folder {
                    // Remote source is supposed to be a folder. Does it exist ?
                    let src_exists = handle.remote.get_is_directory(&request, &self.remote_src)?;
                    if src_exists {
                        // And does (local) destination folder already exist ?
                        let dest_path = Path::new(&self.local_dest);
                        if dest_path.exists() {
                            // Both remote and local folder already exist. Comparison required
                            let mut changes: Vec<Field> = Vec::new();
                            // 1. Folder structure comparison
                            changes.append(&mut self.compare_folders(handle, request)?);
                            // 2. Files comparison
                            changes.append(&mut self.compare_files(handle, request)?);
                            return Ok(handle.response.needs_modification(request, &changes));
                        } else {
                            return Ok(handle.response.needs_creation(request));
                        }

                        
                    } else {
                        // Remote folder not found so nothing much to do about it
                        return Err(handle.response.is_failed(request, &String::from("Remote folder not found")));
                    }
                } else {
                    // Remote source is supposed to be a file. Does it exist ?
                    let src_exists = handle.remote.get_is_file(request, &self.remote_src)?;
                    if ! src_exists {
                        return Err(handle.response.is_failed(request, &String::from("Remote source file not found")));
                    }

                    // Does (local) destination file already exist ?
                    let dest_path = Path::new(&self.local_dest);
                    if dest_path.exists() {
                        // Yes it already exists but...
                        // ... is it the same as (remote) source file ?
                        let local_512 = handle.local.get_sha512(request, dest_path, true)?;
                        let remote_512 = handle.remote.get_sha512(request, &self.remote_src)?;
                        if remote_512.eq(&local_512) {
                            // Yes it is, nothing to do then.
                            return Ok(handle.response.is_matched(request)); 
                        } else {
                            // No it is not so we need to get the right file.
                            return Ok(handle.response.needs_modification(request, &vec![Field::File(self.remote_src.clone())]));
                        }
                    } else {
                        // No the (local) destination file doesn't already exists.
                        // We need to fetch the remote file.
                        return Ok(handle.response.needs_creation(request));
                    }
                }

            },

            TaskRequestType::Create => {

                if self.is_folder {
                    let _ = self.do_create_folder_structure(handle, request);
                } else {
                    let _ = self.do_fetch_file(handle, request, &self.remote_src, None);
                }

                return Ok(handle.response.is_created(request));
            }

            TaskRequestType::Modify => {
                if self.is_folder {
                    // TBD
                    // for change in request.changes.iter() {
                    //     match change {
                    //         Field::File(remote_src) => {
                    //             self.do_fetch_file(handle, request, remote_src)?;
                    //         }

                    //         _ => {}
                    //     }
                    let _ = self.do_apply_changes(handle, request);
                        
                    // }
                } else {
                    // First we remove the local deprecated file...
                    let dest_path = Path::new(&self.local_dest);
                    let _ = std::fs::remove_file(dest_path);
                    // ... then we retrieve the remote one.
                    self.do_fetch_file(handle, request, &self.remote_src, None)?;
             }
                return Ok(handle.response.is_modified(request, request.changes.clone()));
            }

            TaskRequestType::Remove => {
                return Ok(handle.response.is_removed(request));
            }
    
            _ => { return Err(handle.response.not_supported(request)); }
    
        }
    }

}

impl FetchAction {

    // If no local_dest path is specified (None), the default behavior will be to use the local_dest field from the playbook
    pub fn do_fetch_file(&self, handle: &Arc<TaskHandle>, request: &Arc<TaskRequest>, remote_src: &String, local_dest: Option<String>) -> Result<(), Arc<TaskResponse>> {
        match local_dest {
            Some(local_dest) => {
                handle.remote.fetch_file(request, remote_src, &PathBuf::from(local_dest))
            }
            None => {
                handle.remote.fetch_file(request, remote_src, &self.local_dest)
            }
        }
        
    }

    // We can't assume that rsync is installed everywhere. So we first duplicate the folders, then the files.
    pub fn do_create_folder_structure(&self, handle: &Arc<TaskHandle>, request: &Arc<TaskRequest>) -> Result<(), Arc<TaskResponse>> {

        // Create the local dest folder
        let _ = fs::create_dir_all(&self.local_dest); // TODO : add error handling here

        // Create the whole folder structure first
        let cmd_remote_folder_list = format!("find {} -type d", self.remote_src);
        let remote_folder_list_result = handle.remote.run(request, &cmd_remote_folder_list, CheckRc::Checked);

        match remote_folder_list_result {
            Ok(r) => {
                let (_rc, remote_folder_list) = cmd_info(&r);

                for specific_remote_src_path in remote_folder_list.lines() {

                    let _ = fs::create_dir_all(
                        translate_path(
                            self.remote_src.clone(), 
                            String::from(specific_remote_src_path), 
                            self.local_dest.display().to_string()
                        ));
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        // Then fetch the files and place them inside the local dest folder structure.
        let cmd_remote_files_list = format!("find {} -type f", self.remote_src);
        let remote_files_list_result = handle.remote.run(request, &cmd_remote_files_list, CheckRc::Checked);

        match remote_files_list_result {
            Ok(r) => {
                let (_rc, remote_files_list) = cmd_info(&r);

                for remote_file_path in remote_files_list.lines() {

                    let local_dest_file_path = translate_path(
                        self.remote_src.clone(), 
                        String::from(remote_file_path), 
                        self.local_dest.display().to_string()
                    );
                    let _ = handle.remote.fetch_file(
                        request,
                        &remote_file_path.to_string(),
                        &PathBuf::from(local_dest_file_path)
                    );
                }

                // for remote_file in remote_files_list.lines() {
                //     // Does (local) destination file already exist ?
                //     let formatted_local_dest_file_path = remote_file.replace(&formatted_remote_source_path, &formatted_local_dest_root_path);
                //     let local_dest_file_path = PathBuf::from(formatted_local_dest_file_path);

                //     if local_dest_file_path.exists() {
                //         // Yes it already exists but...
                //         // ... is it the same as (remote) source file ?
                //         let local_512 = handle.local.get_sha512(request, &local_dest_file_path, true)?;
                //         let remote_512 = handle.remote.get_sha512(request, &remote_file.to_string())?;
                //         if remote_512.eq(&local_512) {
                //             // Yes it is the same, nothing to do then.
                //             continue; 
                //         } else {
                //             // No it is different so we need to get the right file.
                //             handle.remote.fetch_file(request, &remote_file.to_string(), &local_dest_file_path);
                //         }
                //     } else {
                //         // No the (local) destination file doesn't already exists.
                //         // We need to fetch the remote file.
                //         handle.remote.fetch_file(request, &remote_file.to_string(), &local_dest_file_path);
                //     }
                // }

                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }

        
        
    }

    pub fn compare_folders(&self, handle: &Arc<TaskHandle>, request: &Arc<TaskRequest>,) -> Result<Vec<Field>, Arc<TaskResponse>> {
        let mut folders_changes: Vec<Field> = Vec::new();

        // First we get the remote folder structure and directly turn it into the expected local folder structure (as a Vec<String> to facilitate later comparison).
        let cmd_remote_folder_list = format!("find {} -type d", self.remote_src);
        let remote_folder_list_result = handle.remote.run(request, &cmd_remote_folder_list, CheckRc::Checked);
        let mut expected_local_folder_structure: Vec<String> = vec![];


        match remote_folder_list_result {
            Ok(r) => {
                let (_rc, folder_list) = cmd_info(&r);
                for specific_remote_src_path in folder_list.lines() {
                    if handle.remote.get_is_directory(request, &specific_remote_src_path.to_string()).unwrap() {
                        expected_local_folder_structure.push(
                            translate_path(
                                self.remote_src.clone(),
                                specific_remote_src_path.to_string(),
                                self.local_dest.display().to_string())
                        );
                    }

                }
            }
            Err(e) => {
                println!("Error with \'find\' command : {:?}", e); // example: 'find' not in amazonlinux docker image by default
                println!("Trying with \'du\' command...");
                // Something went wrong with the 'find' command on the remote machine.
                // Trying with 'du' command.
                let du_command = format!("du -a {} | cut -f 2", self.remote_src);
                let du_result = handle.remote.run(request, &du_command, CheckRc::Checked);

                match du_result {
                    Ok(r) => {
                        let (_rc, folder_list) = cmd_info(&r);
                        for specific_remote_src_path in folder_list.lines() {

                            expected_local_folder_structure.push(
                                translate_path(
                                    self.remote_src.clone(),
                                    specific_remote_src_path.to_string(),
                                    self.local_dest.display().to_string())
                            );
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }

        // Then we get the actual local folder structure
        let cmd_local_folder_list = Command::new("find")
                .arg(self.local_dest.display().to_string())
                .arg("-type")
                .arg("d")
                .output();
        let mut actual_local_folder_structure: Vec<String> = vec![];

        match cmd_local_folder_list {
            Ok(output) => {
                let local_folder_list = String::from_utf8_lossy(&output.stdout).to_string();
                for local_folder_path in local_folder_list.lines() {
                    actual_local_folder_structure.push(local_folder_path.to_string());
                }
            }
            Err(e) => {
                return Err(handle.response.is_failed(request, &e.to_string()));
            }
        }

        // Now we can do the comparison between the expected and the actual local folder structure
        // Each expected path that is not in the actual structure will be added to the list of changes.
        for expected_folder in expected_local_folder_structure.iter() {
            if ! actual_local_folder_structure.contains(expected_folder) {
                folders_changes.push(
                    Field::Folder(expected_folder.clone())
                );
            }
        }

        Ok(folders_changes)

    }

    pub fn compare_files(&self, handle: &Arc<TaskHandle>, request: &Arc<TaskRequest>) -> Result<Vec<Field>, Arc<TaskResponse>> {
        let mut files_changes: Vec<Field> = Vec::new();

        // First we get the remote files list and directly turn it into the expected local files list (as a Vec<String> to facilitate later comparison).
        let cmd_remote_file_list = format!("find {} -type f", self.remote_src);
        let remote_file_list_result = handle.remote.run(request, &cmd_remote_file_list, CheckRc::Checked);
        let mut expected_remote_file_list: Vec<String> = vec![];


        match remote_file_list_result {
            Ok(r) => {
                let (_rc, file_list) = cmd_info(&r);
                for specific_remote_src_path in file_list.lines() {
                    expected_remote_file_list.push(specific_remote_src_path.to_string());
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        // Then we get the actual local files list
        let cmd_local_file_list = Command::new("find")
                .arg(self.local_dest.display().to_string())
                .arg("-type")
                .arg("f")
                .output();
        let mut actual_local_file_list: Vec<String> = vec![];


        match cmd_local_file_list {
            Ok(output) => {
                let local_file_list = String::from_utf8_lossy(&output.stdout).to_string();
                for local_folder_path in local_file_list.lines() {
                    actual_local_file_list.push(local_folder_path.to_string());
                }
            }
            Err(e) => {
                return Err(handle.response.is_failed(request, &e.to_string()));
            }
        }

        // Now we can do the comparison between the expected and the actual local file list
        // If an expected file is not in the actual list, it will be fetched.
        // If an expected file is in the actual list but the hashes are different, it will also be fetched.
        for expected_file in expected_remote_file_list.iter() {

            let translated_expected_file = translate_path(
                            self.remote_src.clone(),
                            expected_file.to_string(),
                            self.local_dest.display().to_string());

            if actual_local_file_list.contains(&translated_expected_file) {
                // Hash comparison
                let local_512 = handle.local.get_sha512(request, &PathBuf::from(translated_expected_file), true)?;
                let remote_512 = handle.remote.get_sha512(request, expected_file)?;
                if remote_512.eq(&local_512) {
                    continue;
                } else {
                    files_changes.push(Field::Folder(expected_file.clone()));
                }
            } else {
                files_changes.push(Field::Folder(expected_file.clone()));
            }
        }

        Ok(files_changes)
    }

    pub fn do_apply_changes(&self, handle: &Arc<TaskHandle>, request: &Arc<TaskRequest>) -> Result<(), Arc<TaskResponse>> {
        // Changes are supposed to be Folders first then Files in the vector
        // To no take any chance, we parse the full vector but only use one Field variant at a time.

        // First we work on the folder structure
        for change in request.changes.iter() {
            if let Field::Folder(specific_remote_src_path) = change {
                let _ = fs::create_dir_all(
                    translate_path(
                        self.remote_src.clone(), 
                        String::from(specific_remote_src_path), 
                        self.local_dest.display().to_string()
                    ));
            }
        }
        // Then we work on the files
        for change in request.changes.iter() {
            if let Field::File(specific_remote_src_path) = change {
                let specific_local_dest_path = translate_path(
                    self.remote_src.clone(),
                    specific_remote_src_path.clone(),
                    self.local_dest.display().to_string());
                
                // First we remove the local deprecated file...
                let _ = std::fs::remove_file(&specific_local_dest_path);
                // ... then we retrieve the remote one.
                self.do_fetch_file(
                    handle,
                    request, 
                    specific_remote_src_path, 
                    Some(specific_local_dest_path))?;
            }
        }

        Ok(())
    }
}

// remote_src_path: "/etc/apt"
// specific_remote_src_path: "/etc/apt/sources.list.d"
// local_dest_path: "/home/user/downlad"
// Expected result : "/home/user/download/sources.list.d"
fn translate_path(remote_src_path: String, specific_remote_src_path: String, local_dest_path: String) -> String {
        // First we make sure we have a final '/' at the end of the paths
        let mut formatted_local_dest_path = local_dest_path.clone();
        if formatted_local_dest_path.chars().last().unwrap() != '/' {
            formatted_local_dest_path.push('/');
        }
        let mut formatted_remote_src_path = remote_src_path.clone();
        if formatted_remote_src_path.chars().last().unwrap() != '/' {
            formatted_remote_src_path.push('/');
        }

        // Then we replace remote_src_path with local_dest_path in specific_remote_src_path
        let final_local_path = specific_remote_src_path.replace(&formatted_remote_src_path, &formatted_local_dest_path);

        // PathBuf::from(final_local_path)
        final_local_path
}

