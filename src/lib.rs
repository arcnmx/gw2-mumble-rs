mod error;
mod linked_mem;

pub use error::*;
pub use linked_mem::*;

use std::{env, ffi::CString, io, mem, ptr};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
        System::Memory::{CreateFileMappingA, MapViewOfFile, FILE_MAP_READ, PAGE_READWRITE},
    },
};

/// Access point to the MumbleLink memory shared file.
#[derive(Debug)]
pub struct MumbleLink {
    handle: HANDLE,
    linked_mem: *const LinkedMem,
}

unsafe impl Send for MumbleLink {}

unsafe impl Sync for MumbleLink {}

impl MumbleLink {
    /// Creates a new access point to the MumbleLink.
    pub fn new() -> Result<Self, Error> {
        const SIZE: usize = mem::size_of::<LinkedMem>();

        let name = Self::link_name();
        if name == "0" {
            return Err(Error::Disabled);
        }
        let name = CString::new(name)?;

        let handle = unsafe {
            CreateFileMappingA(
                INVALID_HANDLE_VALUE,
                None,
                PAGE_READWRITE,
                0,
                SIZE as u32,
                PCSTR::from_raw(name.as_ptr() as _),
            )?
        };

        let ptr = unsafe { MapViewOfFile(handle, FILE_MAP_READ, 0, 0, SIZE) };
        if ptr.is_null() {
            let err = io::Error::last_os_error();
            unsafe { CloseHandle(handle) };
            Err(err.into())
        } else {
            Ok(Self {
                handle,
                linked_mem: ptr as _,
            })
        }
    }

    /// Returns a pointer to the [`LinkedMem`] of the MumbleLink.
    pub fn as_ptr(&self) -> *const LinkedMem {
        self.linked_mem
    }

    /// Reads the current [`LinkedMem`] contents from the MumbleLink.
    pub fn read(&self) -> LinkedMem {
        unsafe { ptr::read_volatile(self.linked_mem) }
    }

    /// Resolves the name of the MumbleLink memory shared file.
    pub fn link_name() -> String {
        env::args()
            .skip_while(|arg| arg != "-mumble")
            .nth(1)
            .unwrap_or_else(|| "MumbleLink".into())
    }
}

impl Drop for MumbleLink {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}
