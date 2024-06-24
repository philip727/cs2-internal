pub mod module;

// used for horizon zero dawn, maybe useful idk
pub fn add_offsets_to_addr(base_addr: usize, offsets: &[usize]) -> usize {
    let mut addr = base_addr as *const usize;

    let mut i = 0;
    for &offset in offsets {
        unsafe {
            //println!("{addr:p} + {offset:#0x} = ");
            addr = (addr as usize + offset) as *const usize;
            //println!("bfr: {addr:p}");
            if i == offsets.len() - 1 {
                break;
            }
            addr = std::ptr::read_volatile(addr as *const *const usize);
            //println!("fin: {addr:p}");
        }

        i += 1;
    }

    addr as usize
}

