pub mod engine_client;
pub mod game_entity_system;
pub mod game_resource_service;
pub mod swap_chain_dx11;

use anyhow::anyhow;
use std::ffi::{c_char, c_void, CString};

use crate::utils::module::Module;


type CreateInterfaceFn = extern "C" fn(name: *const c_char, rc: *mut i32) -> *mut c_void;

pub fn get_factory(module: &Module) -> Option<CreateInterfaceFn> {
    module.get_function_addr("CreateInterface").map(|fn_addr| unsafe {
        std::mem::transmute::<_, CreateInterfaceFn>(fn_addr)
    })
}

pub fn create_interface(factory: CreateInterfaceFn, version: &str) -> Option<*mut c_void> {
    let c_version = CString::new(version).unwrap();
    let interface = factory(c_version.as_ptr(), std::ptr::null_mut());
    if !interface.is_null() {
        return Some(interface);
    }

    None
}

pub trait CaptureInterface<T> {
    fn capture(module: &Module, version: &str) -> anyhow::Result<*mut T> {
        let create_interface_fn = get_factory(module).ok_or(anyhow!(
            "Failed to get CreateInterface function from provided module"
        ))?;

        let interface_ptr = create_interface(create_interface_fn, version)
            .ok_or(anyhow!("failed to create interface with version {version}"))?;

        Ok(interface_ptr as *mut T)
    }

    unsafe fn get_virtual_func<TVirtFunc>(this: *mut Self, index: usize) -> anyhow::Result<TVirtFunc> {
        let base_addr = this as *mut usize;
        if base_addr.is_null() {
            return Err(anyhow!("Base address of interface is null"));
        }

        let vtable = *base_addr as *mut usize;
        if vtable.is_null() {
            return Err(anyhow!("Virtual table at {:p} is null", base_addr));
        }

        let fn_ptr = vtable.add(index).read() as *mut TVirtFunc;
        if fn_ptr.is_null() {
            return Err(anyhow!(
                "Function pointer at {:p} (index {}) in vtable at {:p} is not aligned with TVirtualFunction",
                fn_ptr,
                index,
                base_addr
            ));
        }

        let fn_copied = std::mem::transmute_copy::<*mut TVirtFunc, TVirtFunc>(&fn_ptr);

        Ok(fn_copied)
    }
}
