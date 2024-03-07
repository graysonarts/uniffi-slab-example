#![allow(unused)]
use std::any::Any;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use slab::Slab;

uniffi::include_scaffolding!("lib");

type OpaqueHandle = u32;
struct OpaqueType {
    data: AtomicU32,
}

static COMMAND_SLAB: Lazy<Mutex<Slab<OpaqueType>>> = Lazy::new(|| Mutex::new(Slab::new()));

fn ffi_make_opaque(number: u32) -> OpaqueHandle {
    COMMAND_SLAB.lock().unwrap().insert(OpaqueType {
        data: AtomicU32::new(number),
    }) as OpaqueHandle
}

fn ffi_process_handle(handle: OpaqueHandle) {
    let slab = COMMAND_SLAB.lock().unwrap();
    let command = slab.get(handle as usize).unwrap();
    println!(
        "Processing handle: {:?}",
        command.data.load(std::sync::atomic::Ordering::SeqCst)
    );
}

fn ffi_initialize() {
    println!("{:?}", COMMAND_SLAB.lock().unwrap().type_id());
}

fn ffi_shutdown() {
    let mut slab = COMMAND_SLAB.lock().unwrap();
    println!("Clearing {} items", slab.len());
    slab.clear();
}
