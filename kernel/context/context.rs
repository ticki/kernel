use alloc::arc::Arc;
use alloc::boxed::Box;
use collections::{BTreeMap, Vec, VecDeque};
use spin::Mutex;

use arch;
use syscall::data::Event;
use super::resource::Resource;
use super::memory::{Grant, Memory, SharedMemory};

/// The status of a context.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    /// This context is ready to be switched to.
    Runnable,
    /// This context is blocked by some other process or job.
    Blocked,
    /// This context has exited with some status.
    Exited(usize),
}

/// A context, which identifies either a process or a thread.
///
/// This type contains data representing some context.
#[derive(Debug)]
pub struct Context {
    /// The ID of this context
    pub id: usize,
    /// The ID of the parent context
    pub ppid: usize,
    /// Status of context
    pub status: Status,
    /// Context running or not
    pub running: bool,
    /// Context is halting parent
    pub vfork: bool,
    /// The architecture specific context
    pub arch: arch::context::Context,
    /// Kernel FX
    pub kfx: Option<Box<[u8]>>,
    /// Kernel stack
    pub kstack: Option<Box<[u8]>>,
    /// Executable image
    pub image: Vec<SharedMemory>,
    /// User heap
    pub heap: Option<SharedMemory>,
    /// User stack
    pub stack: Option<Memory>,
    /// User grants
    pub grants: Arc<Mutex<Vec<Grant>>>,
    /// The current working directory
    pub cwd: Arc<Mutex<Vec<u8>>>,
    /// Kernel events
    pub events: Arc<Mutex<VecDeque<Event>>>,
    /// The process environment
    pub env: Arc<Mutex<BTreeMap<Box<[u8]>, Arc<Mutex<Vec<u8>>>>>>,
    /// The open resources in the scheme
    pub resources: Arc<Mutex<Vec<Option<Resource>>>>,
}

impl Context {
    /// Create a new context
    pub fn new(id: usize) -> Context {
        Context {
            id: id,
            ppid: 0,
            status: Status::Blocked,
            running: false,
            vfork: false,
            arch: arch::context::Context::new(),
            kfx: None,
            kstack: None,
            image: Vec::new(),
            heap: None,
            stack: None,
            grants: Arc::new(Mutex::new(Vec::new())),
            cwd: Arc::new(Mutex::new(Vec::new())),
            events: Arc::new(Mutex::new(VecDeque::new())),
            env: Arc::new(Mutex::new(BTreeMap::new())),
            resources: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn canonicalize(&self, path: &[u8]) -> Vec<u8> {
        if path.iter().position(|&b| b == b':').is_none() {
            let cwd = self.cwd.lock();
            if path == b"." {
                cwd.clone()
            } else if path == b".." {
                cwd[..cwd[..cwd.len() - 1]
                        .iter()
                        .rposition(|&b| b == b'/' || b == b':')
                        .map_or(cwd.len(), |i| i + 1)]
                    .to_vec()
            } else if path.starts_with(b"./") {
                let mut canon = cwd.clone();
                canon.extend_from_slice(&path[2..]);
                canon
            } else if path.starts_with(b"../") {
                let mut canon = cwd[..cwd[..cwd.len() - 1]
                        .iter()
                        .rposition(|&b| b == b'/' || b == b':')
                        .map_or(cwd.len(), |i| i + 1)]
                    .to_vec();
                canon.extend_from_slice(&path[3..]);
                canon
            } else if path.starts_with(b"/") {
                let mut canon = cwd[..cwd.iter().position(|&b| b == b':').map_or(1, |i| i + 1)]
                    .to_vec();
                canon.extend_from_slice(&path);
                canon
            } else {
                let mut canon = cwd.clone();
                if !canon.ends_with(b"/") {
                    canon.push(b'/');
                }
                canon.extend_from_slice(&path);
                canon
            }
        } else {
            path.to_vec()
        }
    }

    /// Add a resource to the lowest available slot.
    /// Return the resource descriptor number or None if no slot was found
    pub fn add_resource(&self, resource: Resource) -> Option<usize> {
        let mut resources = self.resources.lock();
        for (i, mut resource_option) in resources.iter_mut().enumerate() {
            if resource_option.is_none() {
                *resource_option = Some(resource);
                return Some(i);
            }
        }
        let len = resources.len();
        if len < super::CONTEXT_MAX_RESOURCES {
            resources.push(Some(resource));
            Some(len)
        } else {
            None
        }
    }

    /// Get a resource
    pub fn get_resource(&self, i: usize) -> Option<Resource> {
        let resources = self.resources.lock();
        if i < resources.len() { resources[i] } else { None }
    }

    /// Remove a resource
    // TODO: adjust resources vector to smaller size if possible
    pub fn remove_resource(&self, i: usize) -> Option<Resource> {
        let mut resources = self.resources.lock();
        if i < resources.len() {
            resources[i].take()
        } else {
            None
        }
    }
}
