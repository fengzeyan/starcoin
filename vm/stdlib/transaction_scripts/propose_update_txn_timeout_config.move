script {
use 0x1::TransactionTimeoutConfig;
use 0x1::OnChainConfigDao;
use 0x1::STC;

fun propose_update_txn_timeout_config(account: &signer,
    duration_seconds: u64,
    exec_delay: u64) {
    let txn_timeout_config = TransactionTimeoutConfig::new_transaction_timeout_config(duration_seconds);
    OnChainConfigDao::propose_update<STC::STC, TransactionTimeoutConfig::TransactionTimeoutConfig>(account, txn_timeout_config, exec_delay);
}

spec fun propose_update_txn_timeout_config {
    pragma verify = false;
}
}
