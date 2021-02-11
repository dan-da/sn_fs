// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! sn_fs: A prototype FUSE filesystem/library that uses crdt_tree
//! for storing directory structure metadata.
//!
//! This prototype operates only as a local filesystem.
//! The plan is to make it into a network filesystem utilizing the CRDT
//! properties to ensure that replicas sync/converge correctly.
//!
//! In this implementation the contents of each file are stored in
//! a corresponding file in the underlying filesystem whose name
//! is the inode identifier.  These inode content files are
//! all located in the mountpoint directory and are deleted when
//! sn_fs is unmounted.  Thus, they are never directly visible
//! to other processes.
//!
//! In a networked implementation, the above mechanism could be used
//! as a method to implement a local cache.
//!
//! For usage/examples, see:
//!   examples/sn_fs.rs
//!
// Note: see for helpful description of inode fields and when/how to update them.
// https://man7.org/linux/man-pages/man7/inode.7.html

#![deny(missing_docs)]

mod sn_fs;
pub use self::sn_fs::SnFs;

mod fs_tree_types;
mod metadata;
