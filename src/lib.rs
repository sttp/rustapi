//******************************************************************************************************
//  lib.rs - Gbtc
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
//  03/25/2023 - J. Ritchie Carroll
//       Generated original version of source code.
//
//******************************************************************************************************

//! <img align="right" src="https://raw.githubusercontent.com/sttp/rustapi/main/assets/sttp.png">
//!
//! Rust API for the Streaming Telemetry Transport Protocol ([STTP](https://github.com/sttp) / [IEEE 2664](https://standards.ieee.org/ieee/2664/7397/))
//!
//! View on GitHub: [sttp/rustapi](https://github.com/sttp/rustapi)
//!
//! STTP is optimized for the demands of transporting high volume streaming data.
//! The protocol allows for the transmission of any information that can be
//! represented longitudinally, e.g., time-series data.

// Expose local modules as public passthroughs of STTP module
mod config;
pub use crate::config::Config;

mod settings;
pub use crate::settings::Settings;

mod ticks;
pub use crate::ticks::Ticks;

pub mod data;

pub mod transport;

// pub fn test() -> bool {
//     let mut c: Config = Config::default();

//     c.metadata_filters = "".to_string();

//     return true;
// }

use std::sync::Arc;
use std::time::SystemTime;

/// Represents an STTP data subscriber.
///
/// The `Subscriber` exists as a simplified implementation of the `DataSubscriber` found
/// in the transport module. The `Subscriber` is intended to simplify common uses of
/// STTP data reception and maintains an internal instance of the `DataSubscriber` for
/// subscription based functionality.
pub struct Subscriber {
    // Configuration reference
    config: Arc<Config>,

    // DataSubscriber reference
    //ds: Arc<transport::DataSubscriber>,

    // Callback references
    status_message_logger: Option<Box<dyn Fn(String) + Send + Sync>>,
    error_message_logger: Option<Box<dyn Fn(String) + Send + Sync>>,
    //metadata_receiver: Option<Box<dyn Fn(data::DataSet) + Send + Sync>>,
    data_start_time_receiver: Option<Box<dyn Fn(SystemTime) + Send + Sync>>,
    configuration_changed_receiver: Option<Box<dyn Fn() + Send + Sync>>,
    historical_read_complete_receiver: Option<Box<dyn Fn() + Send + Sync>>,
    connection_established_receiver: Option<Box<dyn Fn() + Send + Sync>>,
}
