// Copyright (c) 2018-2019 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 or MIT license, at your option.
//
// A copy of the Apache License, Version 2.0 is included in the software as
// LICENSE-APACHE and a copy of the MIT license is included in the software
// as LICENSE-MIT. You may also obtain a copy of the Apache License, Version 2.0
// at https://www.apache.org/licenses/LICENSE-2.0 and a copy of the MIT license
// at https://opensource.org/licenses/MIT.

pub mod header;
mod io;

use bytes::Bytes;
use header::{Data, GoAway, Header, StreamId, WindowUpdate};
use std::{convert::TryInto, num::TryFromIntError};

pub use io::{FrameDecodeError, Io};

/// A Yamux message frame consisting of header and body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Frame<T> {
    header: Header<T>,
    body: Bytes,
}

impl<T> Frame<T> {
    pub fn new(header: Header<T>) -> Self {
        Frame {
            header,
            body: Bytes::new(),
        }
    }

    pub fn header(&self) -> &Header<T> {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut Header<T> {
        &mut self.header
    }

    pub(crate) fn cast<U>(self) -> Frame<U> {
        Frame {
            header: self.header.cast(),
            body: self.body,
        }
    }
}

impl Frame<Data> {
    pub fn data(id: StreamId, b: Bytes) -> Result<Self, TryFromIntError> {
        Ok(Frame {
            header: Header::data(id, b.len().try_into()?),
            body: b,
        })
    }

    pub fn body(&self) -> &Bytes {
        &self.body
    }

    pub fn body_len(&self) -> u32 {
        // Safe cast since we construct `Frame::<Data>`s only with
        // `Bytes` of length [0, u32::MAX] in `Frame::data` above.
        self.body().len() as u32
    }

    pub fn into_body(self) -> Bytes {
        self.body
    }
}

impl Frame<WindowUpdate> {
    pub fn window_update(id: StreamId, credit: u32) -> Self {
        Frame {
            header: Header::window_update(id, credit),
            body: Bytes::new(),
        }
    }
}

impl Frame<GoAway> {
    pub fn term() -> Self {
        Frame {
            header: Header::term(),
            body: Bytes::new(),
        }
    }

    pub fn protocol_error() -> Self {
        Frame {
            header: Header::protocol_error(),
            body: Bytes::new(),
        }
    }

    pub fn internal_error() -> Self {
        Frame {
            header: Header::internal_error(),
            body: Bytes::new(),
        }
    }
}
