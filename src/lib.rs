//! `libnotcurses-sys` is a low-level Rust wrapper for the
//! [notcurses C library](https://www.github.com/dankamongmen/notcurses/)
//!
//! It is built with several layers of zero-overhead abstractions
//! over the C functions and pointers accessed through FFI.
//!
//! It adds greater safety and type correctness over the underlying C library
//! API, while trying to remain very close to it.
//!
//! It offers the choice of using it [*more like Rust*](#like-rust)
//! and/or [*more like C*](#like-c).
//!
//! ## Like Rust
//!
//! Where you use the more safely wrapped types, with its methods and
//! constructors, and error handling with the `NcResult` enum:
//!
//! ### Example
#![doc = concat!["```ignore\n", include_str!("../examples/hello-world-rust.rs"), "\n```" ]]
//!
//! ### Notes on the Rust API
//!
//! The `Drop` trait is not implemented for any wrapping type in this library
//! over structures created by the underlying C library.
//!
//! This means you still have to manually call the `stop()` method for `Nc`
//! and `NcDirect` objects, and the `destroy()` method for the rest of types that
//! allocate, (like `NcPlane`, `NcMenu`…) at the end of their scope.
//!
//! But they do implement methods and use `NcResult` as the return type,
//! for handling errors in the way we are used to in Rust.
//!
//! For the types that don't allocate, most are based on primitives like `i32`,
//! `u32`, `u64`… without a name in the C library. In Rust they are type aliased
//! for the C API (e.g.: `NcChannel_u32`, `NcLogLevel_i32`, `NcStyle_u16`…), to
//! leverage type checking. And for the Rust API they are wrapped as unit structs
//! or enums, with associated methods and constants (e.g. `NcChannel`,
//! `NcLogLevel`, `NcStyle`…).
//!
//! Several methods are declared unsafe when they have addittional contracts to
//! manually upheld in order to avoid *UB*.
//!
//! ### even more like Rust
//!
//! The *WIP* sister crate
//! [`notcurses`](https://github.com/dankamongmen/notcurses-rs) will eventually
//! offer a *closer to Rust*, higher-level, safer, and simpler API, and make it
//! easier to interoperate with the rest of the Rust ecosystem.
//!
//! ## Like C
//!
//! You can access the imported, or reimplemented C API functions directly,
//! and use it in a very similar way as the C library is used.
//!
//! It requires more use of unsafe, since it has less safer abstractions.
//!
//! Error handling is done this way by checking the returned `NcResult_i32`,
//! or in case of receiving a pointer, by comparing it to `null_mut()`.
//!
//! ### Example
#![doc = concat!["```ignore\n", include_str!("../examples/hello-world-c.rs"), "\n```" ]]
//!
//! ### The `notcurses` C API docs
//!
//! - [API reference (man pages)](https://notcurses.com/)
//! - [Wiki Page](https://nick-black.com/dankwiki/index.php/Notcurses)
//! - [The Book Guide (pdf)](https://nick-black.com/htp-notcurses.pdf)
//! - [USAGE.md](https://github.com/dankamongmen/notcurses/blob/master/USAGE.md)
//! - [HACKING.md](https://github.com/dankamongmen/notcurses/blob/master/doc/HACKING.md)
//! - [Doxygen Documentation](https://nick-black.com/notcurses/html/index.html)
//! - [FOSDEM 2021 presentation](https://fosdem.org/2021/schedule/event/notcurses/)
//!
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![allow(clippy::too_many_arguments, clippy::needless_doctest_main)]

mod align;
mod alpha;
mod bindings;
mod blitter;
mod r#box;
mod capabilities;
mod cell;
mod channel;
mod direct;
mod error;
mod fade;
mod fd;
mod file;
mod input;
mod key;
mod log_level;
mod macros;
mod metric;
mod notcurses;
mod palette;
mod pixel;
mod plane;
mod resizecb;
mod rgb;
mod scale;
mod stats;
mod string;
mod style;
mod time;
mod visual;

pub mod widgets;

// wrapper types and traits
//
// Note that the names of the implemented traits can't coincide for type aliases
// with the same underlying primitive, like in the case of `NcAlign` & `NcScale`
// in which case are both aliases over `u32`.
pub use align::NcAlign;
pub use alpha::NcAlpha;
pub use blitter::NcBlitter;
pub use capabilities::NcCapabilities;
pub use cell::NcCell;
pub use channel::{NcChannel, NcChannels};
pub use direct::{NcDirect, NcDirectFlags};
pub use error::{NcError, NcResult};
pub use fade::{NcFadeCb, NcFadeCtx};
pub use fd::{NcFdPlane, NcFdPlaneOptions, NcSubproc, NcSubprocOptions};
pub use file::NcFile;
pub use input::{NcInput, NcInputType, NcMiceEvents, NcReceived};
pub use key::{NcKey, NcKeyMod};
pub use log_level::NcLogLevel;
pub use notcurses::{Nc, NcFlags, NcOptions};
pub use palette::{NcPalette, NcPaletteIndex};
pub use pixel::{NcPixel, NcPixelGeometry, NcPixelImpl};
pub use plane::{NcPlane, NcPlaneFlags, NcPlaneOptions, NcPlaneOptionsBuilder};
pub use r#box::NcBoxMask;
pub use resizecb::NcResizeCb;
pub use rgb::{NcRgb, NcRgba};
pub use scale::NcScale;
pub use stats::NcStats;
pub use string::NcString;
pub use style::NcStyle;
pub use time::NcTime;
pub use visual::{
    NcVisual, NcVisualFlags, NcVisualGeometry, NcVisualOptions, NcVisualOptionsBuilder,
};

pub mod c_api {
    //! The `C API`, including structs, constants, functions and type aliases.
    //!
    //! Includes also both automatically imported functions by bindgen, and
    //! manually wrapped and reimplemented global functions.

    // public re-export of external crates:
    pub use libc;

    pub mod ffi {
        //! Rust FFI bindings, automatically generated with bindgen.
        pub use crate::bindings::ffi::*;
    }

    // public re-export of imported functions & structs:
    #[doc(inline)]
    pub use crate::bindings::*;

    // public re-export of reimplemented functions:
    pub use crate::capabilities::reimplemented::*;
    pub use crate::cell::reimplemented::*;
    pub use crate::channel::reimplemented::*;
    pub use crate::direct::reimplemented::*;
    pub use crate::input::reimplemented::*;
    pub use crate::key::reimplemented::*;
    pub use crate::metric::reimplemented::*;
    pub use crate::notcurses::reimplemented::*;
    pub use crate::palette::reimplemented::*;
    pub use crate::pixel::reimplemented::*;
    pub use crate::plane::reimplemented::*;

    // public re-export of c_api constants & types:
    pub use crate::align::c_api::*;
    pub use crate::alpha::c_api::*;
    pub use crate::blitter::c_api::*;
    pub use crate::channel::c_api::*;
    pub use crate::direct::c_api::*;
    pub use crate::error::c_api::*;
    pub use crate::input::c_api::*;
    pub use crate::key::c_api::*;
    pub use crate::log_level::c_api::*;
    pub use crate::metric::c_api::*;
    pub use crate::notcurses::c_api::*;
    pub use crate::palette::c_api::*;
    pub use crate::pixel::c_api::*;
    pub use crate::plane::options::c_api::*;
    pub use crate::r#box::c_api::*;
    pub use crate::resizecb::c_api::*;
    pub use crate::rgb::c_api::*;
    pub use crate::scale::c_api::*;
    pub use crate::style::c_api::*;
    pub use crate::visual::c_api::*;
    pub use crate::widgets::menu::c_api::*;
    pub use crate::widgets::plot::c_api::*;
    pub use crate::widgets::progbar::c_api::*;
    pub use crate::widgets::reader::c_api::*;
    pub use crate::widgets::reel::c_api::*;
    pub use crate::widgets::tabbed::c_api::*;

    // private re-export of helper functions for testing:
    mod helpers {
        #![allow(unused_imports)]
        pub use crate::notcurses::helpers::*;
        pub use crate::plane::helpers::*;
    }
    #[allow(unused_imports)]
    pub(crate) use helpers::*;
}

/// The library `examples/`.
///
/// Here you can see and navigate the examples as individual modules,
/// as well as the common utility objects from [`utils`][examples::utils].
///
// Note that `cargo doc` doesn't detect changes made inside the `/examples/`
// directory, unless something about the `examples` module has been modified,
// (This maybe a side effect of using the [module `path` attribute][0])
//
// [0]: https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute
#[path = "../examples"]
#[cfg(any(doc))]
pub mod examples {
    #![allow(dead_code)]

    pub mod utils;
    pub use utils::{Canvas, Counter};

    // individual examples

    #[path = "erase_region.rs"]
    pub mod example_erase_region;
    #[path = "info.rs"]
    pub mod example_info;
    #[path = "input.rs"]
    pub mod example_input;
    #[path = "pixel-cell.rs"]
    pub mod example_pixel_cell;
    #[path = "planes.rs"]
    pub mod example_planes;
}
