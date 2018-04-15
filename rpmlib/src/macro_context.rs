//! RPM macros are configuration parameters that have largely replaced the
//! previous rpmrc system.

use failure::Error;
use rpmlib_sys::rpmlib as ffi;
use std::ffi::CString;

/// Scopes in which macros are defined
pub struct MacroContext(ffi::rpmMacroContext);

/// Obtain the default global context
impl Default for MacroContext {
    fn default() -> MacroContext {
        unsafe { MacroContext(ffi::rpmGlobalMacroContext) }
    }
}

impl MacroContext {
    /// Define a macro in this context. Macros take the form:
    ///
    /// `<name>[(opts)] <body>`
    ///
    /// Level defines the macro recursion level (0 is the entry API)
    pub fn define(&self, macro_string: &str, level: isize) -> Result<(), Error> {
        let cstr = CString::new(macro_string).map_err(|e| format_err!("{}", e))?;

        unsafe {
            ffi::rpmDefineMacro(self.0, cstr.as_ptr(), level as i32);
        }

        Ok(())
    }

    /// Delete a macro from this context.
    pub fn delete(&self, name: &str) -> Result<(), Error> {
        let cstr = CString::new(name).unwrap();

        unsafe {
            ffi::delMacro(self.0, cstr.as_ptr());
        }

        Ok(())
    }
}
