//******************************************************************************************************
//  data_subscriber.rs - Gbtc
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
//  04/01/2023 - J. Ritchie Carroll
//       Generated original version of source code.
//
//******************************************************************************************************

// TODO: Remove later
#![allow(dead_code)]

use crate::transport::constants::Defaults;
use crate::transport::OperationalEncoding;
use crate::transport::SignalIndexCache;
use crate::transport::SubscriberConnector;
use crate::transport::SubscriptionInfo;
use crate::Version;

use chrono::DateTime;
use chrono::Utc;
use std::error::Error;
use std::fmt;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::Shutdown;
use std::net::TcpStream;
use std::option::Option;
use std::str::from_utf8;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::vec;
use uuid::Uuid;

use super::DataPacketFlags;
use super::Measurement;
use super::ServerCommand;

const MAX_PACKET_SIZE: usize = 32768;
const PAYLOAD_HEADER_SIZE: usize = 4;
const RESPONSE_HEADER_SIZE: usize = 6;
const EVEN_KEY: usize = 0;
const ODD_KEY: usize = 1;
const KEY_INDEX: usize = 0;
const IV_INDEX: usize = 1;
const MISSING_CACHE_WARNING_INTERVAL: f64 = 20.0;

/// Represents a subscription for an STTP connection.
//#[derive(Debug)]
pub struct DataSubscriber<'a> {
    subscription: SubscriptionInfo,
    subscriber_id: Uuid,
    encoding: OperationalEncoding,
    pub(crate) connector: Arc<Mutex<SubscriberConnector>>,
    connected: AtomicBool,
    validated: AtomicBool,
    listening: AtomicBool,
    subscribed: AtomicBool,

    command_channel_socket: Mutex<Option<TcpStream>>,
    command_channel_response_thread: Option<JoinHandle<()>>,
    data_channel_socket: Mutex<Option<TcpStream>>,
    data_channel_response_thread: Option<JoinHandle<()>>,
    connection_id: String,

    connect_action_mutex: Mutex<()>,
    connection_termination_thread_mutex: Mutex<Option<JoinHandle<()>>>,

    disconnect_thread_mutex: Mutex<Option<JoinHandle<()>>>,
    disconnecting: AtomicBool,
    disconnected: AtomicBool,
    pub(crate) disposing: AtomicBool,

    // Statistics counters
    total_command_channel_bytes_received: AtomicU64,
    total_data_channel_bytes_received: AtomicU64,
    total_measurements_received: AtomicU64,

    /// Called when a informational message should be logged.
    pub status_message_callback: Mutex<Option<Arc<dyn Fn(&str) + Send + Sync + 'a>>>,

    /// Called when an error message should be logged.
    pub error_message_callback: Mutex<Option<Arc<dyn Fn(&str) + Send + Sync + 'a>>>,

    /// Called when a DataSubscriber connection has been established.
    pub connection_established_callback: Mutex<Option<Arc<dyn Fn() + Send + Sync + 'a>>>,

    /// Called when `DataSubscriber` terminates its connection.
    pub connection_terminated_callback: Mutex<Option<Arc<dyn Fn() + Send + Sync + 'a>>>,

    /// Called when `DataSubscriber` automatically reconnects.
    pub auto_reconnect_callback: Mutex<Option<Arc<dyn Fn() + Send + Sync + 'a>>>,

    /// Called when `DataSubscriber` receives a metadata response.
    pub metadata_received_callback: Mutex<Option<Arc<dyn Fn(&[u8]) + Send + Sync + 'a>>>,

    /// Called when `DataSubscriber` receives a new signal index cache response.
    pub subscription_updated_callback:
        Mutex<Option<Arc<dyn Fn(&SignalIndexCache) + Send + Sync + 'a>>>,

    /// Called with timestamp of first received measurement in a subscription.
    pub data_start_time_callback: Mutex<Option<Arc<dyn Fn(u64) + Send + Sync + 'a>>>,

    /// Called when the `DataPublisher` sends a notification that configuration has changed.
    pub configuration_changed_callback: Mutex<Option<Arc<dyn Fn() + Send + Sync + 'a>>>,

    /// Called when `DataSubscriber` receives a set of new measurements from the `DataPublisher`.
    pub new_measurements_callback:
        Mutex<Option<Arc<dyn Fn(Vec<&dyn Measurement>) + Send + Sync + 'a>>>,

    // /// Called when `DataSubscriber` receives a set of new buffer block measurements from the `DataPublisher`.
    //pub new_bufferblocks_callback: Mutex<Option<Arc<dyn Fn(Vec<&BufferBlock>) + Send + Sync>>>,

    //
    /// Called when the `DataPublisher` sends a notification that temporal processing has completed, i.e., the end of a historical playback data stream has been reached.
    pub processing_complete_callback: Mutex<Option<Arc<dyn Fn(&str) + Send + Sync + 'a>>>,

    /// Called when the `DataPublisher` sends a notification that requires receipt.
    pub notification_received_callback: Mutex<Option<Arc<dyn Fn(&str) + Send + Sync + 'a>>>,

    /// Determines whether payload data is compressed, defaults to TSSC.
    pub compress_payload_data: bool,

    /// Determines whether the metadata transfer is compressed, defaults to GZip.
    pub compress_metadata: bool,

    /// Determines whether the signal index cache is compressed, defaults to GZip.
    pub compress_signal_index_cache: bool,

    /// Defines the STTP protocol version used by this library.
    pub version: String,

    /// Defines the STTP library API title as identification information of `DataSubscriber` to a `DataPublisher`.
    pub sttp_source_info: String,

    /// Defines the STTP library API version as identification information of `DataSubscriber` to a `DataPublisher`.
    pub sttp_version_info: String,

    /// Defines when the STTP library API was last updated as identification information of `DataSubscriber` to a `DataPublisher`.
    pub sttp_updated_on_info: String,

    // /// Defines the metadata cache associated with this `DataSubscriber`.
    //pub metadata_cache: MetadataCache,

    //
    /// Defines the socket timeout in seconds for the `DataSubscriber` connection.
    pub socket_timeout: f64,

    // Measurement parsing
    metadata_requested: DateTime<Utc>,
    signal_index_cache: [SignalIndexCache; 2],
    signal_index_cache_mutex: Mutex<()>,
    cache_index: usize,
    time_index: usize,
    base_time_offsets: [i64; 2],
    key_ivs: Option<Vec<Vec<u8>>>,
    last_missing_cache_warning: f64,
    tssc_reset_requested: AtomicBool,
    tssc_last_oos_report: DateTime<Utc>,
    tssc_last_oos_report_mutex: Mutex<()>,
    //buffer_block_expected_sequence_number: u32,
    //buffer_block_cache: Vec<BufferBlock>,
}

impl<'a> DataSubscriber<'a> {
    /// Creates a new `DataSubscriber` instance.
    pub fn new() -> Self {
        DataSubscriber {
            subscription: SubscriptionInfo::default(),
            subscriber_id: Uuid::nil(),
            encoding: OperationalEncoding::UTF8,
            connector: Arc::new(Mutex::new(SubscriberConnector::new())),
            connected: AtomicBool::new(false),
            validated: AtomicBool::new(false),
            listening: AtomicBool::new(false),
            subscribed: AtomicBool::new(false),
            command_channel_socket: Mutex::new(None),
            command_channel_response_thread: None,
            data_channel_socket: Mutex::new(None),
            data_channel_response_thread: None,
            connection_id: "".to_string(),
            connect_action_mutex: Mutex::new(()),
            connection_termination_thread_mutex: Mutex::new(None),
            disconnect_thread_mutex: Mutex::new(None),
            disconnecting: AtomicBool::new(false),
            disconnected: AtomicBool::new(false),
            disposing: AtomicBool::new(false),
            total_command_channel_bytes_received: AtomicU64::new(0),
            total_data_channel_bytes_received: AtomicU64::new(0),
            total_measurements_received: AtomicU64::new(0),
            status_message_callback: Mutex::new(None),
            error_message_callback: Mutex::new(None),
            connection_established_callback: Mutex::new(None),
            connection_terminated_callback: Mutex::new(None),
            auto_reconnect_callback: Mutex::new(None),
            metadata_received_callback: Mutex::new(None),
            subscription_updated_callback: Mutex::new(None),
            data_start_time_callback: Mutex::new(None),
            configuration_changed_callback: Mutex::new(None),
            new_measurements_callback: Mutex::new(None),
            //new_buffer_blocks_callback:Mutex::new(None),
            processing_complete_callback: Mutex::new(None),
            notification_received_callback: Mutex::new(None),
            compress_payload_data: Defaults::COMPRESS_PAYLOAD_DATA,
            compress_metadata: Defaults::COMPRESS_METADATA,
            compress_signal_index_cache: Defaults::COMPRESS_SIGNAL_INDEX_CACHE,
            version: Defaults::VERSION.to_string(),
            sttp_source_info: Version::STTP_SOURCE.to_string(),
            sttp_version_info: Version::STTP_VERSION.to_string(),
            sttp_updated_on_info: Version::STTP_UPDATED_ON.to_string(),
            //metadata_cache: MetadataCache::new(),
            socket_timeout: Defaults::SOCKET_TIMEOUT,
            metadata_requested: DateTime::default(),
            signal_index_cache: [SignalIndexCache::new(), SignalIndexCache::new()],
            signal_index_cache_mutex: Mutex::new(()),
            cache_index: 0,
            time_index: 0,
            base_time_offsets: [0, 0],
            key_ivs: None,
            last_missing_cache_warning: 0.0,
            tssc_reset_requested: AtomicBool::new(false),
            tssc_last_oos_report: DateTime::default(),
            tssc_last_oos_report_mutex: Mutex::new(()),
            //buffer_block_expected_sequence_number: 0,
            //buffer_block_cache: Vec::new(),
        }
    }

    /// Cleanly shuts down a `DataSubscriber` that is no longer being used, e.g., during a normal application exit.
    pub fn dispose(&self) {
        self.disposing.store(true, Ordering::SeqCst);
        self.connector.lock().unwrap().dispose();

        // TODO: Implement
        //self.disconnect(true, false);
    }

    /// Determines if a `DataSubscriber` is currently connected to a `DataPublisher`.
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    /// Determines if a `DataSubscriber` connection has been validated as an STTP connection.
    pub fn is_validated(&self) -> bool {
        self.validated.load(Ordering::SeqCst)
    }

    /// Determines if a `DataSubscriber` is currently listening for a `DataPublisher`
    /// connection, i.e., `DataSubscriber` is in reverse connection mode.
    pub fn is_listening(&self) -> bool {
        self.listening.load(Ordering::SeqCst)
    }

    /// Determines if a DataSubscriber is currently subscribed to a data stream.
    pub fn is_subscribed(&self) -> bool {
        self.subscribed.load(Ordering::SeqCst)
    }

    /// Returns the IP address and DNS host name, if resolvable, of current STTP connection.
    pub fn get_connection_id(&self) -> &str {
        &self.connection_id
    }

    /// Encodes an STTP string according to the defined operational modes.
    pub fn encode_string(&self, data: &str) -> Vec<u8> {
        // Latest version of STTP only encodes to UTF8
        if self.encoding != OperationalEncoding::UTF8 {
            panic!("Rust implementation of STTP only supports UTF8 string encoding")
        }

        data.as_bytes().to_vec()
    }

    /// Decodes an STTP string according to the defined operational modes.
    pub fn decode_string(&self, data: Vec<u8>) -> String {
        // Latest version of STTP only encodes to UTF8
        if self.encoding != OperationalEncoding::UTF8 {
            panic!("Rust implementation of STTP only supports UTF8 string encoding")
        }

        String::from_utf8(data).unwrap()
    }

    // TODO: Implement
    // /// Gets the `MeasurementMetadata` for the specified signalID from the local
    // /// registry. If the metadata does not exist, a new record is created and returned.
    // pub fn lookup_metadata(&self, id: &str) -> &MeasurementMetadata {

    // /// Gets the `MeasurementMetadata` associated with a measurement from the local
    // /// registry. If the metadata does not exist, a new record is created and returned.
    // pub fn metadata(&self, measurement: Measurement) &MeasurementMetadata {

    // /// AdjustedValue gets the Value of a Measurement with any linear adjustments applied from the
    // /// measurement's Adder and Multiplier metadata, if found.
    // pub fn AdjustedValue(&self, measurement: Measurement) float64 {

    /// Requests the the `DataSubscriber` initiate a connection to the `DataPublisher`.
    pub fn connect(&mut self, hostname: &str, port: u16) -> Result<(), Box<dyn Error>> {
        //  User requests to connection are not an auto-reconnect attempt
        self._connect(hostname, port, false)
    }

    fn _connect(
        &mut self,
        hostname: &str,
        port: u16,
        auto_reconnecting: bool,
    ) -> Result<(), Box<dyn Error>> {
        if self.connected.load(Ordering::SeqCst) {
            return Err("subscriber is already connected; disconnect first".into());
        }

        if self.listening.load(Ordering::SeqCst) {
            return Err(
                "subscriber is listening for connections; direct connections disallowed".into(),
            );
        }

        // Make sure any pending disconnect has completed to make sure socket is closed
        let mut disconnect_thread_guard = match self.disconnect_thread_mutex.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return Err("failed to lock disconnect thread mutex".into());
            }
        };

        if let Some(discconnect_thread) = disconnect_thread_guard.take() {
            if discconnect_thread.join().is_err() {
                return Err("failed to join disconnect thread".into());
            }
        }

        drop(disconnect_thread_guard);

        // Let any pending connect or disconnect operation complete before new connect,
        // this prevents destruction disconnect before connection is completed
        if self.connect_action_mutex.lock().is_err() {
            return Err("failed to lock connect action mutex".into());
        }

        // Initialize connection state
        self.setup_connection();

        match self.connector.lock() {
            Ok(mut guard) => {
                if !auto_reconnecting {
                    guard.reset_connection();
                }
                guard.connection_refused.store(false, Ordering::SeqCst);
            }
            Err(e) => {
                return Err(format!("failed to lock connector: {}", e).into());
            }
        }

        // TODO: Add TLS implementation options
        match TcpStream::connect(hostname.to_owned() + ":" + port.to_string().as_str()) {
            Ok(stream) => {
                self.establish_connection(stream, false);
            }
            Err(e) => {
                return Err(format!("failed to connect: {}", e).into());
            }
        }

        Ok(())
    }

    fn setup_connection(&mut self) {
        self.disconnected.store(false, Ordering::SeqCst);
        self.subscribed.store(false, Ordering::SeqCst);

        self.total_command_channel_bytes_received
            .store(0, Ordering::SeqCst);
        self.total_data_channel_bytes_received
            .store(0, Ordering::SeqCst);
        self.total_measurements_received.store(0, Ordering::SeqCst);

        self.key_ivs = None;

        // TODO: Implement
        //self.buffer_block_expected_sequence_number = 0;
        //self.measurement_registry = Dict(Uuid, MeasurementMetadata);
    }

    fn establish_connection(&mut self, stream: TcpStream, listening: bool) {
        let mut addr_name = "<unknown>".to_string();

        if listening {
            // TODO: Implement / Add DNS resolution
        } else {
            match self.connector.lock() {
                Ok(guard) => {
                    addr_name = guard.hostname.to_owned() + ":" + guard.port.to_string().as_str();
                }
                Err(_) => {}
            }
        }

        self.connection_id = addr_name.to_string();

        if listening {
            self.dispatch_status_message(
                &("Processing connection attempt from \"".to_string()
                    + &self.connection_id
                    + "\" ..."),
            );
        }

        self.connected.store(true, Ordering::SeqCst);
        self.last_missing_cache_warning = 0.0;

        let read_stream = stream
            .try_clone()
            .expect("failed to clone command channel socket");

        self.command_channel_socket = Mutex::new(Some(stream));

        self.command_channel_response_thread = Some(thread::spawn(move || {
            Self::run_command_channel_response_thread(read_stream);
        }));

        self.send_operational_modes();

        // Notify consumers of the connection
        let callback_mutex = self.connection_established_callback.lock();

        if callback_mutex.is_ok() {
            let callback_guard = callback_mutex.unwrap();
            if callback_guard.is_some() {
                let callback = callback_guard.as_ref().unwrap();
                callback();
            }
        }
    }

    ///
    pub fn subscribe(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err("subscriber is not connected; cannot subscribe".into());
        }

        if !self.validated.load(Ordering::SeqCst) {
            return Err("subscriber is not validated; cannot subscribe".into());
        }

        if self.subscribed.load(Ordering::SeqCst) {
            self.unsubscribe();
        }

        self.total_measurements_received.store(0, Ordering::SeqCst);

        let mut parameter_builder = String::new();

        parameter_builder.push_str("throttled=");
        parameter_builder.push_str(&format!("{}", self.subscription.throttled));
        parameter_builder.push_str(";publishInterval=");
        parameter_builder.push_str(&format!("{:.6}", self.subscription.publish_interval));
        parameter_builder.push_str(";includeTime=");
        parameter_builder.push_str(&format!("{}", self.subscription.include_time));
        parameter_builder.push_str(";enableTimeReasonabilityCheck=");
        parameter_builder.push_str(&format!(
            "{}",
            self.subscription.enable_time_reasonability_check
        ));
        parameter_builder.push_str(";lagTime=");
        parameter_builder.push_str(&format!("{:.6}", self.subscription.lag_time));
        parameter_builder.push_str(";leadTime=");
        parameter_builder.push_str(&format!("{:.6}", self.subscription.lead_time));
        parameter_builder.push_str(";useLocalClockAsRealTime=");
        parameter_builder.push_str(&format!(
            "{}",
            self.subscription.use_local_clock_as_real_time
        ));
        parameter_builder.push_str(";processingInterval=");
        parameter_builder.push_str(&format!("{}", self.subscription.processing_interval));
        parameter_builder.push_str(";useMillisecondResolution=");
        parameter_builder.push_str(&format!("{}", self.subscription.use_millisecond_resolution));
        parameter_builder.push_str(";requestNaNValueFilter");
        parameter_builder.push_str(&format!("{}", self.subscription.request_nan_value_filter));
        parameter_builder.push_str(";assemblyInfo={source=");
        parameter_builder.push_str(&self.sttp_source_info);
        parameter_builder.push_str(";version=");
        parameter_builder.push_str(&self.sttp_version_info);
        parameter_builder.push_str(";updatedOn=");
        parameter_builder.push_str(&self.sttp_updated_on_info);
        parameter_builder.push_str("}");

        if self.subscription.filter_expression.len() > 0 {
            parameter_builder.push_str(";filterExpression={");
            parameter_builder.push_str(&self.subscription.filter_expression);
            parameter_builder.push_str("}");
        }

        // if self.subscription.udp_data_channel {
        // TODO: Implement
        // }

        if self.subscription.start_time.len() > 0 {
            parameter_builder.push_str(";startTimeConstraint=");
            parameter_builder.push_str(&self.subscription.start_time);
        }

        if self.subscription.stop_time.len() > 0 {
            parameter_builder.push_str(";stopTimeConstraint=");
            parameter_builder.push_str(&self.subscription.stop_time);
        }

        if self.subscription.constraint_parameters.len() > 0 {
            parameter_builder.push_str(";timeConstraintParameters=");
            parameter_builder.push_str(&self.subscription.constraint_parameters);
        }

        if self.subscription.extra_connection_string_parameters.len() > 0 {
            parameter_builder.push(';');
            parameter_builder.push_str(&self.subscription.extra_connection_string_parameters);
        }

        let parameter_string = parameter_builder.to_string();
        let length = parameter_string.len();
        let mut buffer = vec![0u8; 5 + length];

        buffer[0] = DataPacketFlags::COMPACT.bits();
        buffer[1..5].copy_from_slice(&length.to_be_bytes());
        buffer[5..].copy_from_slice(parameter_string.as_bytes());

        self.send_server_command_with_payload(ServerCommand::Subscribe, buffer.as_slice());

        // Reset TSSC decompressor on successful (re)subscription
        let guard = self.tssc_last_oos_report_mutex.try_lock();

        if guard.is_ok() {
            self.tssc_last_oos_report = Utc::now();
        }

        self.tssc_reset_requested.store(true, Ordering::SeqCst);

        Ok(())
    }

    /// Notifies the `DataPublisher` that a `DataSubscriber` would like to stop receiving streaming data.
    pub fn unsubscribe(&mut self) {
        if !self.connected.load(Ordering::SeqCst) || !self.subscribed.load(Ordering::SeqCst) {
            return;
        }

        self.send_server_command(ServerCommand::Unsubscribe);

        self.disconnecting.store(true, Ordering::SeqCst);

        let result = self.data_channel_socket.lock();

        if result.is_ok() {
            let mut guard = result.unwrap();

            if guard.is_some() {
                let socket = guard.as_mut().unwrap();
                let result = socket.shutdown(Shutdown::Both);

                if result.is_err() {
                    self.dispatch_error_message(&format!(
                        "exception while disconnecting data subscriber UDP data channel: {}",
                        result.err().unwrap()
                    ));
                }
            }
        }

        if self.data_channel_response_thread.is_some() {
            let thread = self.data_channel_response_thread.take().unwrap();
            let result = thread.join();

            if result.is_err() {
                self.dispatch_error_message("failed to join disconnect thread");
            }
        }

        self.disconnecting.store(false, Ordering::SeqCst);
    }

    /// Initiates a `DataSubscriber` disconnect sequence.
    pub fn disconnect(&self) {
        if self.disconnecting.load(Ordering::SeqCst) {
            return;
        }

        // Disconnect method executes shutdown on a separate thread without stopping to prevent
        // issues where user may call disconnect method from a dispatched event thread. Also,
        // user requests to disconnect are not an auto-reconnect attempt and should initiate
        // shutdown of listening socket as well.
        self._disconnect(false, false, true);
    }

    fn _disconnect(&self, join_thread: bool, auto_reconnecting: bool, include_listener: bool) {
        // Check if disconnect thread is running or subscriber has already disconnected
        if self.disconnecting.load(Ordering::SeqCst) {
            if !auto_reconnecting
                && !self.listening.load(Ordering::SeqCst)
                && !self.disconnected.load(Ordering::SeqCst)
            {
                match self.connector.lock() {
                    Ok(guard) => {
                        guard.cancel();
                    }
                    Err(_) => {}
                }
            }

            if join_thread && !self.disconnected.load(Ordering::SeqCst) {
                let result = self.disconnect_thread_mutex.lock();

                if result.is_ok() {
                    let mut guard = result.unwrap();

                    if guard.is_some() {
                        let thread = guard.take().unwrap();
                        let result = thread.join();

                        if result.is_err() {
                            self.dispatch_error_message("failed to join disconnect thread");
                        }
                    }
                }
            }

            return;
        }

        // Notify running threads that the subscriber is disconnecting, i.e., disconnect thread is active
        self.disconnecting.store(true, Ordering::SeqCst);
        self.connected.store(false, Ordering::SeqCst);
        self.validated.store(false, Ordering::SeqCst);

        if include_listener {
            self.listening.store(false, Ordering::SeqCst);
        }

        self.subscribed.store(false, Ordering::SeqCst);

        // TODO: Implement
        // let disconnect_thread = thread::spawn(move || {
        //     self.run_disconnect_thread(&auto_reconnecting, &include_listener)
        // });

        // self.disconnect_thread_mutex = Mutex::new(Some(disconnect_thread));

        // let result = self.disconnect_thread_mutex.lock();

        // if result.is_ok() {
        //     let mut guard = result.unwrap();

        //     if guard.is_some() {
        //         let thread = guard.take().unwrap();
        //         let result = thread.join();

        //         if result.is_err() {
        //             self.dispatch_error_message("failed to join disconnect thread");
        //         }
        //     }
        // }
    }

    fn run_disconnect_thread(&self, auto_reconnecting: &bool, include_listener: &bool) {}

    fn run_command_channel_response_thread(mut stream: TcpStream) {
        // TODO: Implement properly
        loop
        /* self.connected.load(Ordering::Relaxed) */
        {
            // Read data from the server
            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    let response = from_utf8(&buffer[0..bytes_read]).unwrap();
                    println!("Received response: {}", response);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
    }

    fn send_operational_modes(&self) {}

    fn send_server_command(&mut self, command: ServerCommand) {
        self.send_server_command_with_payload(command, &[]);
    }

    fn send_server_command_with_message(&mut self, command: ServerCommand, message: &str) {
        self.send_server_command_with_payload(command, message.as_bytes());
    }

    fn send_server_command_with_payload(&mut self, command: ServerCommand, data: &[u8]) {
        if !self.connected.load(Ordering::SeqCst) {
            return;
        }

        let packet_size = data.len() + 1;
        let command_buffer_size = 5 + packet_size + PAYLOAD_HEADER_SIZE;
        let mut buffer = vec![0u8; command_buffer_size];

        // Insert packet size
        buffer[..4].copy_from_slice(&packet_size.to_be_bytes());

        // Insert command code
        buffer[4] = command as u8;

        if data.len() > 0 {
            buffer[5..].copy_from_slice(data);
        }

        if command == ServerCommand::MetadataRefresh {
            // Track start time of metadata request to calculate round-trip receive time
            self.metadata_requested = Utc::now();
        }

        let result = self.command_channel_socket.lock();

        if result.is_err() {
            self.dispatch_error_message(&format!(
                "failed to send server command - disconnecting: {}",
                result.err().unwrap()
            ));
            self.dispatch_connection_terminated();

            return;
        }

        let guard = result.unwrap();

        if guard.is_none() {
            return;
        }

        let mut socket = guard.as_ref().unwrap();
        let result = socket.write_all(&buffer);

        if result.is_err() {
            // Write error, connection may have been closed by peer; terminate connection
            self.dispatch_error_message(&format!(
                "failed to send server command - disconnecting: {}",
                result.err().unwrap()
            ));
            self.dispatch_connection_terminated()
        }
    }

    fn dispatch_status_message(&self, message: &str) {
        let result = self.status_message_callback.lock();
        if result.is_ok() {
            let guard = result.unwrap();
            if guard.is_some() {
                guard.as_ref().unwrap()(message);
            }
        }
    }

    fn dispatch_error_message(&self, message: &str) {
        let result = self.error_message_callback.lock();
        if result.is_ok() {
            let guard = result.unwrap();
            if guard.is_some() {
                guard.as_ref().unwrap()(message);
            }
        }
    }

    fn dispatch_connection_terminated(&self) {
        let result = self.connection_termination_thread_mutex.lock();

        if result.is_ok() {
            let guard = result.unwrap();
            if guard.is_some() {
                return;
            }

            // TODO: Implement
            // self.connection_termination_thread_mutex
            //     .lock()
            //     .unwrap()
            //     .replace(thread::spawn(move || {
            //         //self._disconnect(false, true);
            //         self.connection_termination_thread_mutex = Mutex::new(None);
            //     }));
        }
    }
}
