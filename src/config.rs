//******************************************************************************************************
//  config.rs - Gbtc
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

/// Defines STTP connection related configuration parameters.
#[derive(Clone)]
pub struct Config {
    /// Defines the maximum number of times to retry a connection.
    /// Set value to -1 to retry infinitely.
    /// Note: setting ignored for listening connections.
    pub max_retries: i32,

    /// Defines the base retry interval, in milliseconds. Retries will
    /// exponentially back-off starting from this interval.
    /// Note: setting ignored for listening connections.
    pub retry_interval: i32,

    /// Defines the maximum retry interval, in milliseconds.
    /// Note: setting ignored for listening connections.
    pub max_retry_interval: i32,

    /// Defines flag that determines if connections should be
    /// automatically reattempted.
    /// Note: setting ignored for listening connections.
    pub auto_reconnect: bool,

    /// Defines the flag that determines if metadata should be
    /// automatically requested upon successful connection. When true, metadata will
    /// be requested upon connection before subscription; otherwise, any metadata
    /// operations must be handled manually.
    pub auto_request_metadata: bool,

    /// Defines the flag that determines if subscription should be
    /// handled automatically upon successful connection. When AutoRequestMetadata
    /// is true and AutoSubscribe is true, subscription will occur after reception
    /// of metadata. When AutoRequestMetadata is false and AutoSubscribe is true,
    /// subscription will occur at successful connection. When AutoSubscribe is
    /// false, any subscribe operations must be handled manually.
    pub auto_subscribe: bool,

    /// Determines whether payload data is compressed.
    pub compress_payload_data: bool,

    /// Determines whether the metadata transfer is compressed.
    pub compress_metadata: bool,

    /// Determines whether the signal index cache is compressed.
    pub compress_signal_index_cache: bool,

    /// Defines any filters to be applied to incoming metadata to reduce total
    /// received metadata. Each filter expression should be separated by semi-colon.
    pub metadata_filters: String,

    /// Defines the target STTP protocol version. This currently defaults to 2.
    pub version: u8,
}

/// Define the default values for an STTP connection Config.
impl Default for Config {
    fn default() -> Self {
        Self {
            max_retries: -1,
            retry_interval: 1000,
            max_retry_interval: 30_000,
            auto_reconnect: true,
            auto_request_metadata: true,
            auto_subscribe: true,
            compress_payload_data: true,
            compress_metadata: true,
            compress_signal_index_cache: true,
            metadata_filters: "".to_string(),
            version: 2,
        }
    }
}
