use std::pin::Pin;

use arrow_flight::FlightData;
use futures::channel::mpsc::{self, Sender};
use futures::{Stream, StreamExt};

use crate::database::Database;
use crate::flight::do_put::DoPutResponse;
use crate::flight::FlightEncoder;
use crate::Result;

use super::{get_env_or_default, BulkWriteOptions, DEFAULT_CHANNEL_BUFFER_SIZE};

/// [`BulkStreamSession`] represents the current underlying Flight DoPut session
/// held by [`crate::BulkStreamWriter`].
///
/// It is responsible for maintaining the underlying resources and state for
/// that session.
pub(super) struct BulkStreamSession {
    pub(super) sender: Sender<FlightData>,
    pub(super) response_stream: Pin<Box<dyn Stream<Item = Result<DoPutResponse>>>>,
    pub(super) encoder: FlightEncoder,
    pub(super) schema_sent: bool,
}

impl BulkStreamSession {
    pub(super) async fn new(database: &Database, options: &BulkWriteOptions) -> Result<Self> {
        let encoder = FlightEncoder::with_compression(options.compression);
        let channel_buffer_size = get_env_or_default(
            "GREPTIMEDB_CHANNEL_BUFFER_SIZE",
            DEFAULT_CHANNEL_BUFFER_SIZE,
        );
        let (sender, receiver) = mpsc::channel(channel_buffer_size);
        let request_stream = receiver.boxed();
        let response_stream = database.do_put(request_stream).await?;

        Ok(Self {
            sender,
            response_stream,
            encoder,
            schema_sent: false,
        })
    }
}
