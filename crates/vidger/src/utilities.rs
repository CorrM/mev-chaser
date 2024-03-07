use tokio::runtime::Handle;

#[inline(always)]
pub fn block_on<F: core::future::Future>(f: F) -> F::Output {
    let runtime: Option<Handle> = Handle::try_current().ok();

    /*
    let rt = Handle::current();
    rt.spawn_blocking(func)
    */

    match runtime {
        Some(runtime) => tokio::task::block_in_place(|| runtime.block_on(f)),
        None => futures::executor::block_on(f),
    }
}
