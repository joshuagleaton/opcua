// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
#![rustfmt::skip]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    request_header::RequestHeader,
    service_types::RegisteredServer,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RegisterServerRequest {
    pub request_header: RequestHeader,
    pub server: RegisteredServer,
}

impl MessageInfo for RegisterServerRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::RegisterServerRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RegisterServerRequest> for RegisterServerRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.server.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.server.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_options)?;
        let server = RegisteredServer::decode(stream, decoding_options)?;
        Ok(RegisterServerRequest {
            request_header,
            server,
        })
    }
}
