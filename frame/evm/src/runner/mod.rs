// This file is part of Frontier.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod meter;
pub mod stack;

use crate::{Config, Weight};
use alloc::vec::Vec;
use ethereum::AuthorizationList;
use fp_evm::{CallInfo, CreateInfo};
use sp_core::{H160, H256, U256};

#[derive(Debug)]
pub struct RunnerError<E: Into<sp_runtime::DispatchError>> {
	pub error: E,
	pub weight: Weight,
}

pub trait Runner<T: Config> {
	type Error: Into<sp_runtime::DispatchError>;

	fn validate(
		source: H160,
		target: Option<H160>,
		input: Vec<u8>,
		value: U256,
		gas_limit: u64,
		max_fee_per_gas: Option<U256>,
		max_priority_fee_per_gas: Option<U256>,
		nonce: Option<U256>,
		access_list: Vec<(H160, Vec<H256>)>,
		authorization_list: Vec<(U256, H160, U256, Option<H160>)>,
		is_transactional: bool,
		weight_limit: Option<Weight>,
		proof_size_base_cost: Option<u64>,
		evm_config: &evm::Config,
	) -> Result<(), RunnerError<Self::Error>>;

	fn call(
		source: H160,
		target: H160,
		input: Vec<u8>,
		value: U256,
		gas_limit: u64,
		max_fee_per_gas: Option<U256>,
		max_priority_fee_per_gas: Option<U256>,
		nonce: Option<U256>,
		access_list: Vec<(H160, Vec<H256>)>,
		authorization_list: AuthorizationList,
		is_transactional: bool,
		validate: bool,
		weight_limit: Option<Weight>,
		proof_size_base_cost: Option<u64>,
		config: &evm::Config,
	) -> Result<CallInfo, RunnerError<Self::Error>>;

	fn create(
		source: H160,
		init: Vec<u8>,
		value: U256,
		gas_limit: u64,
		max_fee_per_gas: Option<U256>,
		max_priority_fee_per_gas: Option<U256>,
		nonce: Option<U256>,
		access_list: Vec<(H160, Vec<H256>)>,
		authorization_list: AuthorizationList,
		is_transactional: bool,
		validate: bool,
		weight_limit: Option<Weight>,
		proof_size_base_cost: Option<u64>,
		config: &evm::Config,
	) -> Result<CreateInfo, RunnerError<Self::Error>>;

	fn create2(
		source: H160,
		init: Vec<u8>,
		salt: H256,
		value: U256,
		gas_limit: u64,
		max_fee_per_gas: Option<U256>,
		max_priority_fee_per_gas: Option<U256>,
		nonce: Option<U256>,
		access_list: Vec<(H160, Vec<H256>)>,
		authorization_list: AuthorizationList,
		is_transactional: bool,
		validate: bool,
		weight_limit: Option<Weight>,
		proof_size_base_cost: Option<u64>,
		config: &evm::Config,
	) -> Result<CreateInfo, RunnerError<Self::Error>>;

	/// Execute an EVM call bypassing reentrancy protection.
	///
	/// This function is specifically designed for fee payment operations
	/// (Oracle price queries, ERC20 transfers) that need to be executed
	/// during the `OnChargeEVMTransaction::withdraw_fee` phase.
	///
	/// # Security
	/// - This bypasses reentrancy protection, use with caution
	/// - Should only be called from trusted fee payment pallets
	/// - Limited to simple view calls or token transfers
	///
	/// # Arguments
	/// * `source` - The sender address (typically Treasury)
	/// * `target` - The contract to call (Oracle or ERC20)
	/// * `input` - Encoded function call data
	/// * `gas_limit` - Maximum gas for the call
	/// * `is_transactional` - Whether to persist state changes
	/// * `config` - EVM configuration
	fn call_bypassing_reentrancy(
		source: H160,
		target: H160,
		input: Vec<u8>,
		gas_limit: u64,
		is_transactional: bool,
		config: &evm::Config,
	) -> Result<CallInfo, RunnerError<Self::Error>>;

	/// Execute a read-only EVM call without incrementing nonce.
	///
	/// This function is designed for view/pure function calls (e.g., Oracle price queries)
	/// that should not modify any state, including the caller's nonce.
	///
	/// Unlike `call_bypassing_reentrancy` which uses `transact_call` (incrementing nonce),
	/// this uses the internal `call` mechanism that doesn't touch nonce.
	///
	/// # Arguments
	/// * `source` - The caller address (used for context, nonce not modified)
	/// * `target` - The contract to call
	/// * `input` - Encoded function call data
	/// * `gas_limit` - Maximum gas for the call
	/// * `config` - EVM configuration
	///
	/// # Returns
	/// * `Ok(CallInfo)` - Call result with return data
	/// * `Err(RunnerError)` - If the call fails
	fn view_call(
		source: H160,
		target: H160,
		input: Vec<u8>,
		gas_limit: u64,
		config: &evm::Config,
	) -> Result<CallInfo, RunnerError<Self::Error>>;
}
