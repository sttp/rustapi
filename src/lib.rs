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

#![doc = include_str!("../README.md")]
#![deny(
    dead_code,
    arithmetic_overflow,
    invalid_type_param_default,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    overflowing_literals,
    patterns_in_fns_without_body,
    pub_use_of_private_extern_crate,
    unknown_crate_types,
    order_dependent_trait_objects,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    late_bound_lifetime_arguments,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    private_in_public,
    stable_features,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unused_comparisons,
    unreachable_pub,
    anonymous_parameters,
    missing_copy_implementations,
    //missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    clippy::all
    )]
#![forbid(
    unsafe_code,
    rustdoc::broken_intra_doc_links,
    while_true,
    bare_trait_objects
)]
#![cfg_attr(test, allow(dead_code))]

// Expose local submodules as public passthroughs of sttp module
// for better organization and ease of use of public API

mod config;
pub use crate::config::Config; // >> sttp::Config

mod settings;
pub use crate::settings::Settings; // >> sttp::Settings

mod ticks;
pub use crate::ticks::Ticks; // >> sttp::Ticks

/// Represents data functionality of the STTP library.
pub mod data;

/// Represents transport functionality of the STTP library.
pub mod transport;

// use std::sync::Arc;
// use std::time::SystemTime;

/// Represents an STTP data subscriber.
///
/// The `Subscriber` exists as a simplified implementation of the `DataSubscriber` found
/// in the transport module. The `Subscriber` is intended to simplify common uses of
/// STTP data reception and maintains an internal instance of the `DataSubscriber` for
/// subscription based functionality.
#[derive(Clone, Copy)]
pub struct Subscriber {
    // // Configuration reference
    // config: Arc<Config>,

    // // DataSubscriber reference
    // //ds: Arc<transport::DataSubscriber>,

    // // Callback references
    // status_message_logger: Option<Box<dyn Fn(String) + Send + Sync>>,
    // error_message_logger: Option<Box<dyn Fn(String) + Send + Sync>>,
    // //metadata_receiver: Option<Box<dyn Fn(data::DataSet) + Send + Sync>>,
    // data_start_time_receiver: Option<Box<dyn Fn(SystemTime) + Send + Sync>>,
    // configuration_changed_receiver: Option<Box<dyn Fn() + Send + Sync>>,
    // historical__complete_receiver: Option<Box<dyn Fn() + Send + Sync>>,
    // connection_established_receiver: Option<Box<dyn Fn() + Send + Sync>>,
}
