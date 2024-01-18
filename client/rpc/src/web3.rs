// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020-2022 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
use ethereum_types::H256;
use jsonrpsee::core::RpcResult;
use target_info::Target;
// Substrate
use sp_core::keccak_256;
// Frontier
use fc_rpc_core::{types::Bytes, Web3ApiServer};

/// Web3 API implementation.
pub struct Web3 {
	client_version: String,
}

impl Web3 {
	pub fn new(client_version: &str) -> Self {
		Self {
			client_version: format!(
				"bifrost-node/v{client_version}/{os}-{arch}/rustc{rustc_version}",
				os = Target::os(),
				arch = Target::arch(),
				rustc_version = env!("RUSTC_VERSION"),
			),
		}
	}
}

impl Web3ApiServer for Web3 {
	fn client_version(&self) -> RpcResult<String> {
		Ok(self.client_version.clone())
	}

	fn sha3(&self, input: Bytes) -> RpcResult<H256> {
		Ok(H256::from(keccak_256(&input.into_vec())))
	}
}
