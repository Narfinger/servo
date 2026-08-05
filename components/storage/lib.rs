/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod cache_storage;
pub mod client_storage;
mod indexeddb;
pub(crate) mod shared;
mod storage_thread;
mod webstorage;

use std::path::{Path, PathBuf};

pub use cache_storage::CacheStorageThreadFactory;
pub use client_storage::ClientStorageThreadFactory;
pub(crate) use indexeddb::IndexedDBThreadFactory;
use malloc_size_of::MallocSizeOf;
use rusqlite::{Params, Result, Row, Statement};
pub use storage_thread::new_storage_threads;
pub(crate) use webstorage::WebStorageThreadFactory;

trait ConnectionTrait: Sized {
    fn open<P: AsRef<Path>>(p: P) -> Result<Self>;
    fn table_exists<N: rusqlite::Name>(&self, db_name: Option<N>, table_name: N) -> Result<bool>;
    fn query_row<T, P: Params, F: FnOnce(&Row<'_>) -> Result<T>>(
        &self,
        sql: &str,
        params: P,
        f: F,
    ) -> rusqlite::Result<T>;
    fn prepare<'a>(&self, sql: &'a str) -> Result<rusqlite::Statement<'a>>;
    fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize>;
}

impl ConnectionTrait for rusqlite::Connection {
    fn open<P: AsRef<Path>>(p: P) -> Result<Self> {
        todo!()
    }

    fn table_exists<N: rusqlite::Name>(&self, db_name: Option<N>, table_name: N) -> Result<bool> {
        todo!()
    }

    fn query_row<T, P: Params, F: FnOnce(&Row<'_>) -> Result<T>>(
        &self,
        sql: &str,
        params: P,
        f: F,
    ) -> rusqlite::Result<T> {
        todo!()
    }

    fn prepare<'a>(&self, sql: &'a str) -> Result<rusqlite::Statement<'a>> {
        todo!()
    }

    fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize> {
        todo!()
    }
}
