/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod cache_storage;
pub mod client_storage;
mod indexeddb;
pub(crate) mod shared;
mod storage_thread;
mod webstorage;

use std::path::Path;

pub use cache_storage::CacheStorageThreadFactory;
pub use client_storage::ClientStorageThreadFactory;
pub(crate) use indexeddb::IndexedDBThreadFactory;
use rusqlite::{Params, Result, ToSql};
pub use storage_thread::new_storage_threads;
pub(crate) use webstorage::WebStorageThreadFactory;

trait RowTrait {
    fn get<T>(&self, index: usize) -> rusqlite::Result<T>;
}

trait StatementTrait {
    type RowType<'a>: RowTrait;
    fn query_one<'a, T, F: FnOnce(&Self::RowType<'a>) -> Result<T>>(
        &self,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T>;
    fn query_and_then<'a, T, F: FnOnce(&Self::RowType<'a>) -> Result<T>>(
        &self,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T>;
    fn query_row<'a, T, F: FnOnce(&Self::RowType<'a>) -> Result<T>>(
        &self,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T>;
    fn execute<P: Params>(&self, params: P) -> Result<usize>;
    fn exists<P: Params>(&self, params: P) -> Result<bool>;
}

trait ConnectionTrait: Sized {
    type RowType<'a>: RowTrait;
    type Statement<'a>: StatementTrait<RowType<'a> = Self::RowType<'a>>;
    fn open<P: AsRef<Path>>(p: P) -> Result<Self>;
    fn table_exists<N: rusqlite::Name>(&self, db_name: Option<N>, table_name: N) -> Result<bool>;
    fn query_row<'a, T, F: FnOnce(&Self::RowType<'a>) -> Result<T>>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T>;
    fn prepare<'b>(&self, sql: &'b str) -> Result<Self::Statement<'b>>;
    fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize>;
}

impl<'a> RowTrait for rusqlite::Row<'a> {
    fn get<T>(&self, index: usize) -> rusqlite::Result<T> {
        todo!()
    }
}

impl StatementTrait for rusqlite::Statement<'a> {
    type RowType<'a> = rusqlite::Row<'a>;

    fn query_one<'a, T, F: FnOnce(&Self::RowType<'a>) -> Result<T>>(
        &self,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T> {
        todo!()
    }

    fn query_and_then<'a, T, F: FnOnce(&Self::RowType<'a>) -> Result<T>>(
        &self,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T> {
        todo!()
    }

    fn query_row<'a, T, F: FnOnce(&Self::RowType<'a>) -> Result<T>>(
        &self,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T> {
        todo!()
    }

    fn execute<P: Params>(&self, params: P) -> Result<usize> {
        todo!()
    }

    fn exists<P: Params>(&self, params: P) -> Result<bool> {
        todo!()
    }
}

impl ConnectionTrait for rusqlite::Connection {
    type RowType<'a> = rusqlite::Row<'a>;
    type Statement<'a> = rusqlite::Statement<'a>;
    fn open<P: AsRef<Path>>(p: P) -> Result<Self> {
        todo!()
    }

    fn table_exists<N: rusqlite::Name>(&self, db_name: Option<N>, table_name: N) -> Result<bool> {
        todo!()
    }

    fn query_row<'a, T, F: FnOnce(&Self::RowType<'a>) -> Result<T>>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T> {
        todo!()
    }

    fn prepare<'b>(&self, sql: &'b str) -> Result<Self::Statement<'b>> {
        todo!()
    }

    fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize> {
        todo!()
    }
}
