use core::ops::{Deref, DerefMut};
use core::{mem, slice};

#[derive(Copy, Clone, Debug, Default)]
pub struct Event {
    pub id: usize,
    pub flags: usize,
    pub data: usize
}

impl Deref for Event {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Event as *const u8, mem::size_of::<Event>()) as &[u8]
        }
    }
}

impl DerefMut for Event {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Event as *mut u8, mem::size_of::<Event>()) as &mut [u8]
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(packed)]
pub struct Packet {
    pub id: u64,
    pub pid: usize,
    pub uid: u32,
    pub gid: u32,
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize
}

impl Deref for Packet {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Packet as *const u8, mem::size_of::<Packet>()) as &[u8]
        }
    }
}

impl DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Packet as *mut u8, mem::size_of::<Packet>()) as &mut [u8]
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(packed)]
pub struct Stat {
    pub st_mode: u16,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_size: u64
}

impl Deref for Stat {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Stat as *const u8, mem::size_of::<Stat>()) as &[u8]
        }
    }
}

impl DerefMut for Stat {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Stat as *mut u8, mem::size_of::<Stat>()) as &mut [u8]
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(packed)]
pub struct TimeSpec {
    pub tv_sec: i64,
    pub tv_nsec: i32,
}
