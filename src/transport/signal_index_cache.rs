//******************************************************************************************************
//  signal_index_cache.rs - Gbtc
//
//  Copyright © 2023, Grid Protection Alliance.  All Rights Reserved.
//
//  Licensed to the Grid Protection Alliance (GPA) under one or more contributor license agreements. See
//  the NOTICE file distributed with this work for additional information regarding copyright ownership.
//  The GPA licenses this file to you under the MIT License (MIT), the "License"; you may not use this
//  file except in compliance with the License. You may obtain a copy of the License at:
//
//      http://opensource.org/licenses/MIT
//
//  Unless agreed to in writing, the subject software distributed under the License is distributed on an
//  "AS-IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. Refer to the
//  License for the specific language governing permissions and limitations.
//
//  Code Modification History:
//  ----------------------------------------------------------------------------------------------------
//  03/26/2023 - J. Ritchie Carroll
//       Generated original version of source code.
//
//******************************************************************************************************

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::str;
use uuid::Uuid;

/// Represents a mapping of 32-bit runtime IDs to 128-bit globally unique measurement IDs. The class
/// additionally provides reverse lookup and an extra mapping to human-readable measurement keys.
#[derive(Default, Debug, Clone)]
pub struct SignalIndexCache {
    reference: HashMap<i32, u32>,
    signal_id_list: Vec<Uuid>,
    source_list: Vec<String>,
    id_list: Vec<u64>,
    signal_id_cache: HashMap<Uuid, i32>,
    // binary_length: u32,
    // tssc_decoder: Option<Decoder>,
}

impl SignalIndexCache {
    /// Creates a new, empty `SignalIndexCache`.
    pub fn new() -> SignalIndexCache {
        SignalIndexCache {
            reference: HashMap::new(),
            signal_id_list: Vec::new(),
            source_list: Vec::new(),
            id_list: Vec::new(),
            signal_id_cache: HashMap::new(),
            // binary_length: 0,
            // tssc_decoder: None,
        }
    }

    fn add_record(
        &mut self,
        //ds: &Arc<datasubscriber::DataSubscriber>,
        signal_index: i32,
        signal_id: Uuid,
        source: String,
        id: u64,
        // char_size_estimate: u32,
    ) {
        let index = self.signal_id_list.len() as u32;
        self.reference.insert(signal_index, index);
        self.signal_id_list.push(signal_id);
        self.source_list.push(source.clone());
        self.id_list.push(id);
        self.signal_id_cache.insert(signal_id, signal_index);

        // Lookup measurement metadata, registering it if not defined already
        // Replace with actual metadata lookup function from DataSubscriber
        // let metadata = ds.lookup_metadata(signal_id, source, id);

        // Char size here helps provide a rough-estimate on binary length used to reserve
        // bytes for a vector, if exact size is needed call recalculate_binary_length first
        // self.binary_length += 32 + (source.len() as u32) * char_size_estimate;
    }

    /// Determines if the specified signalindex exists with the `SignalIndexCache`.
    pub fn contains(&self, signal_index: i32) -> bool {
        self.reference.contains_key(&signal_index)
    }

    /// Returns the signal ID Guid for the specified signalindex in the `SignalIndexCache`.
    pub fn signal_id(&self, signal_index: i32) -> Uuid {
        if let Some(index) = self.reference.get(&signal_index) {
            self.signal_id_list[*index as usize]
        } else {
            Uuid::nil()
        }
    }

    /// Gets a set for all the Guid values found in the `SignalIndexCache`.
    pub fn signal_ids(&self) -> HashSet<Uuid> {
        self.signal_id_list.iter().cloned().collect()
    }

    /// Returns the `Measurement` source string for the specified signalindex in the `SignalIndexCache`.
    pub fn source(&self, signal_index: i32) -> &str {
        if let Some(index) = self.reference.get(&signal_index) {
            &self.source_list[*index as usize]
        } else {
            ""
        }
    }

    /// Returns the `Measurement` integer ID for the specified signalindex in the `SignalIndexCache`.
    pub fn id(&self, signal_index: i32) -> u64 {
        if let Some(index) = self.reference.get(&signal_index) {
            self.id_list[*index as usize]
        } else {
            u64::MAX
        }
    }

    /// Record returns the key `Measurement` values, signal ID Guid, source string, and integer ID and a
    /// final boolean value representing find success for the specified signalindex in the `SignalIndexCache`.
    pub fn record(&self, signal_index: i32) -> (Uuid, &str, u64, bool) {
        if let Some(index) = self.reference.get(&signal_index) {
            (
                self.signal_id_list[*index as usize],
                &self.source_list[*index as usize],
                self.id_list[*index as usize],
                true,
            )
        } else {
            (Uuid::nil(), "", 0, false)
        }
    }

    /// Returns the signal index for the specified signal ID Guid in the `SignalIndexCache`.
    pub fn signal_index(&self, signal_id: Uuid) -> i32 {
        if let Some(signal_index) = self.signal_id_cache.get(&signal_id) {
            *signal_index
        } else {
            -1
        }
    }

    /// Gets the number of `Measurement` records that can be found in the `SignalIndexCache`.
    pub fn count(&self) -> u32 {
        self.signal_id_cache.len() as u32
    }

    /// Parses a `SignalIndexCache` from the specified byte buffer received from a `DataPublisher`.
    pub fn decode(&mut self, buffer: &[u8]) -> Result<Uuid, Box<dyn Error>> {
        let length = buffer.len();

        if length < 4 {
            return Err("not enough buffer provided to parse".into());
        }

        let binary_length = u32::from_be_bytes(buffer.try_into().unwrap());
        let mut offset = 4;

        if (length as u32) < binary_length {
            return Err("not enough buffer provided to parse".into());
        }

        let subscriber_id = Uuid::from_slice(&buffer[offset..offset + 16])?;
        offset += 16;

        let reference_count = u32::from_be_bytes(buffer[offset..].try_into().unwrap());
        offset += 4;

        for _ in 0..reference_count {
            let signal_index = i32::from_be_bytes(buffer[offset..].try_into().unwrap());
            offset += 4;

            let signal_id = Uuid::from_slice(&buffer[offset..offset + 16])?;
            offset += 16;

            let source_size = u32::from_be_bytes(buffer[offset..].try_into().unwrap()) as usize;
            offset += 4;

            let source = str::from_utf8(&buffer[offset..offset + source_size])?;
            offset += source_size;

            let key_id = u64::from_be_bytes(buffer[offset..].try_into().unwrap());
            offset += 8;

            self.add_record(signal_index, signal_id, source.to_string(), key_id);
        }

        Ok(subscriber_id)
    }
}
