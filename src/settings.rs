//******************************************************************************************************
//  settings.rs - Gbtc
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

/// Defines STTP subscription related settings.
///
/// `Settings` exists as a simplified implementation of the `SubscriptionInfo`
/// found in the `transport` module. Internally, the `Subscriber` class maps
/// `Settings` values to a `SubscriptionInfo` instance for use with a `DataSubscriber`.
#[derive(Clone)]
pub struct Settings {
    /// Determines if data will be published using down-sampling.
    pub throttled: bool,
    /// Defines the down-sampling publish interval, in seconds, to use when Throttled is true.
    pub publish_interval: f64,

    /// Defines the desired UDP port to use for publication. Zero value means do not receive data on UDP, i.e.,
    /// data will be delivered to the STTP client via TCP.
    pub udp_port: u16,

    /// Determines if time should be included in non-compressed, compact measurements.
    pub include_time: bool,

    /// Determines if publisher should perform time reasonability checks.
    /// When enabled LagTime and LeadTime will be used to determine if a measurement timestamp is reasonable.
    pub enable_time_reasonability_check: bool,

    /// Defines the allowed past time deviation tolerance in seconds (can be sub-second).
    /// Value is used to determine if a measurement timestamp is reasonable.
    /// Only applicable when EnableTimeReasonabilityCheck is true.
    pub lag_time: f64,

    /// Defines the allowed future time deviation tolerance in seconds (can be sub-second).
    /// Value is used to determine if a measurement timestamp is reasonable.
    /// Only applicable when EnableTimeReasonabilityCheck is true.
    pub lead_time: f64,

    /// Determines if publisher should use local clock as real time. If false,
    /// the timestamp of the latest measurement will be used as real-time.
    /// Only applicable when EnableTimeReasonabilityCheck is true.
    pub use_local_clock_as_real_time: bool,

    /// Determines if time should be restricted to milliseconds in non-compressed, compact measurements.
    pub use_millisecond_resolution: bool,

    /// Requests that the publisher filter, i.e., does not send, any NaN values.
    pub request_nan_value_filter: bool,

    /// Defines the start time for a requested temporal data playback, i.e., a historical subscription.
    /// Simply by specifying a StartTime and StopTime, a subscription is considered a historical subscription.
    /// Note that the publisher may not support historical subscriptions, in which case the subscribe will fail.
    pub start_time: String,

    /// Defines the stop time for a requested temporal data playback, i.e., a historical subscription.
    /// Simply by specifying a StartTime and StopTime, a subscription is considered a historical subscription.
    /// Note that the publisher may not support historical subscriptions, in which case the subscribe will fail.
    pub stop_time: String,

    //// Defines any custom constraint parameters for a requested temporal data playback. This can
    /// include parameters that may be needed to initiate, filter, or control historical data access.
    pub constraint_parameters: String,

    /// Defines the initial playback speed, in milliseconds, for a requested temporal data playback.
    /// With the exception of the values of -1 and 0, this value specifies the desired processing interval for data, i.e.,
    /// basically a delay, or timer interval, over which to process data. A value of -1 means to use the default processing
    /// interval while a value of 0 means to process data as fast as possible.
    pub processing_interval: i32,

    /// Defines any extra custom connection string parameters that may be needed for a subscription.
    pub extra_connection_string_parameters: String,
}

/// Define the default values for STTP subscription Settings.
impl Default for Settings {
    fn default() -> Self {
        Self {
            throttled: false,
            publish_interval: 1.0,
            udp_port: 0,
            include_time: true,
            enable_time_reasonability_check: false,
            lag_time: 10.0,
            lead_time: 5.0,
            use_local_clock_as_real_time: false,
            use_millisecond_resolution: false,
            request_nan_value_filter: false,
            start_time: "".to_string(),
            stop_time: "".to_string(),
            constraint_parameters: "".to_string(),
            processing_interval: -1,
            extra_connection_string_parameters: "".to_string(),
        }
    }
}
