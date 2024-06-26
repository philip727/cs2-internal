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
        let rva = *(addr.add(rva_offset as usize) as *const u32);
        let rip = addr.wrapping_add(rip_offset as usize) as u64;

        (rva as u64 + rip) as *mut c_void
    }
}
pub unsafe fn relative_rip(address: *mut c_void, instruction: i32) -> *mut c_void {
    // Calculate the address of the instruction
    let instruction_address = address.wrapping_add(instruction as usize);

    // Read the 32-bit integer from the calculated address
    let offset = std::ptr::read((instruction_address.sub(4)) as *const i32);

    // Calculate the final address
    address
        .wrapping_add(instruction as usize)
        .wrapping_add(offset as usize)
}

pub unsafe fn dereference_addr(src: *mut c_void) -> *mut c_void {
    std::mem::transmute::<usize, *mut c_void>(*(src as *mut usize))
}
