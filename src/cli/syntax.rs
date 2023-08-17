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
// long with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::playbooks::traversal::{playbook_traversal};
use crate::connection::no::{NoFactory};
use crate::playbooks::context::{PlaybookContext};
use crate::playbooks::visitor::PlaybookVisitor;
use crate::connection::factory::ConnectionFactory;
use crate::inventory::inventory::Inventory;
use std::path::PathBuf;
use std::sync::{Arc,Mutex,RwLock};

struct SyntaxVisitor {}
impl SyntaxVisitor {
    pub fn new() -> Self { Self {} }
}
impl PlaybookVisitor for SyntaxVisitor {
    fn is_syntax_only(&self)    -> bool { return true; }
    fn is_check_mode(&self)     -> bool { return true; }
}

pub fn playbook_syntax_scan(inventory: &Arc<RwLock<Inventory>>, playbook_paths: &Vec<PathBuf>) -> Result<(), String> {
    
    let context : Arc<RwLock<PlaybookContext>> = Arc::new(RwLock::new(PlaybookContext::new()));
    let visitor : Arc<Mutex<dyn PlaybookVisitor>> = Arc::new(Mutex::new(SyntaxVisitor::new()));
    let factory : Arc<RwLock<dyn ConnectionFactory>> = Arc::new(RwLock::new(NoFactory::new()));

    // FIXME: the default user should come from the CLI --user at least in cases of ssh commands, otherwise
    // we don't really need it.
    return playbook_traversal(inventory, &playbook_paths, &context, &visitor, &factory, String::from("root"));

}