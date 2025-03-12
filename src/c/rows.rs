use std::slice::from_raw_parts;

use greptime_proto::v1::{
    ColumnSchema as ProtoColumnSchema, Row as ProtoRow, RowInsertRequest, Rows as ProtoRows,
};

use crate::{c::utils::convert_c_string, ensure_not_null};

use super::{
    error::{RustResult, StatusCode},
    schema::ColumnSchema,
    value::Value,
};

/// The [`Rows`] struct contains the data to be inserted.
pub struct Rows {
    table_name: String,
    column_schemas: Vec<ProtoColumnSchema>,
    rows: Vec<ProtoRow>,
}

impl From<Rows> for RowInsertRequest {
    fn from(rows: Rows) -> Self {
        let Rows {
            table_name,
            column_schemas,
            rows,
        } = rows;

        let rows = ProtoRows {
            schema: column_schemas,
            rows,
        };

        RowInsertRequest {
            table_name,
            rows: Some(rows),
        }
    }
}

/// Create a new [`Rows`] with pre-allocate, and assign it to `rows_ptr`.
///
/// # Safety
///
/// The rows_ptr must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn rows_new_with_capacity(
    table_name: *const libc::c_char,
    capacity: usize,
    column_schema_ptr: *const ColumnSchema,
    len: usize,
    rows_ptr: *mut *mut Rows,
) -> StatusCode {
    if let Err(_e) =
        do_rows_new_with_capacity(table_name, capacity, column_schema_ptr, len, rows_ptr)
    {
        StatusCode::Err
    } else {
        StatusCode::Success
    }
}

unsafe fn do_rows_new_with_capacity(
    table_name: *const libc::c_char,
    capacity: usize,
    column_schema_ptr: *const ColumnSchema,
    len: usize,
    rows_ptr: *mut *mut Rows,
) -> RustResult<()> {
    ensure_not_null!(rows_ptr, "rows_ptr");
    ensure_not_null!(column_schema_ptr, "column_schema_ptr");

    let table_name = convert_c_string(table_name)?;

    let column_schemas = from_raw_parts(column_schema_ptr, len)
        .iter()
        .map(|column_schema| column_schema.try_into())
        .collect::<RustResult<_>>()?;

    let rows = Rows {
        table_name,
        rows: Vec::with_capacity(capacity),
        column_schemas,
    };
    *rows_ptr = Box::into_raw(Box::new(rows));

    Ok(())
}

/// Push a row to the [`Rows`].
///
/// # Note
///
/// There is no schema validation here. The caller must ensure that the
/// `value_ptr` and `len` match the schema.
///
/// # Safety
///
/// The `value_ptr` and `len` must identify a valid [`Value`] array. And
/// `rows_ptr` must be a valid pointer to a [`Rows`].
pub unsafe extern "C" fn rows_push_row(
    value_ptr: *mut Value,
    len: usize,
    rows_ptr: *mut Rows,
) -> StatusCode {
    if value_ptr.is_null() || rows_ptr.is_null() {
        return StatusCode::Err;
    }

    let rows = &mut *rows_ptr;

    let values = from_raw_parts(value_ptr, len)
        .iter()
        .map(|val| (*val).into())
        .collect();
    rows.rows.push(ProtoRow { values });

    StatusCode::Success
}

/// Release the [`Rows`] and free the memory.
///
/// # Safety
///
/// The `rows_ptr` must be a valid pointer to a [`Rows`].
pub unsafe extern "C" fn rows_free(rows_ptr: *mut Rows) -> StatusCode {
    if rows_ptr.is_null() {
        return StatusCode::Err;
    }

    let _ = Box::from_raw(rows_ptr);

    StatusCode::Success
}
