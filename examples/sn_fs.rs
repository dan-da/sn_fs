// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use brb_membership::actor::ed25519::SigningActor;
use brb_membership::SigningActor as SigningActorTrait;
use log::error;
use openat::{Dir, SimpleType};
use sn_fs::SnFs;
use std::env;
use std::ffi::OsStr;
use std::path::Path;

fn main() {
    env_logger::builder().format_timestamp_nanos().init();
    let mountpoint = match env::args_os().nth(1) {
        Some(v) => v,
        None => {
            print_usage();
            return;
        }
    };

    // We use Dir::open() to get access to the mountpoint directory
    // before the mount occurs.  This handle enables us to later create/write/read
    // "real" files beneath the mountpoint even though other processes will only
    // see the filesystem view that our SnFs provides.
    let mountpoint_fd = match Dir::open(Path::new(&mountpoint)) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Unable to open {:?}.  {:?}", mountpoint, e);
            return;
        }
    };

    // Actor is ed25519 Public Key
    let actor = SigningActor::default().actor();

    // Notes:
    //  1. todo: these options should come from command line.
    //  2. allow_other enables other users to read/write.  Required for testing chown.
    //  3. allow_other requires that `user_allow_other` is in /etc/fuse.conf.
    //    let options = ["-o", "ro", "-o", "fsname=safefs"]    // -o ro = mount read only
    let options = ["-o", "fsname=sn_fs"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    // mount the filesystem.
    if let Err(e) = fuse::mount(SnFs::new(actor, mountpoint_fd), &mountpoint, &options) {
        eprintln!("Mount failed.  {:?}", e);
        return;
    }

    // Delete all "real" files (each file representing content of 1 inode) under mount point.
    // this code should be in SnFs::destroy(), but its not getting called.
    // Seems like a fuse bug/issue.
    let mountpoint_fd = Dir::open(Path::new(&mountpoint)).unwrap();
    if let Ok(entries) = mountpoint_fd.list_dir(".") {
        for result in entries {
            if let Ok(entry) = result {
                if entry.simple_type() == Some(SimpleType::File)
                    && mountpoint_fd
                        .remove_file(Path::new(entry.file_name()))
                        .is_err()
                {
                    error!("Unable to remove file {:?}", entry.file_name());
                }
            }
        }
    }
}

fn print_usage() {
    eprintln!("Usage: sn_fs <mountpoint_path>");
}
