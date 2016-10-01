//! # The Redox OS Kernel, redux
//!
//! The Redox OS Kernel is a hybrid kernel that supports X86 systems and
//! provides Unix-like syscalls for primarily Rust applications

#![feature(alloc)]
#![feature(asm)]
#![feature(collections)]
#![feature(const_fn)]
#![feature(drop_types_in_const)]
#![feature(heap_api)]
#![feature(question_mark)]
#![feature(never_type)]
#![feature(thread_local)]
#![no_std]

use arch::interrupt;

/// Architecture specific items (test)
#[cfg(test)]
#[macro_use]
extern crate arch_test as arch;

/// Architecture specific items (ARM)
#[cfg(all(not(test), target_arch = "arm"))]
#[macro_use]
extern crate arch_arm as arch;

/// Architecture specific items (x86_64)
#[cfg(all(not(test), target_arch = "x86_64"))]
#[macro_use]
extern crate arch_x86_64 as arch;

extern crate alloc;
#[macro_use]
extern crate collections;

#[macro_use]
extern crate bitflags;
extern crate goblin;
extern crate spin;

use core::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

/// Context management
pub mod context;

/// ELF resource parsing
#[cfg(all(not(test), target_arch = "x86_64"))]
pub mod elf;

/// Schemes, resourcesystem handlers
pub mod scheme;

/// Syscall handlers
pub mod syscall;

/// Tests
#[cfg(test)]
pub mod tests;

#[thread_local]
static CPU_ID: AtomicUsize = ATOMIC_USIZE_INIT;

#[inline(always)]
pub fn cpu_id() -> usize {
    CPU_ID.load(Ordering::Relaxed)
}

pub extern fn userspace_init() {
    assert_eq!(syscall::chdir(b"initfs:bin"), Ok(0));

    assert_eq!(syscall::open(b"debug:", 0), Ok(0));
    assert_eq!(syscall::open(b"debug:", 0), Ok(1));
    assert_eq!(syscall::open(b"debug:", 0), Ok(2));

    syscall::exec(b"initfs:bin/init", &[]).expect("failed to execute initfs:init");

    panic!("initfs:init returned")
}

#[no_mangle]
pub extern fn kmain() {
    CPU_ID.store(0, Ordering::SeqCst);

    context::init();

    let pid = syscall::getpid();
    println!("BSP: {:?}", pid);

    match context::contexts_mut().spawn(userspace_init) {
        Ok(context_lock) => {
            let mut context = context_lock.write();
            context.status = context::Status::Runnable;
        },
        Err(err) => {
            panic!("failed to spawn userspace_init: {:?}", err);
        }
    }

    loop {
        unsafe {
            interrupt::disable();
            if context::switch() {
                interrupt::enable_and_nop();
            } else {
                interrupt::enable_and_halt();
            }
        }
    }
}

#[no_mangle]
pub extern fn kmain_ap(id: usize) {
    CPU_ID.store(id, Ordering::SeqCst);

    context::init();

    let pid = syscall::getpid();
    println!("AP {}: {:?}", id, pid);

    loop {
        unsafe { interrupt::enable_and_halt() }
    }
}
