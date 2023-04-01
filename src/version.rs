//******************************************************************************************************
//  version.rs - Gbtc
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

/// Defines STTP API Implementation Version ID Constants.
#[derive(Clone, Copy)]
pub struct Version;

impl Version {
    /// Defines the STTP library API title used for data subscriber identification.
    pub const STTP_SOURCE: &str = "STTP Rust Library";

    /// Defines the STTP library API version used for data subscriber identification.
    /// Note: This is not the STTP protocol version, but the version of the STTP library API.
    pub const STTP_VERSION: &str = "0.1.0";

    /// Defines when the STTP library API was last updated used for data subscriber identification.
    pub const STTP_UPDATED_ON: &str = "2023-04-01";
}
