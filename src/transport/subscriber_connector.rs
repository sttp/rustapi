//******************************************************************************************************
//  subscriber_connector.rs - Gbtc
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

use crate::transport::ConnectStatus;
use crate::transport::DataSubscriber;
//use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// Represents a connector that will establish or automatically reestablish a connection
/// from a `DataSubscriber` to a `DataPublisher`.
//#[derive(Debug)]
pub struct SubscriberConnector {
    /// Called when an error message should be logged.
    pub error_message_callback: Box<dyn Fn(String) + Send + Sync>,

    /// Called when SubscriberConnector attempts to reconnect.
    pub reconnect_callback: Box<dyn Fn(Arc<DataSubscriber>) + Send + Sync>,

    /// Defines the `DataPublisher` DNS name or IP.
    pub hostname: String,

    /// Defines the TCP/IP listening port of the DataPublisher.
    pub port: u16,

    /// Defines the maximum number of times to retry a connection.
    /// Set value to -1 to retry infinitely.
    pub max_retries: i32,

    /// RetryInterval defines the base retry interval, in milliseconds. Retries will
    /// exponentially back-off starting from this interval.
    pub retry_interval: i32,

    /// Defines the maximum retry interval, in milliseconds.
    pub max_retry_interval: i32,

    /// Defines flag that determines if connections should be
    /// automatically reattempted.
    pub auto_reconnect: bool,

    connect_attempt: i32,
    connection_refused: AtomicBool,
    cancel: AtomicBool,
    reconnect_thread_mutex: Mutex<Option<JoinHandle<()>>>,
    wait_timer_mutex: Mutex<Option<JoinHandle<()>>>,

    assigning_handler_mutex: RwLock<()>,
}

// impl DerefMut for SubscriberConnector {
//     fn deref_mut(&mut self) -> &mut SubscriberConnector {
//         self
//     }
// }

// impl Deref for SubscriberConnector {
//     type Target = SubscriberConnector;

//     fn deref(&self) -> &SubscriberConnector {
//         &self
//     }
// }

impl SubscriberConnector {
    /// Creates a new `SubscriberConnector`
    pub fn new() -> Self {
        Self {
            error_message_callback: Box::new(|_| {}),
            reconnect_callback: Box::new(|_| {}),
            hostname: "localhost".to_string(),
            port: 6162,
            max_retries: -1,
            retry_interval: 1000,
            max_retry_interval: 60000,
            auto_reconnect: true,
            connect_attempt: 0,
            connection_refused: AtomicBool::new(false),
            cancel: AtomicBool::new(false),
            reconnect_thread_mutex: Mutex::new(None),
            wait_timer_mutex: Mutex::new(None),
            assigning_handler_mutex: RwLock::new(()),
        }
    }

    fn auto_reconnect(&self, ds: Arc<DataSubscriber>) {
        if self.cancel.load(Ordering::SeqCst) || ds.disposing.load(Ordering::SeqCst) {
            return;
        }

        // Make sure to wait on any running reconnect to complete...
        let mut reconnect_thread_guard = self.reconnect_thread_mutex.lock().unwrap();

        if let Some(reconnect_thread) = reconnect_thread_guard.take() {
            drop(reconnect_thread_guard);
            reconnect_thread.join().unwrap();
        }

        let sc = Arc::clone(&ds.connector);
        let reconnect_thread = thread::spawn({
            move || {
                // Reset connection attempt counter if last attempt was not refused
                if !sc.connection_refused.load(Ordering::SeqCst) {
                    //sc.reset_connection();
                }

                if sc.max_retries != -1 && sc.connect_attempt >= sc.max_retries {
                    (sc.error_message_callback)(
                        "Maximum connection retries attempted. Auto-reconnect canceled."
                            .to_string(),
                    );
                    return;
                }

                sc.wait_for_retry();

                if sc.cancel.load(Ordering::SeqCst) || ds.disposing.load(Ordering::SeqCst) {
                    return;
                }

                if sc.connect(ds.clone(), true) == ConnectStatus::Canceled {
                    return;
                }

                // Notify the user that reconnect attempt was completed.
                sc.begin_callback_sync();

                if !sc.cancel.load(Ordering::SeqCst) {
                    (sc.reconnect_callback)(ds.clone());
                }

                sc.end_callback_sync();
            }
        });

        let mut reconnect_thread_guard = self.reconnect_thread_mutex.lock().unwrap();
        *reconnect_thread_guard = Some(reconnect_thread);
    }

    fn reset_connection(&mut self) {
        self.connect_attempt = 0;
        self.connection_refused.store(false, Ordering::SeqCst);
    }

    // fn wait_for_retry(&self) {
    //     let mut wait_timer_mutex = self.wait_timer_mutex.lock().unwrap();
    //     let mut wait_timer = wait_timer_mutex.take().unwrap();

    //     if wait_timer.is_none() {
    //         *wait_timer = Some(std::time::Instant::now());
    //     }

    //     let elapsed = wait_timer.unwrap().elapsed().as_millis() as i32;

    //     if elapsed < self.retry_interval {
    //         thread::sleep(Duration::from_millis(
    //             (self.retry_interval - elapsed) as u64,
    //         ));
    //     }

    //     *wait_timer_mutex = Some(wait_timer);
    // }

    fn wait_for_retry(&self) {
        // Apply exponential back-off algorithm for retry attempt delays
        let exponent: i32 = if self.connect_attempt > 13 {
            12
        } else {
            self.connect_attempt - 1
        };

        let mut retry_interval = 0;

        if self.connect_attempt > 0 {
            retry_interval = self.retry_interval * 2i32.pow(exponent as u32);
        }

        retry_interval = i32::min(retry_interval, self.max_retry_interval);

        // Notify the user that we are attempting to reconnect.
        let message = if self.connect_attempt > 0 {
            format!(
                "Connection attempt {}, to \"{}:{}\" was terminated. Attempting to reconnect in {:.2} seconds...",
                self.connect_attempt + 1,
                self.hostname,
                self.port,
                retry_interval
            )
        } else {
            format!(
                "Connection to \"{}:{}\" was terminated. Attempting to reconnect...",
                self.hostname, self.port
            )
        };

        //self.dispatch_error_message(&message);

        // Lock the wait timer mutex and update the value
        {
            let mut wait_timer_guard = self.wait_timer_mutex.lock().unwrap();
            *wait_timer_guard = Some(thread::spawn(move || {
                thread::sleep(Duration::from_secs(retry_interval as u64));
            }));
        }

        // Wait for the timer to complete
        if let Some(handle) = self.wait_timer_mutex.lock().unwrap().take() {
            handle.join().unwrap();
        }
    }

    fn begin_callback_sync(&self) {
        let mut assigning_handler_mutex = self.assigning_handler_mutex.write().unwrap();
        let mut reconnect_thread_mutex = self.reconnect_thread_mutex.lock().unwrap();

        if let Some(reconnect_thread) = reconnect_thread_mutex.take() {
            drop(reconnect_thread_mutex);
            reconnect_thread.join().unwrap();
        }

        drop(assigning_handler_mutex);
    }

    fn end_callback_sync(&self) {
        let mut assigning_handler_mutex = self.assigning_handler_mutex.write().unwrap();
        let mut reconnect_thread_mutex = self.reconnect_thread_mutex.lock().unwrap();

        if let Some(reconnect_thread) = reconnect_thread_mutex.take() {
            drop(reconnect_thread_mutex);
            reconnect_thread.join().unwrap();
        }

        drop(assigning_handler_mutex);
    }

    fn connect(&self, ds: Arc<DataSubscriber>, auto_reconnect: bool) -> ConnectStatus {
        //     if self.cancel.load(Ordering::SeqCst) || ds.disposing.load(Ordering::SeqCst) {
        //         return ConnectStatus::Canceled;
        //     }

        //     let mut wait_timer_mutex = self.wait_timer_mutex.lock().unwrap();
        //     let mut wait_timer = wait_timer_mutex.take().unwrap();

        //     if wait_timer.is_none() {
        //         *wait_timer = Some(std::time::Instant::now());
        //     }

        //     let elapsed = wait_timer.unwrap().elapsed().as_millis() as i32;

        //     if elapsed < self.retry_interval {
        //         thread::sleep(Duration::from_millis(
        //             (self.retry_interval - elapsed) as u64,
        //         ));
        //     }

        //     *wait_timer_mutex = Some(wait_timer);

        //     if self.cancel.load(Ordering::SeqCst) || ds.disposing.load(Ordering::SeqCst) {
        //         return ConnectStatus::Canceled;
        //     }

        //     let mut reconnect_thread_mutex = self.reconnect_thread_mutex.lock().unwrap();
        //     if let Some(reconnect_thread) = reconnect_thread_mutex.take() {
        //         drop(reconnect_thread_mutex);
        //         reconnect_thread.join().unwrap();
        //     }

        //     let mut wait_timer_mutex = self.wait_timer_mutex.lock().unwrap();
        //     let mut wait_timer = wait_timer_mutex.take().unwrap();

        //     if wait_timer.is_none() {
        //         *wait_timer = Some(std::time::Instant::now());
        //     }

        //     let elapsed = wait_timer.unwrap().elapsed().as_millis() as i32;

        //     if elapsed < self.retry_interval {
        //         thread::sleep(Duration::from_millis(
        //             (self.retry_interval - elapsed) as u64,
        //         ));
        //     }

        //     *wait_timer_mutex = Some(wait_timer);

        //     if self.cancel.load(Ordering::SeqCst) || ds.disposing.load(Ordering::SeqCst) {
        //         return ConnectStatus::Canceled;
        //     }

        //     let mut reconnect_thread_mutex = self.reconnect_thread_mutex.lock().unwrap();
        //     if let Some(reconnect_thread) = reconnect_thread_mutex.take() {
        //         drop(reconnect_thread_mutex);
        //         reconnect_thread.join().unwrap();
        //     }

        //     let mut wait_timer_mutex = self.wait_timer_mutex.lock().unwrap();
        //     let mut wait_timer = wait_timer_mutex.take().unwrap();

        //     if wait_timer.is_none() {
        //         *wait_timer = Some(std::time::Instant::now());
        //     }

        ConnectStatus::Success
    }
}
