//******************************************************************************************************
//  subscription_info.rs - Gbtc
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
//******************************************************************************************************

use crate::transport::constants::Defaults;

/// Defines subscription related settings for a `DataSubscriber` instance.
#[derive(Debug, Clone)]
pub struct SubscriptionInfo {
    /// Gets or sets the desired measurements for a subscription. Examples include:
    ///
    /// * Directly specified signal IDs (UUID values in string format):
    /// > `38A47B0-F10B-4143-9A0A-0DBC4FFEF1E8; {E4BBFE6A-35BD-4E5B-92C9-11FF913E7877}`
    /// * Directly specified tag names:
    /// > `DOM_GPLAINS-BUS1:VH; TVA_SHELBY-BUS1:VH`
    /// * A filter expression against a selection view:
    /// > `FILTER ActiveMeasurements WHERE Company = 'GPA' AND SignalType = 'FREQ'`
    pub filter_expression: String,

    /// Gets or sets flag that determines if data will be published using down-sampling.
    pub throttled: bool,

    /// Gets or sets the down-sampling publish interval to use when `Throttled` is `true`.
    pub publish_interval: f64,

    /// Gets or sets flag that requests that a UDP channel be used for data publication.
    pub udp_data_channel: bool,

    /// Gets or sets the desired UDP port to use for publication.
    pub data_channel_local_port: u16,

    /// Gets or sets the desired network interface to use for UDP publication.
    pub data_channel_interface: String,

    /// Gets or sets flag that determines if time should be included in non-compressed, compact measurements.
    pub include_time: bool,

    /// Determines if publisher should perform time reasonability checks.
    /// When enabled `lag_time` and `lead_time` will be used to determine if a measurement timestamp is reasonable.
    pub enable_time_reasonability_check: bool,

    /// Defines defines the allowed past time deviation tolerance in seconds (can be sub-second).
    /// Value is used to determine if a measurement timestamp is reasonable.
    /// Only applicable when `enable_time_reasonability_check` is `true`.
    pub lag_time: f64,

    /// Defines defines the allowed future time deviation tolerance in seconds (can be sub-second).
    /// Value is used to determine if a measurement timestamp is reasonable.
    /// Only applicable when `enable_time_reasonability_check` is `true`.
    pub lead_time: f64,

    /// Determines if publisher should use local clock as real time. If false,
    /// the timestamp of the latest measurement will be used as real-time.
    /// Only applicable when `enable_time_reasonability_check` is `true`.
    pub use_local_clock_as_real_time: bool,

    /// Gets or sets flag that determines if time should be restricted to milliseconds in non-compressed, compact measurements.
    pub use_millisecond_resolution: bool,

    /// Gets or sets flag that requests that the publisher filter, i.e., does not send, any `NaN` values.
    pub request_nan_value_filter: bool,

    /// Gets or sets that defines the start time for a requested temporal data playback, i.e., a historical subscription.
    /// Simply by specifying a `StartTime` and `StopTime`, a subscription is considered a historical subscription.
    /// Note that the publisher may not support historical subscriptions, in which case the subscribe will fail.
    pub start_time: String,

    /// Gets or sets that defines the stop time for a requested temporal data playback, i.e., a historical subscription.
    /// Simply by specifying a `StartTime` and `StopTime`, a subscription is considered a historical subscription.
    /// Note that the publisher may not support historical subscriptions, in which case the subscribe will fail.
    pub stop_time: String,

    /// Gets or sets any custom constraint parameters for a requested temporal data playback. This can include
    /// parameters that may be needed to initiate, filter, or control historical data access.
    pub constraint_parameters: String,

    /// Gets or sets the initial playback speed, in milliseconds, for a requested temporal data playback.
    /// With the exception of the values of -1 and 0, this value specifies the desired processing interval for data, i.e.,
    /// basically a delay, or timer interval, over which to process data.A value of -1 means to use the default processing
    /// interval while a value of 0 means to process data as fast as possible.
    pub processing_interval: i32,

    /// Gets or sets any extra or custom connection string parameters that may be needed for a subscription.
    pub extra_connection_string_parameters: String,
}

/// Define the default values for STTP `SubscriptionInfo`.
impl Default for SubscriptionInfo {
    fn default() -> Self {
        Self {
            filter_expression: Defaults::FILTER_EXPRESSION.to_string(),
            throttled: Defaults::THROTTLED,
            publish_interval: Defaults::PUBLISH_INTERVAL,
            udp_data_channel: Defaults::ENABLE_UDP_DATA_CHANNEL,
            data_channel_local_port: Defaults::DATA_CHANNEL_LOCAL_PORT,
            data_channel_interface: Defaults::DATA_CHANNEL_INTERFACE.to_string(),
            include_time: Defaults::INCLUDE_TIME,
            enable_time_reasonability_check: Defaults::ENABLE_TIME_REASONABILITY_CHECK,
            lag_time: Defaults::LAG_TIME,
            lead_time: Defaults::LEAD_TIME,
            use_local_clock_as_real_time: Defaults::USE_LOCAL_CLOCK_AS_REAL_TIME,
            use_millisecond_resolution: Defaults::USE_MILLISECOND_RESOLUTION,
            request_nan_value_filter: Defaults::REQUEST_NAN_VALUE_FILTER,
            start_time: Defaults::START_TIME.to_string(),
            stop_time: Defaults::STOP_TIME.to_string(),
            constraint_parameters: Defaults::CONSTRAINT_PARAMETERS.to_string(),
            processing_interval: Defaults::PROCESSING_INTERVAL,
            extra_connection_string_parameters: Defaults::EXTRA_CONNECTION_STRING_PARAMETERS
                .to_string(),
        }
    }
}
