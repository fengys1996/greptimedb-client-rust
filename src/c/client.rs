use greptime_proto::v1::RowInsertRequests;
use snafu::ResultExt;

use crate::{
    c::runtime::runtime, ensure_not_null, ChannelConfig, ChannelManager, ClientBuilder,
    ClientTlsOption, Database,
};

use super::{
    error::{self, RustResult, StatusCode},
    rows::Rows,
    utils::convert_c_string,
};

pub struct Client {
    db: Database,
}

#[repr(C)]
pub struct ClientOptions {
    endpoint: *const libc::c_char,
    db_name: *const libc::c_char,
    secure: bool,
}

/// Create a new client for ingesting data into GreptimeDB.
///
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn client_new(
    opts: ClientOptions,
    client_ptr: *mut *mut Client,
) -> StatusCode {
    if let Err(_e) = do_client_new(opts, client_ptr) {
        StatusCode::Err
    } else {
        StatusCode::Success
    }
}

unsafe fn do_client_new(opts: ClientOptions, client_ptr: *mut *mut Client) -> RustResult<()> {
    let ClientOptions {
        endpoint,
        db_name,
        secure,
    } = opts;

    let greptimedb_endpoint = convert_c_string(endpoint)?;
    let greptimedb_dbname = convert_c_string(db_name)?;
    let greptimedb_secure = secure;

    let builder = ClientBuilder::default().peers(vec![&greptimedb_endpoint]);

    let grpc_client = if greptimedb_secure {
        let channel_config = ChannelConfig::default().client_tls_config(ClientTlsOption::default());

        let channel_manager =
            ChannelManager::with_tls_config(channel_config).context(error::ClientSnafu)?;
        builder.channel_manager(channel_manager).build()
    } else {
        builder.build()
    };

    let client = Database::new_with_dbname(greptimedb_dbname, grpc_client);
    *client_ptr = Box::into_raw(Box::new(Client { db: client }));

    Ok(())
}

/// Insert [`Rows`] into the GreptimeDB.
///
/// # Safety
///
/// Caller must ensure that `client_ptr` and `rows_ptr` are valid pointers.
pub unsafe extern "C" fn client_insert_rows(
    rows_ptr: *mut Rows,
    client_ptr: *mut Client,
) -> StatusCode {
    if let Err(_e) = do_client_insert_rows(rows_ptr, client_ptr) {
        StatusCode::Err
    } else {
        StatusCode::Success
    }
}

unsafe fn do_client_insert_rows(rows_ptr: *mut Rows, client_ptr: *mut Client) -> RustResult<()> {
    ensure_not_null!(client_ptr, "client_ptr");
    ensure_not_null!(rows_ptr, "rows_ptr");

    let _rows = unsafe { &*rows_ptr };
    let client = unsafe { &mut *client_ptr };

    // FIXME
    let grpc_insert_req = RowInsertRequests { inserts: vec![] };
    let r = runtime();
    let handle = r.spawn(async move {
        client
            .db
            .row_insert(grpc_insert_req)
            .await
            .context(error::ClientSnafu)
    });
    r.block_on(handle).context(error::TaskJoinSnafu)??;

    Ok(())
}

/// Release the client and free the memory.
///
/// # Safety
///
/// Caller must ensure that `client_ptr` is a valid pointer to a [`Client`].
#[no_mangle]
pub unsafe extern "C" fn client_free(client_ptr: *mut Client) {
    if client_ptr.is_null() {
        return;
    }

    let _ = Box::from_raw(client_ptr);
}
