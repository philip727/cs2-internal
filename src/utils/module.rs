
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{FARPROC, HMODULE},
        System::{
            LibraryLoader::GetProcAddress,
            ProcessStatus::{GetModuleInformation, MODULEINFO},
            Threading::GetCurrentProcess,
        },
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Module {
    handle: HMODULE,
    base_address: usize,
}

impl Module {
    pub fn new(module: HMODULE) -> anyhow::Result<Self> {
        let mut module_info = unsafe { std::mem::zeroed::<MODULEINFO>() };

        unsafe {
            GetModuleInformation(
                GetCurrentProcess(),
                module,
                &mut module_info,
                std::mem::size_of::<MODULEINFO>() as u32,
            )?
        }

        Ok(Self {
            handle: module,
            base_address: module_info.lpBaseOfDll as usize,
        })
    }

    pub fn base_addr(&self) -> usize {
        self.base_address
    }

    pub fn handle(&self) -> HMODULE {
        self.handle
    }

    #[inline]
    pub fn get_function_addr(&self, fn_name: &str) -> FARPROC {
        unsafe { GetProcAddress(self.handle, PCSTR(format!("{fn_name}\x00").as_ptr())) }
    }
}
