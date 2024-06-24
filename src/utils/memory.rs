use std::os::raw::c_void;

//pub fn relative_rip(addr: *mut c_void, instruction: i32) -> *mut c_void {
//    unsafe { return addr.add(instruction) + *((addr.add(instruction) - 4) as *mut i32) }
//}

pub fn resolve_relative_address(
    addr: *mut c_void,
    rva_offset: u32,
    rip_offset: u32,
) -> *mut c_void {
    unsafe {
        let rva = *(addr.add(rva_offset as usize) as *mut u32);
        let rip = (addr as u64).wrapping_add(rip_offset as u64);

        (rva as u64 + rip) as *mut c_void
    }
}
