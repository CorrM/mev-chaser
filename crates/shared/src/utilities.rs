use tokio::runtime::{Handle, Runtime};

pub use pool_path_finder::*;

pub mod pool_path_finder;

pub fn block_on<F: core::future::Future>(mut runtime: Option<Runtime>, f: F) -> F::Output {
    if runtime.is_none() {
        runtime = Handle::try_current().is_err().then(|| Runtime::new().unwrap());
    }

    match runtime {
        Some(runtime) => runtime.block_on(f),
        None => tokio::task::block_in_place(|| Handle::current().block_on(f)),
    }
}
