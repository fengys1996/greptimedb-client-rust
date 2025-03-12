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

/// The [`RowBatch`] struct contains the data to be inserted.
pub struct RowBatch {
    pub(crate) inner: Option<Inner>,
}

pub(crate) struct Inner {
    pub(crate) table_name: String,
    pub(crate) column_schemas: Vec<ProtoColumnSchema>,
    pub(crate) rows: Vec<ProtoRow>,
}

impl From<RowBatch> for RowInsertRequest {
    fn from(row_batch: RowBatch) -> Self {
        let Inner {
            table_name,
            column_schemas,
            rows,
        } = row_batch.inner.unwrap();

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

/// Create a new [`RowBatch`] with pre-allocate, and assign it to `row_batch_ptr`.
///
/// # Safety
///
/// The row_batch_ptr must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn row_batch_new_with_capacity(
    table_name: *const libc::c_char,
    capacity: usize,
    column_schema_ptr: *const ColumnSchema,
    len: usize,
    row_batch_ptr: *mut *mut RowBatch,
) -> StatusCode {
    if let Err(_e) =
        do_row_batch_new_with_capacity(table_name, capacity, column_schema_ptr, len, row_batch_ptr)
    {
        StatusCode::Err
    } else {
        StatusCode::Success
    }
}

unsafe fn do_row_batch_new_with_capacity(
    table_name: *const libc::c_char,
    capacity: usize,
    column_schema_ptr: *const ColumnSchema,
    len: usize,
    row_batch_ptr: *mut *mut RowBatch,
) -> RustResult<()> {
    ensure_not_null!(row_batch_ptr, "row_batch_ptr");
    ensure_not_null!(column_schema_ptr, "column_schema_ptr");

    let table_name = convert_c_string(table_name)?;

    let column_schemas = from_raw_parts(column_schema_ptr, len)
        .iter()
        .map(|column_schema| column_schema.try_into())
        .collect::<RustResult<_>>()?;

    let row_batch = RowBatch {
        inner: Some(Inner {
            table_name,
            rows: Vec::with_capacity(capacity),
            column_schemas,
        }),
    };
    *row_batch_ptr = Box::into_raw(Box::new(row_batch));

    Ok(())
}

/// Push a row to the [`RowBatch`].
///
/// # Note
///
/// There is no schema validation here. The caller must ensure that the
/// `value_ptr` and `len` match the schema.
///
/// # Safety
///
/// The `value_ptr` and `len` must identify a valid [`Value`] array. And
/// `row_batch_ptr` must be a valid pointer to a [`RowBatch`].
#[no_mangle]
pub unsafe extern "C" fn row_batch_push_row(
    value_ptr: *mut Value,
    len: usize,
    row_batch_ptr: *mut RowBatch,
) -> StatusCode {
    if value_ptr.is_null() || row_batch_ptr.is_null() {
        return StatusCode::Err;
    }

    let row_batch = &mut *row_batch_ptr;

    let Ok(values) = from_raw_parts(value_ptr, len)
        .iter()
        .map(|val| (*val).try_into())
        .collect::<RustResult<_>>()
    else {
        return StatusCode::Err;
    };

    row_batch
        .inner
        .as_mut()
        .unwrap()
        .rows
        .push(ProtoRow { values });

    StatusCode::Success
}

/// Release the [`RowBatch`] and free the memory.
///
/// # Safety
///
/// The `row_batch_ptr` must be a valid pointer to a [`RowBatch`].
#[no_mangle]
pub unsafe extern "C" fn row_batch_free(row_batch_ptr: *mut RowBatch) -> StatusCode {
    if row_batch_ptr.is_null() {
        return StatusCode::Err;
    }

    let _ = Box::from_raw(row_batch_ptr);

    StatusCode::Success
}
