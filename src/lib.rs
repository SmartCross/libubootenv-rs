#![allow(clippy::needless_return)]

mod libubootenv;
use libubootenv::*;

use nix::errno::Errno;
use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

pub type Result<T> = std::result::Result<T, Errno>;


pub struct UBootContext {
    context: *mut uboot_ctx,
    opened: bool,
}

impl Drop for UBootContext {
    fn drop(&mut self) {
        unsafe {
            if self.opened {
                self.close();
            }
            libuboot_exit(self.context);
        }
    }
}

fn ret_or_val<T>(ret: i32, val: T) -> Result<T> {
    match ret {
        0 => Ok(val),
        _ => Err(nix::errno::Errno::from_i32(ret)),
    }
}

impl UBootContext {
    pub fn initialize() -> Result<UBootContext> {
        unsafe {
            let mut ptr: *mut uboot_ctx = null_mut();
            let ret = libuboot_initialize(&mut ptr, null_mut());
            return ret_or_val(ret, UBootContext { context: ptr, opened: false });
        }
    }

    pub fn open(&mut self) -> Result<()> {
        let ret = unsafe { libuboot_open(self.context) };
        if ret == 0 {
            self.opened = true;
        }
        return ret_or_val(ret, ());
    }

    pub fn env_store(&mut self) -> Result<()> {
        assert!(self.opened);
        let ret = unsafe { libuboot_env_store(self.context) };
        return ret_or_val(ret, ());
    }

    pub fn close(&mut self) {
        unsafe {
            libuboot_close(self.context);
        }
        self.opened = false;
    }

    pub fn load_file(&mut self, path: &str) -> Result<()> {
        let path_c = CString::new(path).unwrap();
        let ret = unsafe { libuboot_load_file(self.context, path_c.as_ptr()) };
        if ret == 0 {
            self.opened = true;
        }
        return ret_or_val(ret, ());
    }

    pub fn read_config(&mut self, path: &str) -> Result<()> {
        let path_c = CString::new(path).unwrap();
        let ret = unsafe { libuboot_read_config(self.context, path_c.as_ptr()) };
        return ret_or_val(ret, ());
    }

    pub fn set_env(&mut self, varname: &str, value: &str) -> Result<()> {
        assert!(self.opened);
        let varn_c = CString::new(varname).unwrap();
        let val_c = CString::new(value).unwrap();
        let ret = unsafe {
            libuboot_set_env(self.context, varn_c.as_ptr(), val_c.as_ptr())
        };
        return ret_or_val(ret, ());
    }

    pub fn get_env(&mut self, varname: &str) -> Option<String> {
        assert!(self.opened);
        let result: Option<String>;
        let varn_c = CString::new(varname).unwrap();
        unsafe {
            let ret = libuboot_get_env(self.context, varn_c.as_ptr());
            if ret.is_null() {
                result = None;
            } else {
                let str = CStr::from_ptr(ret);
                result = str.to_str().ok().map(|g| g.to_owned());
                libc::free(ret as *mut libc::c_void);
            }
        }
        return result;
    }
}
