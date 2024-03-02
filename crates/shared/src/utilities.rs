use tokio::runtime::Runtime;

pub use pool_path_finder::*;

pub mod pool_path_finder;

pub fn block_on<F: core::future::Future>(mut runtime: Option<Runtime>, f: F) -> F::Output {
    if runtime.is_none() {
        runtime = tokio::runtime::Handle::try_current()
            .is_err()
            .then(|| Runtime::new().unwrap());
    }

    match runtime {
        Some(runtime) => runtime.block_on(f),
        None => futures::executor::block_on(f),
    }
}
