use ethereum::{AccessListItem, AuthorizationList};
use ethereum_types::{H160, H256, U256};
use fc_evm_tracing::types::{block, single};
use fc_rpc_core::types::Bytes;
use fc_rpc_core_types::RequestBlockId;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use serde::Deserialize;

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceParams {
	pub disable_storage: Option<bool>,
	pub disable_memory: Option<bool>,
	pub disable_stack: Option<bool>,
	/// Javascript tracer (we just check if it's Blockscout tracer string)
	pub tracer: Option<String>,
	pub tracer_config: Option<single::TraceCallConfig>,
	pub timeout: Option<String>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceCallParams {
	/// Sender
	pub from: Option<H160>,
	/// Recipient
	pub to: H160,
	/// Gas Price, legacy.
	pub gas_price: Option<U256>,
	/// Max BaseFeePerGas the user is willing to pay.
	pub max_fee_per_gas: Option<U256>,
	/// The miner's tip.
	pub max_priority_fee_per_gas: Option<U256>,
	/// Gas
	pub gas: Option<U256>,
	/// Value of transaction in wei
	pub value: Option<U256>,
	/// Additional data sent with transaction
	pub data: Option<Bytes>,
	/// Nonce
	pub nonce: Option<U256>,
	/// EIP-2930 access list
	pub access_list: Option<Vec<AccessListItem>>,
	/// EIP-7702 authorization list
	pub authorization_list: Option<AuthorizationList>,
	/// EIP-2718 type
	#[serde(rename = "type")]
	pub transaction_type: Option<U256>,
}

#[rpc(server)]
#[jsonrpsee::core::async_trait]
pub trait Debug {
	#[method(name = "debug_traceTransaction")]
	async fn trace_transaction(
		&self,
		transaction_hash: H256,
		params: Option<TraceParams>,
	) -> RpcResult<single::TransactionTrace>;
	#[method(name = "debug_traceCall")]
	async fn trace_call(
		&self,
		call_params: TraceCallParams,
		id: RequestBlockId,
		params: Option<TraceParams>,
	) -> RpcResult<single::TransactionTrace>;
	#[method(name = "debug_traceBlockByNumber", aliases = ["debug_traceBlockByHash"])]
	async fn trace_block(
		&self,
		id: RequestBlockId,
		params: Option<TraceParams>,
	) -> RpcResult<Vec<block::BlockTransactionTrace>>;
}
