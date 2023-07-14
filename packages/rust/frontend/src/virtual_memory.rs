use std::sync::RwLock;
use std::vec::IntoIter;

pub struct VirtualMemory<T> {
    // so this is a really cheap way of creating virtual memory
    // for now we only need our  memory to store FrontendInstances and outside of testing, we only need one
    // so for simplicity we will just use a Vec
    memory: RwLock<Vec<T>>,
}

impl<T: Clone> VirtualMemory<T> {
    pub fn new() -> Self {
        Self {
            memory: RwLock::new(Vec::with_capacity(1)),
        }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        let memory = self.memory.read().unwrap();
        memory.get(index).cloned()
    }

    pub fn push(&self, instance: T) -> usize {
        let mut write = self.memory.write().unwrap();
        write.push(instance);
        write.len() - 1
    }

    pub fn clone_iter(&self) -> IntoIter<T> {
        self.memory.read().unwrap().clone().into_iter()
    }

    pub fn remove(&self, index: usize) -> T {
        self.memory.write().unwrap().swap_remove(index)
    }
}
