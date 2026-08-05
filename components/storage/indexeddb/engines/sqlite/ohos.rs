/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::ffi::CString;

use malloc_size_of::MallocSizeOf;
use ohos_rdb_sys::cursor::OH_Cursor;
use ohos_rdb_sys::data_values::{OH_Values_Create, OH_Values_Destroy};
use ohos_rdb_sys::relational_store::{
    OH_Rdb_CreateConfig, OH_Rdb_CreateOrOpen, OH_Rdb_DestroyConfig, OH_Rdb_ExecuteQueryV2,
    OH_Rdb_Store,
};
use rusqlite::ToSql;

use crate::indexeddb::engines::sqlite::database_model::Model;
use crate::{ConnectionTrait, RowTrait, StatementTrait};

pub struct OhosConnection(*mut OH_Rdb_Store);

impl MallocSizeOf for OhosConnection {
    fn size_of(&self, ops: &mut malloc_size_of::MallocSizeOfOps) -> usize {
        0
    }
}

pub(crate) struct OhosRow(*mut OH_Cursor);

impl RowTrait for OhosRow {
    fn get<T>(&self, index: usize) -> rusqlite::Result<T> {
        todo!()
    }
}

pub(crate) struct OhosStatement {}

impl StatementTrait for OhosStatement {
    type RowType<'a> = OhosRow;

    fn query_one<'a, T, F: FnOnce(&Self::RowType<'a>) -> rusqlite::Result<T>>(
        &self,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T> {
        todo!()
    }

    fn query_and_then<'a, T, F: FnOnce(&Self::RowType<'a>) -> rusqlite::Result<T>>(
        &self,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T> {
        todo!()
    }

    fn query_row<'a, T, F: FnOnce(&Self::RowType<'a>) -> rusqlite::Result<T>>(
        &self,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T> {
        todo!()
    }

    fn execute<P: rusqlite::Params>(&self, params: P) -> rusqlite::Result<usize> {
        todo!()
    }

    fn query_map<'a, T, F: Fn(&Self::RowType<'a>) -> rusqlite::Result<T>>(
        &self,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<Vec<T>> {
        todo!()
    }

    fn exists<P: rusqlite::Params>(&self, params: P) -> rusqlite::Result<bool> {
        todo!()
    }
}

impl ConnectionTrait for OhosConnection {
    type RowType<'a> = OhosRow;
    type Statement<'a> = OhosStatement;
    fn open<P: AsRef<std::path::Path>>(p: P) -> rusqlite::Result<Self> {
        unsafe {
            let config = OH_Rdb_CreateConfig();
            let mut error_code = 0;
            let store = OH_Rdb_CreateOrOpen(config, &mut error_code);
            OH_Rdb_DestroyConfig(config);
            Ok(OhosConnection(store))
        }
    }

    fn table_exists<N: rusqlite::Name>(
        &self,
        db_name: Option<N>,
        table_name: N,
    ) -> rusqlite::Result<bool> {
        Ok(false)
    }

    fn query_row<'a, T, F: FnOnce(&Self::RowType<'a>) -> rusqlite::Result<T>>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
        f: F,
    ) -> rusqlite::Result<T> {
        unsafe {
            let mut values = OH_Values_Create();
            for i in params {
                i.to_sql().unwrap();
                let string = i.to_sql().unwrap();
                /*
                match string {
                    rusqlite::types::ToSqlOutput::Borrowed(value_ref) => todo!(),
                    rusqlite::types::ToSqlOutput::Owned(value) => match value {
                        rusqlite::types::Value::Null => todo!(),
                        rusqlite::types::Value::Integer(_) => todo!(),
                        rusqlite::types::Value::Real(_) => todo!(),
                        rusqlite::types::Value::Text(_) => todo!(),
                        rusqlite::types::Value::Blob(items) => todo!(),
                    },
                    _ => todo!(),
                }
                let cstring = CString::new(string).unwrap();
                OH_Values_PutText(values, cstring.as_ptr());
                 */
            }

            let sql_query = CString::new(sql).unwrap();
            let cursor = OH_Rdb_ExecuteQueryV2(self.0, sql_query.as_ptr(), values);

            let mut count = 0;
            (*cursor).getRowCount.unwrap()(cursor, &mut count);
            log::error!("We found this many rows {:?}", count);
            let result = f(&OhosRow(cursor));
            //(*cursor).destroy
            OH_Values_Destroy(values);

            result
        }
    }

    fn prepare<'b>(&self, sql: &'b str) -> rusqlite::Result<Self::Statement<'b>> {
        todo!()
    }

    fn execute<P: rusqlite::Params>(&self, sql: &str, params: P) -> rusqlite::Result<usize> {
        todo!()
    }
}
