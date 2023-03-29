//******************************************************************************************************
//  constants.rs - Gbtc
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
//******************************************************************************************************

#![allow(missing_copy_implementations)]

use bitflags::bitflags;

/// Defines default values for various STTP settings.
pub struct Defaults;

impl Defaults {
    /// Default for maximum number of retries for a connection attempt.
    pub const MAX_RETRIES: i32 = -1;

    /// Default for retry interval in seconds.
    pub const RETRY_INTERVAL: f64 = 1.0;

    /// Default for maximum retry interval in seconds.
    pub const MAX_RETRY_INTERVAL: f64 = 30.0;

    /// Default for auto-reconnect flag.
    pub const AUTO_RECONNECT: bool = true;

    /// Default for auto-request metadata flag.
    pub const AUTO_REQUEST_METADATA: bool = true;

    /// Default for auto-subscribe flag.
    pub const AUTO_SUBSCRIBE: bool = true;

    /// Default for compress payload data flag.
    pub const COMPRESS_PAYLOAD_DATA: bool = true;

    /// Default for compress metadata flag.
    pub const COMPRESS_METADATA: bool = true;

    /// Default for compress signal index cache flag.
    pub const COMPRESS_SIGNAL_INDEX_CACHE: bool = true;

    /// Default for metadata filters.
    pub const METADATA_FILTERS: &str = "";

    /// Default for socket timeout in seconds.
    pub const SOCKET_TIMEOUT: f64 = 2.0;

    /// Default for STTP version.
    pub const VERSION: u8 = 2;

    /// Default for filter expression.
    pub const FILTER_EXPRESSION: &str = "";

    /// Default for throttled flag.
    pub const THROTTLED: bool = false;

    /// Default for publish interval in seconds.
    pub const PUBLISH_INTERVAL: f64 = 1.0;

    /// Default for UDP data channel flag.
    pub const UDP_DATA_CHANNEL: bool = false;

    /// Default for local port for data channel.
    pub const DATA_CHANNEL_LOCAL_PORT: u16 = 0;

    /// Default for interface for data channel.
    pub const DATA_CHANNEL_INTERFACE: &str = "";

    /// Default for include time flag.
    pub const INCLUDE_TIME: bool = true;

    /// Default for enable time reasonability check flag.
    pub const ENABLE_TIME_REASONABILITY_CHECK: bool = false;

    /// Default for lag time in seconds.
    pub const LAG_TIME: f64 = 10.0;

    /// Default for lead time in seconds.
    pub const LEAD_TIME: f64 = 5.0;

    /// Default for use local clock as real time flag.
    pub const USE_LOCAL_CLOCK_AS_REAL_TIME: bool = false;

    /// Default for use millisecond resolution flag.
    pub const USE_MILLISECOND_RESOLUTION: bool = false;

    /// Default for request NAN-value filter flag.
    pub const REQUEST_NAN_VALUE_FILTER: bool = false;

    /// Default for start time.
    pub const START_TIME: &str = "";

    /// Default for stop time.
    pub const STOP_TIME: &str = "";

    /// Default for constraint parameters.
    pub const CONSTRAINT_PARAMETERS: &str = "";

    /// Default for processing interval in seconds.
    pub const PROCESSING_INTERVAL: i32 = -1;

    /// Default for extra connection string parameters.
    pub const EXTRA_CONNECTION_STRING_PARAMETERS: &str = "";
}

// TODO; Update bitflags crate when bug fix to expose flags and docs is released:
// https://github.com/bitflags/bitflags/issues/320

bitflags! {
    /// Defines the bitflags representing the possible quality states of a `Measurement` value.
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StateFlags: u32 {
        /// Measurement flag for a normal state.
        const NORMAL = 0x0;

        /// Measurement flag for a bad data state.
        const BAD_DATA = 0x1;

        /// Measurement flag for a suspect data state.
        const SUSPECT_DATA = 0x2;

        /// Measurement flag for an over range error, i.e., unreasonable high value.
        const OVER_RANGE_ERROR = 0x4;

        /// Measurement flag for an under range error, i.e., unreasonable low value.
        const UNDER_RANGE_ERROR = 0x8;

        /// Measurement flag for an alarm for high value.
        const ALARM_HIGH = 0x10;

        /// Measurement flag for an alarm for low value.
        const ALARM_LOW = 0x20;

        /// Measurement flag for a warning for high value.
        const WARNING_HIGH = 0x40;

        /// Measurement flag for a warning for low value.
        const WARNING_LOW = 0x80;

        /// Measurement flag for an alarm for flat-lined value, i.e., latched value test alarm.
        const FLATLINE_ALARM = 0x100;

        /// Measurement flag for a comparison alarm, i.e., outside threshold of comparison with a real-time value.
        const COMPARISON_ALARM = 0x200;

        /// Measurement flag for a rate-of-change alarm.
        const ROC_ALARM = 0x400;

        /// Measurement flag for a bad value received.
        const RECEIVED_AS_BAD = 0x800;

        /// Measurement flag for a calculated value state.
        const CALCULATED_VALUE = 0x1000;

        /// Measurement flag for a calculation error with the value.
        const CALCULATION_ERROR = 0x2000;

        /// Measurement flag for a calculation warning state.
        const CALCULATION_WARNING = 0x4000;

        /// Measurement flag for a reserved quality.
        const RESERVED_QUALITY_FLAG = 0x8000;

        /// Measurement flag for a bad time state.
        const BAD_TIME = 0x10000;

        /// Measurement flag for a suspect time state.
        const SUSPECT_TIME = 0x20000;

        /// Measurement flag for a late time alarm.
        const LATE_TIME_ALARM = 0x40000;

        /// Measurement flag for a future time alarm.
        const FUTURE_TIME_ALARM = 0x80000;

        /// Measurement flag for an up-sampled state.
        const UPSAMPLED = 0x100000;

        /// Measurement flag for a down-sampled state.
        const DOWNSAMPLED = 0x200000;

        /// Measurement flag for a discarded value state.
        const DISCARDED_VALUE = 0x400000;

        /// Measurement flag for a reserved time state.
        const RESERVED_TIME_FLAG = 0x800000;

        /// Measurement flag for user defined state 1.
        const USER_DEFINED_FLAG1 = 0x1000000;

        /// Measurement flag for user defined state 2.
        const USER_DEFINED_FLAG2 = 0x2000000;

        /// Measurement flag for user defined state 3.
        const USER_DEFINED_FLAG3 = 0x4000000;

        /// Measurement flag for user defined state 4.
        const USER_DEFINED_FLAG4 = 0x8000000;

        /// Measurement flag for user defined state 5.
        const USER_DEFINED_FLAG5 = 0x10000000;

        /// Measurement flag for a system error state.
        const SYSTEM_ERROR = 0x20000000;

        /// Measurement flag for a system warning state.
        const SYSTEM_WARNING = 0x40000000;

        /// Measurement flag for a measurement error state.
        const MEASUREMENT_ERROR = 0x80000000;
    }
}

bitflags! {
    /// Defines the bitflags representing the possible flags for a data packet.
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataPacketFlags: u8 {
        /// Determines if serialized measurement is compact.
        ///
        /// Obsolete: Bit will be removed in future version. Currently this bit is always set.
        const COMPACT = 0x02;

        /// Determines which cipher index to use when encrypting data packet.
        ///
        /// Bit set = use odd cipher index (i.e., 1), bit clear = use even cipher index (i.e., 0).
        const CIPHER_INDEX = 0x04;

        /// Determines if data packet payload is compressed.
        ///
        /// Bit set = payload compressed, bit clear = payload normal.
        const COMPRESSED = 0x08;

        /// Determines which signal index cache to use when decoding a data packet.
        /// Used by STTP version 2 or greater.
        ///
        /// Bit set = use odd cache index (i.e., 1), bit clear = use even cache index (i.e., 0).
        const CACHEINDEX = 0x10;

        /// Defines state where there are no flags set.
        const NO_FLAGS = 0x0;
    }
}

/// Enumeration of the possible server commands received by a `DataPublisher` and sent by a `DataSubscriber` during an STTP session.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ServerCommand {
    // Although the server commands and responses will be on two different paths, the response enumeration values
    // are defined as distinct from the command values to make it easier to identify codes from a wire analysis.

    //
    /// Command code handling connect operations.
    ///
    /// Only used as part of connection refused response -- value not sent on the wire.
    Connect = 0x00,

    /// Command code for requesting an updated set of metadata.
    MetadataRefresh = 0x01,

    /// Command code for requesting a subscription of streaming data from server based on connection string that follows.
    Subscribe = 0x02,

    /// Command code for requesting that server stop sending streaming data to the client and cancel the current subscription.
    Unsubscribe = 0x03,

    /// Command code for manually requesting that server send a new set of cipher keys for data packet encryption (UDP only).
    RotateCipherKeys = 0x04,

    /// Command code for manually requesting that server to update the processing interval with the following specified value.
    UpdateProcessingInterval = 0x05,

    /// Command code for establishing operational modes.
    ///
    /// As soon as connection is established, requests that server set operational modes that affect how the subscriber and publisher will communicate.
    DefineOperationalModes = 0x06,

    /// Command code for receipt of a notification.
    ///
    /// This message is sent in response to ServerResponse.Notify.
    ConfirmNotification = 0x07,

    /// Command code for receipt of a buffer block measurement.
    ///
    /// This message is sent in response to ServerResponse.BufferBlock.
    ConfirmBufferBlock = 0x08,

    /// Command code for receipt of a base time update.
    ///
    /// This message is sent in response to ServerResponse.UpdateBaseTimes.
    ConfirmUpdateBaseTimes = 0x09,

    /// Command code for confirming the receipt of a signal index cache.
    ///
    /// This allows publisher to safely transition to next signal index cache.
    ConfirmUpdateSignalIndexCache = 0x0A,

    /// Command code for confirming the receipt of a cipher key update.
    ///
    /// This verifies delivery of the cipher keys indicating that it is safe to transition to the new keys.
    ConfirmUpdateCipherKeys = 0x0B,

    /// Command code for requesting the primary metadata schema.
    GetPrimaryMetadataSchema = 0x0C,

    /// Command code for requesting the signal selection schema.
    GetSignalSelectionSchema = 0x0D,

    /// Command code handling user-defined commands.
    UserCommand00 = 0xD0,

    /// Command code handling user-defined commands.
    UserCommand01 = 0xD1,

    /// Command code handling user-defined commands.
    UserCommand02 = 0xD2,

    /// Command code handling user-defined commands.
    UserCommand03 = 0xD3,

    /// Command code handling user-defined commands.
    UserCommand04 = 0xD4,

    /// Command code handling user-defined commands.
    UserCommand05 = 0xD5,

    /// Command code handling user-defined commands.
    UserCommand06 = 0xD6,

    /// Command code handling user-defined commands.
    UserCommand07 = 0xD7,

    /// Command code handling user-defined commands.
    UserCommand08 = 0xD8,

    /// Command code handling user-defined commands.
    UserCommand09 = 0xD9,

    /// Command code handling user-defined commands.
    UserCommand10 = 0xDA,

    /// Command code handling user-defined commands.
    UserCommand11 = 0xDB,

    /// Command code handling user-defined commands.
    UserCommand12 = 0xDC,

    /// Command code handling user-defined commands.
    UserCommand13 = 0xDD,

    /// Command code handling user-defined commands.
    UserCommand14 = 0xDE,

    /// Command code handling user-defined commands.
    UserCommand15 = 0xDF,
}
