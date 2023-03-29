//******************************************************************************************************
//  measurement.rs - Gbtc
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

use crate::{transport::StateFlags, Ticks};
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter, Result as FmtResult};
use uuid::Uuid;

/// Represents a basic unit of measured data for transmission or reception in the STTP API.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Measurement {
    /// Defines measurement's globally unique identifier.
    pub signal_id: Uuid,

    /// Defines instantaneous value of the measurement.
    pub value: f64,

    /// Defines the STTP uint64 timestamp, in ticks, that measurement was taken.
    pub timestamp: Ticks,

    /// Defines flags indicating the state of the measurement as reported by the device that took it.
    pub flags: StateFlags,
}

impl Measurement {
    /// Gets the integer-based time from a `Measurement` ticks-based timestamp, i.e.,
    /// the 62-bit time value excluding any leap-second flags.
    pub fn timestamp_value(&self) -> u64 {
        self.timestamp.timestamp_value()
    }

    /// Gets `Measurement` ticks-based timestamp as a standard Rust `DateTime` value.
    pub fn datetime(&self) -> DateTime<Utc> {
        self.timestamp.to_datetime()
    }
}

impl Display for Measurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} @ {} = {:.3} ({:?})",
            self.signal_id,
            self.timestamp.to_short_string(),
            self.value,
            self.flags
        )
    }
}
