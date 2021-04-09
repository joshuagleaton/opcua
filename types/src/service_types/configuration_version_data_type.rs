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
};

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigurationVersionDataType {
    pub major_version: u32,
    pub minor_version: u32,
}

impl MessageInfo for ConfigurationVersionDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::ConfigurationVersionDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ConfigurationVersionDataType> for ConfigurationVersionDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.major_version.byte_len();
        size += self.minor_version.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.major_version.encode(stream)?;
        size += self.minor_version.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let major_version = u32::decode(stream, decoding_options)?;
        let minor_version = u32::decode(stream, decoding_options)?;
        Ok(ConfigurationVersionDataType {
            major_version,
            minor_version,
        })
    }
}
