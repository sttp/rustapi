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

use bitflags::bitflags;

/// Defines default values for various STTP settings.
#[derive(Clone, Copy)]
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
        /// Currently this bit is always set.
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
///
/// Solicited server commands will receive a `ServerResponse.Succeeded` or `ServerResponse.Failed` response code along with an
/// associated success or failure message. Message type for successful responses will be based  on server command - for example,
/// server response for a successful MetaDataRefresh command will return a serialized `DataSet` of the available server metadata.
/// Message type for failed responses will always be a string of text representing the error message.
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
    ///
    /// Successful return message type will be a `DataSet` containing server device and measurement metadata.
    /// Devices and measurements contain unique UUIDs that should be used to key metadata updates in local repository.
    /// Optional string based message can follow command that should represent client requested meta-data filtering
    /// expressions, e.g.: "FILTER MeasurementDetail WHERE SignalType != 'STAT'"
    MetadataRefresh = 0x01,

    /// Command code for requesting a subscription of streaming data from server based on connection string that follows.
    ///
    /// It will not be necessary to stop an existing subscription before requesting a new one. Successful return message
    /// type will be string indicating total number of allowed points. Client should wait for `UpdateSignalIndexCache` and
    /// `UpdateBaseTime` response codes before attempting to parse data when using the compact measurement format.
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
    /// This message is sent in response to `ServerResponse.Notify`.
    ConfirmNotification = 0x07,

    /// Command code for receipt of a buffer block measurement.
    ///
    /// This message is sent in response to `ServerResponse.BufferBlock`.
    ConfirmBufferBlock = 0x08,

    /// Command code for receipt of a base time update.
    ///
    /// This message is sent in response to `ServerResponse.UpdateBaseTimes`.
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

/// Enumeration of the possible server responses sent by `DataPublisher` and received by `DataSubscriber`
/// during an STTP session.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ServerResponse {
    // Although the server commands and responses will be on two different paths, the response enumeration values
    // are defined as distinct from the command values to make it easier to identify codes from a wire analysis.

    //
    /// Response code indicating a succeeded response.
    ///
    /// Informs client that its solicited server command succeeded, original command and success message follow.
    Succeeded = 0x80,

    /// Response code indicating a failed response.
    ///
    /// Informs client that its solicited server command failed, original command and failure message follow.
    Failed = 0x81,

    /// Response code indicating a data packet.
    ///
    /// Unsolicited response informs client that a data packet follows.
    DataPacket = 0x82,

    /// Response code indicating a signal index cache update.
    ///
    /// Unsolicited response requests that client update its runtime signal index cache with the one that follows.
    UpdateSignalIndexCache = 0x83,

    /// Response code indicating a runtime base-timestamp offsets have been updated.
    ///
    /// Unsolicited response requests that client update its runtime base-timestamp offsets with those that follow.
    UpdateBaseTimes = 0x84,

    /// Response code indicating a runtime cipher keys have been updated.
    ///
    /// Response, solicited or unsolicited, requests that client update its runtime data cipher keys with those that follow.
    UpdateCipherKeys = 0x85,

    /// Response code indicating the start time of data being published.
    ///
    /// Unsolicited response provides the start time of data being processed from the first measurement.
    DataStartTime = 0x86,

    /// Response code indicating that processing has completed.
    ///
    /// Unsolicited response provides notification that input processing has completed, typically via temporal constraint.
    ProcessingComplete = 0x87,

    /// Response code indicating a buffer block.
    ///
    /// Unsolicited response informs client that a raw buffer block follows.
    BufferBlock = 0x88,

    /// Response code indicating a notification.
    ///
    /// Unsolicited response provides a notification message to the client.
    Notify = 0x89,

    /// Response code indicating a that the publisher configuration metadata has changed.
    ///
    /// Unsolicited response provides a notification that the publisher's source configuration has changed and that
    /// client may want to request a meta-data refresh.
    ConfigurationChanged = 0x8A,

    /// Response code handling user-defined responses.
    UserResponse00 = 0xE0,

    /// Response code handling user-defined responses.
    UserResponse01 = 0xE1,

    /// Response code handling user-defined responses.
    UserResponse02 = 0xE2,

    /// Response code handling user-defined responses.
    UserResponse03 = 0xE3,

    /// Response code handling user-defined responses.
    UserResponse04 = 0xE4,

    /// Response code handling user-defined responses.
    UserResponse05 = 0xE5,

    /// Response code handling user-defined responses.
    UserResponse06 = 0xE6,

    /// Response code handling user-defined responses.
    UserResponse07 = 0xE7,

    /// Response code handling user-defined responses.
    UserResponse08 = 0xE8,

    /// Response code handling user-defined responses.
    UserResponse09 = 0xE9,

    /// Response code handling user-defined responses.
    UserResponse10 = 0xEA,

    /// Response code handling user-defined responses.
    UserResponse11 = 0xEB,

    /// Response code handling user-defined responses.
    UserResponse12 = 0xEC,

    /// Response code handling user-defined responses.
    UserResponse13 = 0xED,

    /// Response code handling user-defined responses.
    UserResponse14 = 0xEE,

    /// Response code handling user-defined responses.
    UserResponse15 = 0xEF,

    /// Response code indicating a null-operation keep-alive ping.
    ///
    /// The command channel can remain quiet for some time, this command allows a period test of client connectivity.
    NoOP = 0xFF,
}

bitflags! {
    /// Enumeration of the possible modes that affect how `DataPublisher` and `DataSubscriber` communicate during as STTP session.
    ///
    /// Operational modes are sent from a subscriber to a publisher to request operational behaviors for the
    /// connection, as a result the operation modes must be sent before any other command. The publisher may
    /// silently refuse some requests (e.g., compression) based on its configuration. Operational modes only
    /// apply to fundamental protocol control.
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OperationalModes : u32 {
        /// Bit mask used to get version number of protocol.
        ///
        /// Version number is currently set to 2.
        const VersionMask = 0x000000FF;

        ///Bit mask used to get character encoding used when exchanging messages between publisher and subscriber.
        ///
        /// STTP currently only supports UTF-8 string encoding.
        const EncodingMask = 0x00000300;

        /// Bit mask used to apply an implementation-specific extension to STTP.
        ///
        /// If the value is zero, no implementation specific extensions are applied.
        /// If the value is non-zero, an implementation specific extension is applied, and all parties need to coordinate and agree to the extension.
        /// If extended flags are unsupported, returned failure message text should be prefixed with UNSUPPORTED EXTENSION: as the context reference.
        const ImplementationSpecificExtensionMask = 0x00FF0000;

        /// Bit flag used to determine whether external measurements are exchanged during metadata synchronization.
        ///
        /// Bit set = external measurements are exchanged, bit clear = no external measurements are exchanged
        const ReceiveExternalMetadata = 0x02000000;

        /// Bit flag used to determine whether internal measurements are exchanged during metadata synchronization.
        ///
        /// Bit set = internal measurements are exchanged, bit clear = no internal measurements are exchanged
        const ReceiveInternalMetadata = 0x04000000;

        /// Bit flag used to determine whether payload data is compressed when exchanging between publisher and subscriber.
        ///
        /// Bit set = compress, bit clear = no compression
        const CompressPayloadData = 0x20000000;

        /// Bit flag used to determine whether the signal index cache is compressed when exchanging between publisher and subscriber.
        ///
        /// Bit set = compress, bit clear = no compression
        const CompressSignalIndexCache = 0x40000000;

        /// Bit flag used to determine whether metadata is compressed when exchanging between publisher and subscriber.
        ///
        /// Bit set = compress, bit clear = no compression
        const CompressMetadata = 0x80000000;

        /// State where there are no flags set.
        const NoFlags = 0x00000000;
    }
}

/// Enumeration of the possible string encoding options of an STTP session.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum OperationalEncoding {
    /// Targets little-endian 16-bit Unicode character encoding for strings.
    #[deprecated = "STTP currently only supports UTF-8 string encoding."]
    UTF16LE = 0x00000000,

    /// Targets big-endian 16-bit Unicode character encoding for strings.
    #[deprecated = "STTP currently only supports UTF-8 string encoding."]
    UTF16BE = 0x00000100,

    /// Targets 8-bit variable-width Unicode character encoding for strings.
    UTF8 = 0x00000200,
}

bitflags! {
    /// Enumeration of the possible compression modes supported by STTP.
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[deprecated = "Only used for backwards compatibility with pre-standard STTP implementations. OperationalModes now supports custom compression types"]
    pub struct CompressionModes : u32
    {
        /// Bit flag used determine if GZip compression will be used to metadata exchange.
        const GZip = 0x00000020;

        /// Bit flag used determine if the time-series special compression algorithm will be used for data exchange.
        const TSSC = 0x00000040;

        /// Defines state where no compression will be used.
        const None = 0x00000000;
    }
}

/// Enumeration of the possible security modes used by the `DataPublisher` to secure data
/// sent over the command channel in STTP.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SecurityMode {
    /// Defines security mode where data will be sent over the wire unencrypted.
    Off,

    /// Defines security mode where data will be sent over wire using Transport Layer Security (TLS).
    TLS,
}

/// Enumeration of the possible connection status results used by the `SubscriberConnector`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectStatus {
    /// Connection succeeded status.
    Success = 1,

    /// Connection failed status.
    Failed = 0,

    /// Connection cancelled status.
    Canceled = -1,
}
