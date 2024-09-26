// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

//==================================================================================================
// Imports
//==================================================================================================

use ::core::fmt;

//==================================================================================================
// Structures
//==================================================================================================

/// A formatter object
pub struct Logger;

//==================================================================================================
// Trait Implementations
//==================================================================================================

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = ::sys::kcall::debug::debug(s.as_ptr(), s.len());
        Ok(())
    }
}
