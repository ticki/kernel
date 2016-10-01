use core::slice;

use super::*;

pub trait Scheme {
    fn handle(&self, packet: &mut Packet) {
        packet.a = Error::mux(match packet.a {
            SYS_OPEN => {
                self.open(unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) },
                          packet.d)
            }
            SYS_MKDIR => {
                self.mkdir(unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) },
                           packet.d)
            }
            SYS_RMDIR => {
                self.rmdir(unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) })
            }
            SYS_UNLINK => {
                self.unlink(unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) })
            }

            SYS_DUP => self.dup(packet.b),
            SYS_READ => {
                self.read(packet.b,
                          unsafe { slice::from_raw_parts_mut(packet.c as *mut u8, packet.d) })
            }
            SYS_WRITE => {
                self.write(packet.b,
                           unsafe { slice::from_raw_parts(packet.c as *const u8, packet.d) })
            }
            SYS_LSEEK => self.seek(packet.b, packet.c, packet.d),
            SYS_FEVENT => self.fevent(packet.b, packet.c),
            SYS_FPATH => {
                self.fpath(packet.b,
                           unsafe { slice::from_raw_parts_mut(packet.c as *mut u8, packet.d) })
            }
            SYS_FSTAT => self.fstat(packet.b, unsafe { &mut *(packet.c as *mut Stat) }),
            SYS_FSYNC => self.fsync(packet.b),
            SYS_FTRUNCATE => self.ftruncate(packet.b, packet.c),
            SYS_CLOSE => self.close(packet.b),

            _ => Err(Error::new(ENOSYS)),
        });
    }

    // Scheme operations

    #[allow(unused_variables)]
    fn open(&self, path: &[u8], flags: usize) -> Result<usize> {
        Err(Error::new(ENOENT))
    }

    #[allow(unused_variables)]
    fn mkdir(&self, path: &[u8], mode: usize) -> Result<usize> {
        Err(Error::new(ENOENT))
    }

    #[allow(unused_variables)]
    fn rmdir(&self, path: &[u8]) -> Result<usize> {
        Err(Error::new(ENOENT))
    }

    #[allow(unused_variables)]
    fn unlink(&self, path: &[u8]) -> Result<usize> {
        Err(Error::new(ENOENT))
    }

    // Resource operations
    #[allow(unused_variables)]
    fn dup(&self, old_id: usize) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn read(&self, id: usize, buf: &mut [u8]) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn write(&self, id: usize, buf: &[u8]) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn seek(&self, id: usize, pos: usize, whence: usize) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fevent(&self, id: usize, flags: usize) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fpath(&self, id: usize, buf: &mut [u8]) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fstat(&self, id: usize, stat: &mut Stat) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fsync(&self, id: usize) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn ftruncate(&self, id: usize, len: usize) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn close(&self, id: usize) -> Result<usize> {
        Err(Error::new(EBADF))
    }
}
