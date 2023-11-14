use ethers::{
    core::types::{GethDebugTracingOptions, H256},
    providers::{Http, Middleware, Provider},
    types::{GethDebugBuiltInTracerType, GethDebugTracerType},
};
use eyre::Result;
use std::str::FromStr;

/// use `debug_traceTransaction` to fetch traces
/// requires, a valid endpoint in `RPC_URL` env var that supports `debug_traceTransaction`
#[tokio::main]
async fn main() -> Result<()> {
    // if let Ok(url) = std::env::var("RPC_URL") {
        let url = "https://sepolia-rollup.arbitrum.io/rpc";
        let client = Provider::<Http>::try_from(url)?;
        let tx_hash = "0x7d4cb959fff0c0b0e305650f204ea197c342ae9728d90bcc5936943e181e6e8a";
        let h: H256 = H256::from_str(tx_hash)?;

        // default tracer
        let options = GethDebugTracingOptions::default();
        let traces = client.get_transaction(h).await?;
        let get_transaction_receipt = client.get_transaction_receipt(h).await?;
        println!("{traces:?}");
        println!("{get_transaction_receipt:?}");

        // call tracer
        let options = GethDebugTracingOptions {
            disable_storage: Some(true),
            enable_memory: Some(false),
            tracer: Some(GethDebugTracerType::BuiltInTracer(
                GethDebugBuiltInTracerType::CallTracer,
            )),
            ..Default::default()
        };
        let traces = client.debug_trace_transaction(h, options).await?;
        println!("{traces:?}");

        // js tracer
        let options = GethDebugTracingOptions {
                disable_storage: Some(true),
                enable_memory: Some(false),
                tracer: Some(GethDebugTracerType::JsTracer(String::from("{data: [], fault: function(log) {}, step: function(log) { if(log.op.toString() == \"DELEGATECALL\") this.data.push(log.stack.peek(0)); }, result: function() { return this.data; }}"))),
                ..Default::default()
            };
        let traces = client.debug_trace_transaction(h, options).await?;
        println!("{traces:?}");
    // }

    Ok(())
}
