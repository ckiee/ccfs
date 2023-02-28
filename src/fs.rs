


use fuse_mt::{FileAttr, FilesystemMT};



use log::debug;
use log::{info, warn};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use std::{time::SystemTime};




//
// XXX: fuse_mt seems to be very cursed, might have to switch back to fuser and just suffer
// a bunch more. or maybe ssh+ sshfs?
//
//
//
//
//
pub struct CCFS {
    fd_map: HashMap<u64, PathBuf>,
}

impl CCFS {
    pub fn new() -> Self {
        CCFS {
            fd_map: HashMap::new(),
        }
    }
}

impl FilesystemMT for CCFS {
    fn init(&self, _req: fuse_mt::RequestInfo) -> fuse_mt::ResultEmpty {
        info!("mounted CCFS");
        Ok(())
    }
    fn destroy(&self) {
        // Nothing.
    }

    fn getattr(
        &self,
        _req: fuse_mt::RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
    ) -> fuse_mt::ResultEntry {
        let null_time = SystemTime::UNIX_EPOCH;
        Ok((
            Duration::from_millis(1),
            FileAttr {
                size: 0,
                blocks: 0,
                atime: null_time,
                mtime: null_time,
                ctime: null_time,
                crtime: null_time,
                // TODO: dir
                kind: fuse_mt::FileType::RegularFile,
                perm: 0o655,
                uid: _req.uid,
                gid: _req.gid,
                // no-op
                nlink: 0,
                rdev: 0,
                flags: 0,
            },
        ))
    }

    // fn truncate(
    //     &self,
    //     _req: fuse_mt::RequestInfo,
    //     _path: &Path,
    //     _fh: Option<u64>,
    //     _size: u64,
    // ) -> fuse_mt::ResultEmpty {
    //     unimplemented!();
    // }

    fn mkdir(
        &self,
        _req: fuse_mt::RequestInfo,
        _parent: &Path,
        _name: &std::ffi::OsStr,
        _mode: u32,
    ) -> fuse_mt::ResultEntry {
        unimplemented!();
    }

    fn rmdir(
        &self,
        _req: fuse_mt::RequestInfo,
        _parent: &Path,
        _name: &std::ffi::OsStr,
    ) -> fuse_mt::ResultEmpty {
        unimplemented!();
    }

    fn rename(
        &self,
        _req: fuse_mt::RequestInfo,
        _parent: &Path,
        _name: &std::ffi::OsStr,
        _newparent: &Path,
        _newname: &std::ffi::OsStr,
    ) -> fuse_mt::ResultEmpty {
        unimplemented!();
    }

    fn read(
        &self,
        _req: fuse_mt::RequestInfo,
        _path: &Path,
        _fh: u64,
        _offset: u64,
        _size: u32,
        callback: impl FnOnce(fuse_mt::ResultSlice<'_>) -> fuse_mt::CallbackResult,
    ) -> fuse_mt::CallbackResult {
        dbg!("=== read", _req, _path, _fh, _offset, _size);
        // callback(Err(libc::EINVAL))
        callback(Ok(&[]))
    }

    fn write(
        &self,
        _req: fuse_mt::RequestInfo,
        _path: &Path,
        _fh: u64,
        _offset: u64,
        _data: Vec<u8>,
        _flags: u32,
    ) -> fuse_mt::ResultWrite {
        unimplemented!();
    }

    fn release(
        &self,
        _req: fuse_mt::RequestInfo,
        _path: &Path,
        _fh: u64,
        _flags: u32,
        _lock_owner: u64,
        _flush: bool,
    ) -> fuse_mt::ResultEmpty {
        Ok(())
    }

    fn readdir(
        &self,
        _req: fuse_mt::RequestInfo,
        _path: &Path,
        _fh: u64,
    ) -> fuse_mt::ResultReaddir {
        unimplemented!();
    }

    fn releasedir(
        &self,
        _req: fuse_mt::RequestInfo,
        _path: &Path,
        _fh: u64,
        _flags: u32,
    ) -> fuse_mt::ResultEmpty {
        unimplemented!();
    }

    fn access(&self, _req: fuse_mt::RequestInfo, _path: &Path, _mask: u32) -> fuse_mt::ResultEmpty {
        let root = PathBuf::from_str("/").unwrap();
        let file_exists = _path == &root;
        if !file_exists {
            warn!("access? ENO!");
            return Err(libc::EACCES);
        }
        debug!("access? yes!");
        Ok(())
    }

    fn open(
        &self,
        _req: fuse_mt::RequestInfo,
        _path: &Path,
        _flags: u32,
    ) -> fuse_mt::ResultOpen {
        dbg!(_req, _path, _flags);
        // assume no files exist. for now.
        if (_flags & (libc::O_CREAT as u32)) == 0 {
            // finneee i'll create just this for you..

            // need a bazillion Arc<whatever< whatever< 's for this
            // self.fd_map.insert(_req.unique /*just convenient*/, _path.to_owned());

            Ok((_req.unique, 0))
        } else {
            // muhahaha
            Err(libc::EACCES)
        }
    }

    fn create(
        &self,
        _req: fuse_mt::RequestInfo,
        _parent: &Path,
        _name: &std::ffi::OsStr,
        _mode: u32,
        _flags: u32,
    ) -> fuse_mt::ResultCreate {
        unimplemented!();
    }

    //
    // will never be implemented
    //

    fn symlink(
        &self,
        _req: fuse_mt::RequestInfo,
        _parent: &Path,
        _name: &std::ffi::OsStr,
        _target: &Path,
    ) -> fuse_mt::ResultEntry {
        Err(libc::ENOSYS)
    }

    fn unlink(
        &self,
        _req: fuse_mt::RequestInfo,
        _parent: &Path,
        _name: &std::ffi::OsStr,
    ) -> fuse_mt::ResultEmpty {
        Err(libc::ENOSYS)
    }

    fn readlink(&self, _req: fuse_mt::RequestInfo, _path: &Path) -> fuse_mt::ResultData {
        Err(libc::ENOSYS)
    }

    fn mknod(
        &self,
        _req: fuse_mt::RequestInfo,
        _parent: &Path,
        _name: &std::ffi::OsStr,
        _mode: u32,
        _rdev: u32,
    ) -> fuse_mt::ResultEntry {
        Err(libc::ENOSYS)
    }

    fn link(
        &self,
        _req: fuse_mt::RequestInfo,
        _path: &Path,
        _newparent: &Path,
        _newname: &std::ffi::OsStr,
    ) -> fuse_mt::ResultEntry {
        unimplemented!();
    }
}
