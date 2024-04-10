use crate::{util::read_until_nul, Context, LinkedMem, Mount, Position, UiState};
use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr::NonNull};

/// A pointer to [`LinkedMem`] with utility.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct MumblePtr(NonNull<LinkedMem>);

impl MumblePtr {
    /// Creates a new access point to the [`LinkedMem`].
    ///
    /// # Safety
    /// If the passed pointer is non-null, it must be properly aligned, dereferenceable, and point to an initialized instance of [`LinkedMem`].
    #[inline]
    pub unsafe fn new(ptr: *mut LinkedMem) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Returns a [`NonNull`] pointer to the [`LinkedMem`].
    #[inline]
    pub const fn as_non_null(&self) -> NonNull<LinkedMem> {
        self.0
    }

    /// Returns a raw pointer to the [`LinkedMem`].
    #[inline]
    pub const fn as_ptr(&self) -> *const LinkedMem {
        self.0.as_ptr()
    }

    /// Returns a reference to the [`LinkedMem`].
    ///
    /// # Safety
    /// See [`NonNull::as_ref`].
    #[inline]
    pub const unsafe fn as_ref(&self) -> &LinkedMem {
        self.0.as_ref()
    }

    /// Reads the entire current [`LinkedMem`] contents.
    #[inline]
    pub fn read(&self) -> LinkedMem {
        unsafe { self.as_ptr().read_volatile() }
    }

    /// Reads the current `ui_version`.
    #[inline]
    pub fn read_ui_version(&self) -> u32 {
        read_member!(self.ui_version)
    }

    /// Reads the current `ui_tick`.
    #[inline]
    pub fn read_ui_tick(&self) -> u32 {
        read_member!(self.ui_tick)
    }

    /// Reads the current player avatar [`Position`].
    #[inline]
    pub fn read_avatar(&self) -> Position {
        read_member!(self.avatar)
    }

    /// Reads the current game name.
    #[inline]
    pub fn read_name(&self) -> Vec<u16> {
        read_until_nul(member_ptr!(self.name))
    }

    /// Reads the current game name as [`OsString`].
    #[inline]
    pub fn read_name_string(&self) -> OsString {
        OsString::from_wide(&self.read_name())
    }

    /// Reads the current camera [`Position`].
    #[inline]
    pub fn read_camera(&self) -> Position {
        read_member!(self.camera)
    }

    /// Reads the current player identity.
    #[inline]
    pub fn read_identity(&self) -> Vec<u16> {
        read_until_nul(member_ptr!(self.identity))
    }

    /// Reads the current player identity as [`OsString`].
    #[inline]
    pub fn read_identity_string(&self) -> OsString {
        OsString::from_wide(&self.read_identity())
    }

    /// Parses the current player identity JSON contents.
    #[cfg(feature = "json")]
    pub fn parse_identity(&self) -> serde_json::Result<crate::Identity> {
        let string = self.read_identity_string();
        serde_json::from_str(&string.to_string_lossy())
    }

    /// Reads the current [`Context`].
    #[inline]
    pub fn read_context(&self) -> Context {
        read_member!(self.context)
    }

    /// Reads the current server address.
    #[inline]
    pub fn read_server_address(&self) -> [u8; 28] {
        read_member!(self.context.server_address)
    }

    /// Reads the current server address.
    #[inline]
    pub fn read_map_id(&self) -> u32 {
        read_member!(self.context.map_id)
    }

    /// Reads the current server address.
    #[inline]
    pub fn read_map_type(&self) -> u32 {
        read_member!(self.context.map_type)
    }

    /// Reads the current shard id.
    #[inline]
    pub fn read_shard_id(&self) -> u32 {
        read_member!(self.context.shard_id)
    }

    /// Reads the current instance id.
    #[inline]
    pub fn read_instance(&self) -> u32 {
        read_member!(self.context.instance)
    }

    /// Reads the current build id.
    #[inline]
    pub fn read_build_id(&self) -> u32 {
        read_member!(self.context.build_id)
    }

    /// Reads the current UI state.
    #[inline]
    pub fn read_ui_state(&self) -> UiState {
        read_member!(self.context.ui_state)
    }

    /// Reads the current compass width in pixels.
    #[inline]
    pub fn read_compass_width(&self) -> u16 {
        read_member!(self.context.compass_width)
    }

    /// Reads the current compass height in pixels.
    #[inline]
    pub fn read_compass_height(&self) -> u16 {
        read_member!(self.context.compass_height)
    }

    /// Reads the current compass dimensions in pixels.
    #[inline]
    pub fn read_compass_dimensions(&self) -> [u16; 2] {
        [self.read_compass_width(), self.read_compass_height()]
    }

    /// Reads the current compass rotation in radians.
    #[inline]
    pub fn read_compass_rotation(&self) -> f32 {
        read_member!(self.context.compass_rotation)
    }

    /// Reads the current player position x in continent coordinates.
    ///
    /// Not updated in competitive modes.
    #[inline]
    pub fn read_player_x(&self) -> f32 {
        read_member!(self.context.player_x)
    }

    /// Reads the current player position y in continent coordinates.
    ///
    /// Not updated in competitive modes.
    #[inline]
    pub fn read_player_y(&self) -> f32 {
        read_member!(self.context.player_y)
    }

    /// Reads the current player position x in continent coordinates.
    ///
    /// Not updated in competitive modes.
    #[inline]
    pub fn read_player_position(&self) -> [f32; 2] {
        [self.read_player_x(), self.read_player_y()]
    }

    /// Reads the current map center x in continent coordinates.
    ///
    /// Not updated in competitive modes.
    #[inline]
    pub fn read_map_center_x(&self) -> f32 {
        read_member!(self.context.map_center_x)
    }

    /// Reads the current map center y in continent coordinates.
    ///
    /// Not updated in competitive modes.
    #[inline]
    pub fn read_map_center_y(&self) -> f32 {
        read_member!(self.context.map_center_y)
    }

    /// Reads the current map center in continent coordinates.
    ///
    /// Not updated in competitive modes.
    #[inline]
    pub fn read_map_center(&self) -> [f32; 2] {
        [self.read_map_center_x(), self.read_map_center_y()]
    }

    /// Reads the map scale.
    #[inline]
    pub fn read_map_scale(&self) -> f32 {
        read_member!(self.context.map_scale)
    }

    /// Reads the process id.
    #[inline]
    pub fn read_process_id(&self) -> u32 {
        read_member!(self.context.process_id)
    }

    /// Reads the currently used mount.
    #[inline]
    pub fn read_mount_index(&self) -> Mount {
        read_member!(self.context.mount_index)
    }

    /// Reads the game description.
    #[inline]
    pub fn read_description(&self) -> Vec<u16> {
        read_until_nul(member_ptr!(self.description))
    }

    /// Reads the game description as [`OsString`].
    #[inline]
    pub fn read_description_string(&self) -> OsString {
        OsString::from_wide(&self.read_description())
    }
}

unsafe impl Send for MumblePtr {}

unsafe impl Sync for MumblePtr {}

macro_rules! member_ptr {
    ( $self:ident $( .$member:ident )+ ) => {{
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!( (*$self.as_ptr())  $( .$member )+ ) }
    }};
}

macro_rules! read_member {
    ( $self:ident $( .$member:ident )+ ) => {{
        #[allow(unused_unsafe)]
        unsafe { member_ptr!( $self $( .$member )+ ).read_volatile() }
    }};
}

pub(crate) use {member_ptr, read_member};
