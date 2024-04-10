//! Guild Wars 2 MumbleLink bindings.
//!
//! ```no_run
//! use gw2_mumble::MumbleLink;
//!
//! let mumble = MumbleLink::new().unwrap();
//! let camera = mumble.read_camera();
//! let player_pos = mumble.read_avatar();
//! ```
//!
//! [Serde](https://serde.rs) support can be enabled with the `"serde"` feature.
//!
//! Parsing of the player identity JSON is supported when enabling the `"json"` feature:
//! ```ignore
//! let identity = mumble.parse_identity();
//! ```

mod context;
mod error;
mod identity;
mod link_ptr;
mod linked_mem;
mod util;

pub mod map_id;
pub mod map_type;

pub use self::{context::*, error::*, identity::*, link_ptr::*, linked_mem::*};

use std::{env, ffi::CString, io, mem};
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
    ptr: MumblePtr,
}

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
                PCSTR::from_raw(name.as_ptr().cast()),
            )?
        };

        let ptr = unsafe { MapViewOfFile(handle, FILE_MAP_READ, 0, 0, SIZE) }.Value;
        if let Some(ptr) = unsafe { MumblePtr::new(ptr.cast()) } {
            Ok(Self { handle, ptr })
        } else {
            let err = io::Error::last_os_error();
            let _ = unsafe { CloseHandle(handle) };
            Err(err.into())
        }
    }

    /// Returns the [`MumblePtr`] for the MumbleLink.
    #[inline]
    pub fn as_mumble_ptr(&self) -> MumblePtr {
        self.ptr
    }

    /// Resolves the name of the MumbleLink memory mapped file.
    pub fn link_name() -> String {
        env::args()
            .skip_while(|arg| arg != "-mumble")
            .nth(1)
            .unwrap_or_else(|| "MumbleLink".into())
    }
}

impl Drop for MumbleLink {
    #[inline]
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.handle) };
    }
}

impl std::ops::Deref for MumbleLink {
    type Target = MumblePtr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}
