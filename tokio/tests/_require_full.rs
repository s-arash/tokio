#![cfg(not(any(feature = "full", feature = "full-sgx")))]
compile_error!("run main Tokio tests with `--features full`");
