//******************************************************************************************************
//  compact_measurement.rs - Gbtc
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
//  03/29/2023 - J. Ritchie Carroll
//       Generated original version of source code.
//
//*****************************************************************************************************

use crate::transport::{Measurement, SignalIndexCache, StateFlags};
use crate::Ticks;
use bitflags::bitflags;
use chrono::{DateTime, Utc};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::Arc;
use uuid::Uuid;

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct CompactStateFlags: u8 {
        const DATA_RANGE = 0b0000_0001;
        const DATA_QUALITY = 0b0000_0010;
        const TIME_QUALITY = 0b0000_0100;
        const SYSTEM_ISSUE = 0b0000_1000;
        const CALCULATED_VALUE = 0b0001_0000;
        const DISCARDED_VALUE = 0b0010_0000;
        const BASE_TIME_OFFSET = 0b0100_0000;
        const TIME_INDEX = 0b1000_0000;
        const NO_FLAGS = 0b0000_0000;
    }
}

const DATA_RANGE_MASK: StateFlags = StateFlags::OVER_RANGE_ERROR.union(
    StateFlags::UNDER_RANGE_ERROR.union(StateFlags::ALARM_HIGH.union(
        StateFlags::ALARM_LOW.union(StateFlags::WARNING_HIGH.union(StateFlags::WARNING_LOW)),
    )),
);

const DATA_QUALITY_MASK: StateFlags =
    StateFlags::BAD_DATA.union(
        StateFlags::SUSPECT_DATA.union(StateFlags::FLATLINE_ALARM.union(
            StateFlags::COMPARISON_ALARM.union(StateFlags::ROC_ALARM.union(
                StateFlags::RECEIVED_AS_BAD.union(StateFlags::CALCULATION_ERROR.union(
                    StateFlags::CALCULATION_WARNING.union(StateFlags::RESERVED_QUALITY_FLAG),
                )),
            )),
        )),
    );

const TIME_QUALITY_MASK: StateFlags = StateFlags::BAD_TIME.union(
    StateFlags::SUSPECT_TIME.union(
        StateFlags::LATE_TIME_ALARM.union(
            StateFlags::FUTURE_TIME_ALARM.union(
                StateFlags::UP_SAMPLED
                    .union(StateFlags::DOWN_SAMPLED.union(StateFlags::RESERVED_TIME_FLAG)),
            ),
        ),
    ),
);

const SYSTEM_ISSUE_MASK: StateFlags =
    StateFlags::SYSTEM_ERROR.union(StateFlags::SYSTEM_WARNING.union(StateFlags::MEASUREMENT_ERROR));

const CALCULATED_VALUE_MASK: StateFlags = StateFlags::CALCULATED_VALUE;

const DISCARDED_VALUE_MASK: StateFlags = StateFlags::DISCARDED_VALUE;

impl StateFlags {
    fn map_to_compact_flags(&self) -> CompactStateFlags {
        let mut mapped_state_flags = CompactStateFlags::NO_FLAGS;

        if (*self & DATA_RANGE_MASK).bits() > 0 {
            mapped_state_flags |= CompactStateFlags::DATA_RANGE;
        }

        if (*self & DATA_QUALITY_MASK).bits() > 0 {
            mapped_state_flags |= CompactStateFlags::DATA_QUALITY;
        }

        if (*self & TIME_QUALITY_MASK).bits() > 0 {
            mapped_state_flags |= CompactStateFlags::TIME_QUALITY;
        }

        if (*self & SYSTEM_ISSUE_MASK).bits() > 0 {
            mapped_state_flags |= CompactStateFlags::SYSTEM_ISSUE;
        }

        if (*self & CALCULATED_VALUE_MASK).bits() > 0 {
            mapped_state_flags |= CompactStateFlags::CALCULATED_VALUE;
        }

        if (*self & DISCARDED_VALUE_MASK).bits() > 0 {
            mapped_state_flags |= CompactStateFlags::DISCARDED_VALUE;
        }

        mapped_state_flags
    }
}

impl CompactStateFlags {
    fn map_to_full_flags(&self) -> StateFlags {
        let mut mapped_state_flags = StateFlags::NORMAL;

        if (*self & CompactStateFlags::DATA_RANGE).bits() > 0 {
            mapped_state_flags |= DATA_RANGE_MASK;
        }

        if (*self & CompactStateFlags::DATA_QUALITY).bits() > 0 {
            mapped_state_flags |= DATA_QUALITY_MASK;
        }

        if (*self & CompactStateFlags::TIME_QUALITY).bits() > 0 {
            mapped_state_flags |= TIME_QUALITY_MASK;
        }

        if (*self & CompactStateFlags::SYSTEM_ISSUE).bits() > 0 {
            mapped_state_flags |= SYSTEM_ISSUE_MASK;
        }

        if (*self & CompactStateFlags::CALCULATED_VALUE).bits() > 0 {
            mapped_state_flags |= CALCULATED_VALUE_MASK;
        }

        if (*self & CompactStateFlags::DISCARDED_VALUE).bits() > 0 {
            mapped_state_flags |= DISCARDED_VALUE_MASK;
        }

        mapped_state_flags
    }
}

const FIXED_LENGTH: usize = 9;

/// Represents a compact `Measurement` for transmission or reception in the STTP API.
#[derive(Debug, Clone)]
pub struct CompactMeasurement {
    signal_id: Uuid,
    value: f64,
    timestamp: Ticks,
    flags: StateFlags,
    signal_index_cache: Arc<SignalIndexCache>,
    include_time: bool,
    base_time_offsets: [u64; 2],
    time_index: i32,
    use_millisecond_resolution: bool,
    using_base_time_offset: bool,
}

impl CompactMeasurement {
    /// Creates a new `CompactMeasurement`.
    pub fn new(
        signal_index_cache: Arc<SignalIndexCache>,
        include_time: bool,
        use_millisecond_resolution: bool,
    ) -> Self {
        Self {
            signal_id: Uuid::nil(),
            value: 0.0,
            timestamp: Ticks::default(),
            flags: StateFlags::NORMAL,
            signal_index_cache: signal_index_cache,
            include_time: include_time,
            base_time_offsets: [0, 0],
            time_index: 0,
            use_millisecond_resolution: use_millisecond_resolution,
            using_base_time_offset: false,
        }
    }

    /// Gets flag that determines if time is serialized into measurement binary image.
    pub fn include_time(&self) -> bool {
        self.include_time
    }

    /// Gets the length of the `CompactMeasurement`.
    pub fn get_binary_length(&mut self) -> usize {
        let length = FIXED_LENGTH;

        if !self.include_time {
            return length;
        }

        let base_time_offset = self.base_time_offsets[self.time_index as usize];

        if base_time_offset > 0 {
            let difference = self.timestamp_value() - base_time_offset;

            if difference > 0 {
                if self.use_millisecond_resolution {
                    self.using_base_time_offset =
                        (difference / Ticks::PER_MILLISECOND) < (u16::MAX as u64);
                } else {
                    self.using_base_time_offset = difference < (u32::MAX as u64);
                }
            } else {
                self.using_base_time_offset = false;
            }

            if self.using_base_time_offset {
                if self.use_millisecond_resolution {
                    length + 2
                } else {
                    length + 4
                }
            } else {
                length + 8
            }
        } else {
            length + 8
        }
    }

    /// Gets offset compressed millisecond-resolution 2-byte timestamp.
    pub fn get_timestamp_c2(&self) -> u16 {
        ((self.timestamp_value() - self.base_time_offsets[self.time_index as usize])
            / Ticks::PER_MILLISECOND) as u16
    }

    /// Gets offset compressed tick-resolution 4-byte timestamp.
    pub fn get_timestamp_c4(&self) -> u32 {
        (self.timestamp_value() - self.base_time_offsets[self.time_index as usize]) as u32
    }

    /// Gets byte level compact state flags with encoded time index and base time offset bits.
    pub fn get_compact_state_flags(&self) -> u8 {
        // Encode compact state flags
        let mut flags = self.flags.map_to_compact_flags();

        if self.time_index != 0 {
            flags |= CompactStateFlags::TIME_INDEX;
        }

        if self.using_base_time_offset {
            flags |= CompactStateFlags::BASE_TIME_OFFSET;
        }

        flags.bits()
    }

    /// Sets byte level compact state flags with encoded time index and base time offset bits.
    pub fn set_compact_state_flags(&mut self, value: u8) {
        // Decode compact state flags
        let flags = CompactStateFlags::from_bits(value).unwrap();

        self.flags = flags.map_to_full_flags();

        if flags.contains(CompactStateFlags::TIME_INDEX) {
            self.time_index = 1;
        } else {
            self.time_index = 0;
        }

        self.using_base_time_offset = flags.contains(CompactStateFlags::BASE_TIME_OFFSET);
    }

    /// Gets the 4-byte run-time signal index for this measurement.
    pub fn get_runtime_id(&self) -> i32 {
        self.signal_index_cache.signal_index(self.signal_id)
    }

    /// Sets the 4-byte run-time signal index for this measurement.
    pub fn set_runtime_id(&mut self, signal_index: i32) {
        self.signal_id = self.signal_index_cache.signal_id(signal_index);
    }

    /// parses a CompactMeasurement from the specified byte buffer.
    pub fn decode(&mut self, buffer: &[u8]) -> Result<usize, Box<dyn Error>> {
        if buffer.len() < FIXED_LENGTH {
            return Err("Not enough buffer available to deserialize compact measurement".into());
        }

        // Basic Compact Measurement Format:
        //      Field:     Bytes:
        //      --------   -------
        //       Flags        1
        //        ID          4
        //       Value        4
        //       [Time]    0/2/4/8
        let mut index = 0;

        // Decode state flags
        self.set_compact_state_flags(buffer[0]);
        index += 1;

        // Decode runtime ID
        let bytes: [u8; 4] = buffer[index..].try_into()?;
        self.set_runtime_id(i32::from_be_bytes(bytes));
        index += 4;

        // Decode value
        let bytes: [u8; 4] = buffer[index..].try_into()?;
        self.value = f32::from_be_bytes(bytes) as f64;
        index += 4;

        if !self.include_time {
            return Ok(index);
        }

        if self.using_base_time_offset {
            let base_time_offset = self.base_time_offsets[self.time_index as usize];

            if self.use_millisecond_resolution {
                // Decode 2-byte millisecond offset timestamp
                if base_time_offset > 0 {
                    let bytes: [u8; 2] = buffer[index..].try_into()?;
                    self.timestamp = Ticks::new(
                        base_time_offset
                            + (u64::from(u16::from_be_bytes(bytes)) * Ticks::PER_MILLISECOND),
                    );
                }
                index += 2;
            } else {
                // Decode 4-byte tick offset timestamp
                if base_time_offset > 0 {
                    let bytes: [u8; 4] = buffer[index..].try_into()?;
                    self.timestamp =
                        Ticks::new(base_time_offset + u64::from(u32::from_be_bytes(bytes)));
                }
                index += 4;
            }
        } else {
            // Decode 8-byte full fidelity timestamp
            // Note that only a full fidelity timestamp can carry leap second flags
            let bytes: [u8; 8] = buffer[index..].try_into()?;
            self.timestamp = Ticks::new(u64::from_be_bytes(bytes));
            index += 8;
        }

        Ok(index)
    }
}

impl Measurement for CompactMeasurement {
    fn signal_id(&self) -> Uuid {
        self.signal_id
    }

    fn set_signal_id(&mut self, signal_id: Uuid) {
        self.signal_id = signal_id;
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    fn timestamp(&self) -> Ticks {
        self.timestamp
    }

    fn set_timestamp(&mut self, timestamp: Ticks) {
        self.timestamp = timestamp;
    }

    fn flags(&self) -> StateFlags {
        self.flags
    }

    fn set_flags(&mut self, flags: StateFlags) {
        self.flags = flags;
    }

    fn timestamp_value(&self) -> u64 {
        self.timestamp.timestamp_value()
    }

    fn datetime(&self) -> DateTime<Utc> {
        self.timestamp.to_datetime()
    }
}

impl Display for CompactMeasurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Measurement::fmt(self, f)
    }
}
