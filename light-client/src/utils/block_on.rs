use std::{future::Future, time::Duration};

use crate::components::io::IoError;

/// Run a future to completion on a new thread, with the given timeout.
///
/// This function will block the caller until the given future has completed.
pub fn block_on<F>(timeout: Option<Duration>, f: F) -> Result<F::Output, IoError>
where
    F: Future + Send + 'static,
    F::Output: Send,
{
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| IoError::Runtime)?;

        if let Some(timeout) = timeout {
            rt.block_on(tokio::time::timeout(timeout, f))
                .map_err(|_| IoError::Timeout(timeout))
        } else {
            Ok(rt.block_on(f))
        }
    })
    .join()
    .unwrap()
}
