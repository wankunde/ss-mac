use libproc::libproc::proc_pid::name;
use std::collections::HashMap;

pub struct ProcessMapper {
    cache: HashMap<u32, String>,
}

impl ProcessMapper {
    pub fn new() -> Self {
        ProcessMapper {
            cache: HashMap::new(),
        }
    }

    pub fn get_process_name(&mut self, pid: u32) -> Option<String> {
        if let Some(name) = self.cache.get(&pid) {
            return Some(name.clone());
        }

        match name(pid as i32) {
            Ok(pname) => {
                self.cache.insert(pid, pname.clone());
                Some(pname)
            }
            Err(_) => None,
        }
    }
}
