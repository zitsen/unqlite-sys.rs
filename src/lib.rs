//! Rust bindings for [UnQlite][] library.
//!
//! [![travis-badge][]][travis] [![release-badge][]][cargo] [![downloads]][cargo]
//! [![docs-badge][]][docs] [![license-badge][]][cargo]
//!
//! As its official site says, **UnQlite** is
//! > An Embeddable NoSQL Database Engine.
//!
//! Please see [UnQLite C/C++ API Reference][] for full API documents.
//!
//! # Usage
//!
//! This crate provides several features for [UnQlite compile-time
//! options][apidoc-compile]:
//!
//! * **enable-threads**: equivalent to `UNQLITE_ENABLE_THREADS` enabled.
//! * **jx9-disable-builtin-func**: equivalent to `JX9_DISABLE_BUILTIN_FUNC` enabled.
//! * **jx9-enable-math-fuc**: equivalent to `JX9_ENABLE_MATH_FUNC` enabled.
//! * **jx9-disable-disk-io**: equivalent to `JX9_DISABLE_DISK_IO` enabled.
//! * **enable-jx9-hash-io**: equivalent to `UNQLITE_ENABLE_JX9_HASH_IO` enabled.
//!
//! To provide the same default behavior as original C does, non of the features
//! is enabled by default. When you want some features, such as **enable-threads**,
//! just config in `Cargo.toml`:
//!
//! ```toml
//! [dependencies.unqlite-sys]
//! version = "0.3"
//! features = [ "enable-threads" ]
//! ```
//!
//! For multiple features just add them in toml `features` array.
//!
//! # Threadsafe
//!
//! Note that even "enable-threads" is featured in your crate, it's not meant
//! that your code is threadsafe.
//!
//! > When UnQLite has been compiled with threading support then the threading mode can be altered
//! at run-time using the unqlite_lib_config() interface together with one of these verbs:
//!
//!   >> UNQLITE_LIB_CONFIG_THREAD_LEVEL_SINGLE
//!
//!   >> UNQLITE_LIB_CONFIG_THREAD_LEVEL_MULTI
//!
//! > Platforms others than Windows and UNIX systems must install their own mutex subsystem via
//! unqlite_lib_config() with a configuration verb set to UNQLITE_LIB_CONFIG_USER_MUTEX. Otherwise
//! the library is not threadsafe.
//! >
//! > Note that you must link UnQLite with the POSIX threads library under UNIX systems (i.e:
//! -lpthread).
//!
//! To use in multithread cases, that is **threadsafe**, you may use like this:
//!
//! ```no_run
//! extern crate unqlite_sys as ffi;
//! use ffi::constants as ffic;
//!
//! fn main() {
//!     unsafe {
//!         ffi::unqlite_lib_config(ffic::UNQLITE_LIB_CONFIG_THREAD_LEVEL_MULTI);
//!         ffi::unqlite_lib_init();
//!         assert_eq!(ffi::unqlite_lib_is_threadsafe(), 1);
//!
//!         // do stuff ...
//!
//!     }
//! }
//! ```
//!
//! [UnQlite]: http://unqlite.org
//! [UnQLite C/C++ API Reference]: http://unqlite.org/c_api.html
//! [travis-badge]: https://img.shields.io/travis/zitsen/unqlite-sys.rs.svg?style=flat-square
//! [travis]: https://travis-ci.org/zitsen/unqlite-sys.rs
//! [release-badge]: https://img.shields.io/crates/v/unqlite-sys.svg?style=flat-square
//! [downloads]: https://img.shields.io/crates/d/unqlite-sys.svg?style=flat-square
//! [cargo]: https://crates.io/crates/unqlite-sys
//! [docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
//! [docs]: https://zitsen.github.io/unqlite-sys.rs
//! [license-badge]: https://img.shields.io/crates/l/unqlite-sys.svg?style=flat-square
//! [apidoc-compile]: http://unqlite.org/c_api_const.html#compile_time
pub mod constants;

pub use self::bindgen::*;

#[allow(dead_code, non_snake_case, non_camel_case_types)]
mod bindgen;

#[cfg(test)]
mod tests {
    use super::{
        unqlite_lib_config,
        unqlite_lib_init,
        unqlite_lib_is_threadsafe,
        unqlite_lib_shutdown
    };
    use super::constants as ffic;

    #[cfg(feature = "enable-threads")]
    #[test]
    fn is_threadsafe() {
        unsafe {
            unqlite_lib_config(ffic::UNQLITE_LIB_CONFIG_THREAD_LEVEL_MULTI);
            unqlite_lib_init();
            assert_eq!(unqlite_lib_is_threadsafe(), 1);
            unqlite_lib_shutdown();

            unqlite_lib_config(ffic::UNQLITE_LIB_CONFIG_THREAD_LEVEL_SINGLE);
            unqlite_lib_init();
            assert_eq!(unqlite_lib_is_threadsafe(), 0);
            unqlite_lib_shutdown();
        }
    }

    #[cfg(not(feature = "enable-threads"))]
    #[test]
    fn not_threadsafe() {
        unsafe {
            unqlite_lib_config(ffic::UNQLITE_LIB_CONFIG_THREAD_LEVEL_MULTI);
            unqlite_lib_init();
            assert_eq!(unqlite_lib_is_threadsafe(), 0);
            unqlite_lib_shutdown();

            unqlite_lib_config(ffic::UNQLITE_LIB_CONFIG_THREAD_LEVEL_SINGLE);
            unqlite_lib_init();
            assert_eq!(unqlite_lib_is_threadsafe(), 0);
            unqlite_lib_shutdown();
        }
    }
}
