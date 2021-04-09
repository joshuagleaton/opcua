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
    node_id::NodeId,
    service_types::RelativePath,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BrowsePath {
    pub starting_node: NodeId,
    pub relative_path: RelativePath,
}

impl MessageInfo for BrowsePath {
    fn object_id(&self) -> ObjectId {
        ObjectId::BrowsePath_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BrowsePath> for BrowsePath {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.starting_node.byte_len();
        size += self.relative_path.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.starting_node.encode(stream)?;
        size += self.relative_path.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let starting_node = NodeId::decode(stream, decoding_options)?;
        let relative_path = RelativePath::decode(stream, decoding_options)?;
        Ok(BrowsePath {
            starting_node,
            relative_path,
        })
    }
}
