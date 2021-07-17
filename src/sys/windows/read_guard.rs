use winapi::um::fileapi::UnlockFile;

use std::ops;
use std::os::windows::prelude::*;

use super::utils::syscall;
use super::FileLock;

#[derive(Debug)]
pub struct FileLockReadGuard<'lock, T: AsRawHandle> {
    pub(crate) lock: &'lock FileLock<T>,
}

impl<T: AsRawHandle> ops::Deref for FileLockReadGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.inner
    }
}

impl<T: AsRawHandle> Drop for FileLockReadGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let handle = self.lock.inner.as_raw_handle();
        syscall(unsafe { UnlockFile(handle, 0, 0, 1, 0) })
            .expect("Could not unlock the file descriptor");
    }
}
