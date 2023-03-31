//******************************************************************************************************
//  basic_measurement.rs - Gbtc
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
//  03/30/2023 - J. Ritchie Carroll
//       Generated original version of source code.
//
//*****************************************************************************************************

use crate::transport::{Measurement, StateFlags};
use crate::Ticks;
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter, Result as FmtResult};
use uuid::Uuid;

/// Represents a basic `Measurement` for transmission or reception in the STTP API.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BasicMeasurement {
    signal_id: Uuid,
    value: f64,
    timestamp: Ticks,
    flags: StateFlags,
}

impl Measurement for BasicMeasurement {
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

impl Display for BasicMeasurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Measurement::fmt(self, f)
    }
}
