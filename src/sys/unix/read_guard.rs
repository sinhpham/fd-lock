use libc::{flock, LOCK_UN};
use std::ops;
use std::os::unix::io::AsRawFd;

use super::utils::syscall;
use super::FileLock;

#[derive(Debug)]
pub struct FileLockReadGuard<'lock, T: AsRawFd> {
    lock: &'lock FileLock<T>,
}

impl<'lock, T: AsRawFd> FileLockReadGuard<'lock, T> {
    pub(crate) fn new(lock: &'lock FileLock<T>) -> Self {
        Self { lock }
    }
}

impl<T: AsRawFd> ops::Deref for FileLockReadGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.inner
    }
}

impl<T: AsRawFd> Drop for FileLockReadGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let fd = self.lock.inner.as_raw_fd();
        let _ = syscall(unsafe { flock(fd, LOCK_UN) });
    }
}
