//!

use crate::c_api::ffi;

mod builder;
pub(crate) mod c_api;
pub use builder::NcPlaneOptionsBuilder;

use crate::{NcAlign, NcResizeCb};

use std::ptr::{null, null_mut};

/// Options struct for [`NcPlane`][crate::NcPlane].
///
/// It is recommended to construct it via [`NcPlaneOptionsBuilder`]
/// by calling [`NcPlaneOptions::builder()`].
///
/// # Fields
///
/// - [`y`]: vertical placement relative to parent plane.
/// - [`x`]: horizontal placement relative to parent plane.
/// - [`rows`]: vertical length in rows.
/// - [`cols`]: horizontal length in columns.
/// - [`userptr`]: optional user curry.
/// - [`name`]: optional string identifier for debugging.
/// - [`resizecb`]: callback when parent is resized.
/// - [`flags`]: bitmask of options: [`HORALIGNED`], [`VERALIGNED`], [`FIXED`],
///   [`MARGINALIZED`], [`AUTOGROW`], [`VSCROLL`].
/// - [`margin_b`]: bottom margin (requires the [`MARGINALIZED`] flag).
/// - [`margin_r`]: right margin (requires the [`MARGINALIZED`]).
///
/// [`y`]: ffi::ncplane_options#structfield.y
/// [`x`]: ffi::ncplane_options#structfield.x
/// [`rows`]: ffi::ncplane_options#structfield.rows
/// [`cols`]: ffi::ncplane_options#structfield.cols
/// [`userptr`]: ffi::ncplane_options#structfield.userptr
/// [`name`]: ffi::ncplane_options#structfield.name
/// [`resizecb`]: ffi::ncplane_options#structfield.resizecb
/// [`flags`]: ffi::ncplane_options#structfield.flags
/// [`margin_b`]: ffi::ncplane_options#structfield.margin_b
/// [`margin_r`]: ffi::ncplane_options#structfield.margin_r
/// [`VERALIGNED`]:NcPlaneOptions#associatedconstant.VERALIGNED
/// [`HORALIGNED`]: NcPlaneOptions#associatedconstant.HORALIGNED
/// [`MARGINALIZED`]: NcPlaneOptions#associatedconstant.MARGINALIZED
/// [`FIXED`]: NcPlaneOptions#associatedconstant.FIXED
/// [`AUTOGROW`]: NcPlaneOptions#associatedconstant.AUTOGROW
/// [`VSCROLL`]: NcPlaneOptions#associatedconstant.VSCROLL
pub type NcPlaneOptions = ffi::ncplane_options;

/// # Constructors
impl NcPlaneOptions {
    /// New NcPlaneOptions using the horizontal x.
    pub fn new(y: i32, x: i32, rows: u32, cols: u32) -> Self {
        Self::with_flags(y, x, rows, cols, None, 0, 0, 0)
    }

    /// Returns a default builder object for `NcPlaneOptions`.
    pub fn builder() -> NcPlaneOptionsBuilder {
        NcPlaneOptionsBuilder::default()
    }

    /// Returns a builder object for `NcPlaneOptions` from the current options.
    pub fn to_builder(&self) -> NcPlaneOptionsBuilder {
        NcPlaneOptionsBuilder::from_options(self)
    }

    /// New NcPlaneOptions with horizontal alignment.
    pub fn new_aligned(y: i32, align: NcAlign, rows: u32, cols: u32) -> Self {
        Self::with_flags_aligned(y, align, rows, cols, None, NcPlaneOptions::HORALIGNED)
    }

    /// New NcPlaneOptions, with flags.
    pub fn with_flags(
        y: i32,
        x: i32,
        rows: u32,
        cols: u32,
        resizecb: Option<NcResizeCb>,
        flags: u64,
        margin_b: u32,
        margin_r: u32,
    ) -> Self {
        NcPlaneOptions {
            y: y as i32,
            x: x as i32,
            rows,
            cols,
            userptr: null_mut(),
            name: null(),
            resizecb: crate::c_api::ncresizecb_to_c(resizecb),
            flags,
            margin_b,
            margin_r,
        }
    }

    /// New NcPlaneOptions, with flags and horizontal alignment.
    ///
    /// Note: Already includes the
    /// [`NcPlaneOptions::HORALIGNED`][NcPlaneOptions#associatedconstant.HORALIGNED]
    /// flag.
    pub fn with_flags_aligned(
        y: i32,
        align: NcAlign,
        rows: u32,
        cols: u32,
        resizecb: Option<NcResizeCb>,
        flags: u64,
    ) -> Self {
        let flags = NcPlaneOptions::HORALIGNED | flags;
        NcPlaneOptions {
            y: y as i32,
            x: align.into(),
            rows,
            cols,
            userptr: null_mut(),
            name: null(),
            resizecb: crate::c_api::ncresizecb_to_c(resizecb),
            flags,
            margin_b: 0,
            margin_r: 0,
        }
    }
}

/// # Methods
impl NcPlaneOptions {
    /// Returns `true` if it has the [`VERALIGNED`] flag set.
    ///
    /// [`VERALIGNED`]: NcPlaneOptions#associatedconstant.VERALIGNED
    pub const fn is_veraligned(&self) -> bool {
        self.flags & NcPlaneOptions::VERALIGNED != 0
    }

    /// Returns `true` if it has the [`HORALIGNED`] flag set.
    ///
    /// [`HORALIGNED`]: NcPlaneOptions#associatedconstant.HORALIGNED
    pub const fn is_horaligned(&self) -> bool {
        self.flags & NcPlaneOptions::HORALIGNED != 0
    }

    /// Returns `true` if it has the [`MARGINALIZED`] flag set.
    ///
    /// [`MARGINALIZED`]: NcPlaneOptions#associatedconstant.MARGINALIZED
    pub const fn is_marginalized(&self) -> bool {
        self.flags & NcPlaneOptions::MARGINALIZED != 0
    }

    /// Returns `true` if it has the [`FIXED`] flag set.
    ///
    /// [`FIXED`]: NcPlaneOptions#associatedconstant.FIXED
    pub const fn is_fixed(&self) -> bool {
        self.flags & NcPlaneOptions::FIXED != 0
    }

    /// Returns `true` if it has the [`AUTOGROW`] flag set.
    ///
    /// [`AUTOGROW`]: NcPlaneOptions#associatedconstant.AUTOGROW
    pub const fn is_autogrow(&self) -> bool {
        self.flags & NcPlaneOptions::AUTOGROW != 0
    }

    /// Returns `true` if it has the [`VSCROLL`] flag set.
    ///
    /// [`VSCROLL`]: NcPlaneOptions#associatedconstant.VSCROLL
    pub const fn is_vscroll(&self) -> bool {
        self.flags & NcPlaneOptions::VSCROLL != 0
    }
}
