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
use std::sync::Arc;
use std::vec::Vec;

const MODULE: &str = "fetch";

#[derive(Deserialize,Debug)]
#[serde(deny_unknown_fields)]
pub struct FetchTask {
    pub name: Option<String>,
    pub remote_src: String,
    pub local_dest: String,
    pub attributes: Option<FileAttributesInput>,
    pub beforetask: Option<PreLogicInput>,
    pub aftertask: Option<PostLogicInput>
}

struct FetchAction {
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
                    remote_src: handle.template.string(request, tm, &String::from("src"), &self.remote_src)?,
                    local_dest: PathBuf::from(&handle.template.path(request, tm, &String::from("dest"), &self.local_dest)?),
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
                // TODO : so far, we assume the source path is a file and the destination path is fully qualified
                // -> add checks about that
                // -> add capacity to fetch folders (add optional parameter 'folder', false by default)
                // -> if source path is a folder, copy recursively or not (add optional parameter 'recursive', true by default)
                // -> if destination path does not specifies it, use the source filename/foldername as destination filename/foldername

                // Does (remote) source file exists ? Let's retrieve info on it as a test.
                let src_exists = handle.remote.get_mode(request, &self.remote_src)?;
                if src_exists == None {
                    return Err(handle.response.is_failed(request, &String::from("Remote source file not found")));
                }

                // Does (local) destination file already exists ?
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
                        return Ok(handle.response.needs_creation(request));
                    }
                } else {
                    // No the (local) destination file doesn't already exists.
                    // We need to fetch the remote file.
                    return Ok(handle.response.needs_creation(request));
                }
            },

            TaskRequestType::Create => {
                // If the (local) destination file already exists but with incorrect content, delete it before fetching the remote file
                let dest_path = Path::new(&self.local_dest);
                if dest_path.exists() {
                    let _ = std::fs::remove_file(dest_path); // TODO : Add error handling here
                }
                self.do_fetch(handle, request, None);

                return Ok(handle.response.is_created(request));
            }

            TaskRequestType::Modify => {
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

    pub fn do_fetch(&self, handle: &Arc<TaskHandle>, request: &Arc<TaskRequest>, _changes: Option<Vec<Field>>) -> Result<(), Arc<TaskResponse>> {
        
        handle.remote.fetch_file(request, &self.remote_src, &self.local_dest)
        
    }
}
