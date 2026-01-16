use std::collections::HashMap;

#[derive(Default)]
pub struct ProcessManager {
    pub processes: HashMap<String, u32>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, id: String, pid: u32) {
        self.processes.insert(id, pid);
    }

    pub fn unregister(&mut self, id: &str) {
        self.processes.remove(id);
    }

    pub fn get_pid(&self, id: &str) -> Option<u32> {
        self.processes.get(id).copied()
    }
}
