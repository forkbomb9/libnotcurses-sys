//! `NcPlane`

// functions already exported by bindgen : 100
// -------------------------------------------
// (#) test: 9
// (W) wrap: 93
// -------------------------------------------
//W  ncpile_bottom
//W# ncpile_create
//W  ncpile_rasterize
//W  ncpile_render
//W  ncpile_render_to_buffer,
//W  ncpile_render_to_file,
//W  ncpile_top
//W  ncplane_above
//W  ncplane_abs_x
//W  ncplane_abs_y
//W  ncplane_abs_yx
//W  ncplane_as_rgba
//W  ncplane_at_cursor
//W  ncplane_at_cursor_cell
//W  ncplane_at_yx
//W  ncplane_at_yx_cell
//W  ncplane_autogrow_p
//W  ncplane_base
//W  ncplane_below
//W  ncplane_box
//W  ncplane_center_abs
//W  ncplane_contents
//W  ncplane_create
//W  ncplane_cursor_move_rel
//W# ncplane_cursor_move_yx
//W# ncplane_cursor_yx
//W  ncplane_destroy
//W# ncplane_dim_yx
//W  ncplane_dup
//W# ncplane_erase
//W  ncplane_erase_region
//W  ncplane_fadein
//W  ncplane_fadein_iteration
//W  ncplane_fadeout
//W  ncplane_fadeout_iteration
//W  ncplane_format
//W  ncplane_gradient2x1
//W  ncplane_greyscale
//   ncplane_hline_interp
//W# ncplane_home
//W  ncplane_mergedown
//W  ncplane_mergedown_simple
//W  ncplane_move_above
//W  ncplane_move_below
//W  ncplane_move_family_above
//W  ncplane_move_family_below
//W  ncplane_move_yx
//W# ncplane_notcurses
//W# ncplane_notcurses_const
//W  ncplane_off_styles
//W  ncplane_on_styles
//W  ncplane_parent
//W  ncplane_parent_const
//W  ncplane_pixelgeom
//   ncplane_polyfill_yx
//W  ncplane_pulse
//W  ncplane_putc_yx
//W  ncplane_putnstr_aligned
//W  ncplane_putnstr_yx
//W  ncplane_puttext
//W  ncplane_qrcode
//W  ncplane_reparent
//W  ncplane_reparent_family
//W# ncplane_resize
//W  ncplane_resize_marginalized
//W  ncplane_resize_maximize
//W  ncplane_resize_placewithin
//W  ncplane_resize_realign
//W  ncplane_resizecb
//W  ncplane_rotate_ccw
//W  ncplane_rotate_cw
//W  ncplane_scrolling_p
//W  ncplane_scrollup,
//W  ncplane_scrollup_child,
//W  ncplane_set_autogrow
//W  ncplane_set_base
//W  ncplane_set_base_cell
//W  ncplane_set_bg_alpha
//W  ncplane_set_bg_default
//W  ncplane_set_bg_palindex
//W  ncplane_set_bg_rgb
//W  ncplane_set_bg_rgb8
//W  ncplane_set_fg_alpha
//W  ncplane_set_fg_default
//W  ncplane_set_fg_palindex
//W  ncplane_set_fg_rgb
//W  ncplane_set_fg_rgb8
//W  ncplane_set_resizecb
//W  ncplane_set_scrolling
//W  ncplane_set_styles
//   ncplane_set_userptr
//W  ncplane_stain
//W  ncplane_styles
//W  ncplane_translate
//W  ncplane_translate_abs
//   ncplane_userptr
//   ncplane_vline_interp
//W  ncplane_x
//W  ncplane_y
//W  ncplane_yx

// functions manually reimplemented: 49
// ------------------------------------------
// (+) done: 49
// (W) wrap: 45
// (#) test:  9
// ------------------------------------------
//W+ ncplane_ascii_box WIP
//W+ ncplane_bchannel
//W+ ncplane_bg_alpha
//W# ncplane_bg_default_p
//W+ ncplane_bg_rgb
//W+ ncplane_bg_rgb8
//W+ ncplane_box_sized
//W# ncplane_channels
//W# ncplane_dim_x
//W# ncplane_dim_y
//W+ ncplane_double_box
//W+ ncplane_double_box_sized
//W+ ncplane_fchannel
//W+ ncplane_fg_alpha
//W# ncplane_fg_default_p
//W+ ncplane_fg_rgb
//W+ ncplane_fg_rgb8
//W+ ncplane_gradient
//W+ ncplane_gradient_sized
//W+ ncplane_halign
// + ncplane_hline
//W+ ncplane_moverel
//W+ ncplane_move_bottom
//W+ ncplane_move_family_bottom
//W+ ncplane_move_family_top
//W+ ncplane_move_top
//W+ ncplane_perimeter
//W+ ncplane_perimeter_double
//W+ ncplane_perimeter_rounded
//W+ ncplane_putc
//W+ ncplane_putchar
//W+ ncplane_putchar_stained
//W+ ncplane_putchar_yx
//W+ ncplane_putegc
//W+ ncplane_putegc_stained
//W+ ncplane_putegc_yx
//W+ ncplane_putnstr
//W+ ncplane_putstr
//W+ ncplane_putstr_aligned
//W+ ncplane_putstr_stained
//W+ ncplane_putstr_yx
//W# ncplane_resize_simple
// + ncplane_rounded_box
// + ncplane_rounded_box_sized
//W# ncplane_set_bchannel
//W# ncplane_set_channels
//W# ncplane_set_fchannel
//W+ ncplane_valign
// + ncplane_vline

// wont implement:
// ----------------
//   ncplane_putegc_stained
//   ncplane_putegc_yx
//   ncplane_putwegc_stained
//   ncplane_putwstr_stained
//   ncplane_set_bg_rgb8_clipped
//   ncplane_set_fg_rgb8_clipped
//   ncplane_vprintf_aligned
//   ncplane_vprintf_stained
//   ncplane_vprintf_yx
//
//   ncplane_putegc
//   ncplane_putwc
//   ncplane_putwc_stained
//   ncplane_putwc_yx
//   ncplane_putwegc
//   ncplane_putwegc_yx
//   ncplane_putwstr
//   ncplane_putwstr_aligned
//   ncplane_putwstr_yx
//   ncplane_vprintf

pub(crate) mod helpers;
mod methods;
pub(crate) mod options;
pub(crate) mod reimplemented;
#[cfg(test)]
pub(crate) mod test;

pub use options::{NcPlaneFlags, NcPlaneOptions, NcPlaneOptionsBuilder};

// NcPlane
//
/// A drawable [`Nc`][crate::Nc] notcurses surface, composed of [`NcCell`]s.
///
/// Unites a *CellMatrix* with an *EgcPool* (a matrix of `NcCells` with a pool
/// of extended grapheme characters).
///
/// `type in C: ncplane (struct)`
///
/// # About planes and piles
///
/// A given notcurses context is made up of one or more piles.
///
/// A pile is made up of [`NcPlane`]s, totally ordered on a z-axis.
///
/// You can't manage the piles directly, but only the `NcPlanes`.
///
/// A pile is destroyed when all its planes are destroyed or moved to other
/// piles.
///
/// A pile has a top and bottom plane (this might be a single `NcPlane`),
/// and one or more root planes (`NcPlane`s which are bound to themselves).
///
/// Multiple threads can concurrently operate on distinct piles, rendering or
/// mutating it, while another thread concurrently renders or mutates another.
///
/// Each `NcPlane` is part of one and only one pile. By default, an `NcPlane` is
/// part of the same pile that contains the `NcPlane` to which it is bound.
///
/// When an `NcPlane` is created without being bound to another `NcPlane`, then
/// it becomes the root plane, top, and bottom of a new pile. As a root plane,
/// it is bound to itself.
///
/// A new pile can also be created by reparenting an `NcPlane` to itself,
/// though if the plane is already a root plane, this is a no-op.
///
/// When an `NcPlane` is moved to a different pile (whether new or preexisting),
/// any `NcPlane`s which were bound to it are rebound to its previous parent.
/// If the `NcPlane` was a root plane of some pile, any bound planes become root
/// planes. The new `NcPlane` is placed immediately atop its new parent on its
/// new pile's z-axis.
///
/// When [`NcPlane::reparent_family`] is used, all `NcPlanes` bound to the
/// reparented `NcPlane` are moved along with it. Their relative z-order is
/// maintained.
//
/// Rendering reduces a pile of `NcPlane`s to a single `NcPlane`, proceeding
/// from the top to the bottom along a pile's z-axis. The result is a matrix of
/// [`NcCell`]s. Rasterizing takes this matrix, together with the
/// current state of the visual area, and produces a stream of optimized control
/// sequences and `EGC`s for the terminal. By writing this stream to the
/// terminal, the physical display is synced to some pile's `NcPlane`s.
///
/// [`NcPlane.render`] performs the first of these tasks for the pile of which
/// the plane is a part. The output is maintained internally; calling `render`
/// again on the same pile will replace this state with a fresh render.
/// Multiple piles can be concurrently rendered.
///
/// [`NcPlane.rasterize`] performs rasterization, and writes the result to the
/// terminal. It is a blocking call, and only one rasterization operation may
/// proceed at a time.
///
/// It is necessary to call `NcPlane.rasterize` to generate any visible output;
/// the various *output calls* only draw to the virtual `NcPlane`s. Most of the
/// notcurses `statistics` are updated as a result of a render, and screen
/// geometry is refreshed (similarly to [`Nc.refresh`]) following the render.
///
/// # Methods & Associated Functions
///
/// - [Constructors & Destructors](#ncplane-constructors--destructors)
///
/// Methods:
/// - [`NcAlpha`](#ncplane-methods-ncalphabits)
/// - [`NcChannel` & `NcChannels`](#ncplane-methods-ncchannel)
/// - [`NcRgb`, components & default color](#ncplane-methods-ncrgb-components--default-color)
/// - [`NcStyle` & `NcPaletteIndex`](#ncplane-methods-ncstylemask--paletteindex)
/// - [`NcCell` & strings](#ncplane-methods-nccell--strings)
/// - [cursor](#ncplane-methods-cursor)
/// - [`NcPlane` & `Nc`](#ncplane-methods-ncplane--nc)
/// - [boxes & perimeters](#ncplane-methods-boxes--perimeters)
/// - [Size, position & alignment](#ncplane-methods-size-position--alignment)
/// - [fading, gradients & greyscale](#ncplane-methods-fading-gradients--greyscale)
/// - [*other*](#ncplane-methods-other)
///
/// [`NcCell`]: crate::NcCell
/// [`NcPlane::reparent_family`]: NcPlane#method.reparent_family
/// [`NcPlane.render`]: crate::NcPlane#method.render
/// [`NcPlane.rasterize`]: crate::NcPlane#method.render
/// [`Nc.refresh`]: crate::Nc#method.refresh
pub type NcPlane = crate::c_api::ffi::ncplane;
