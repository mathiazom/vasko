use std::future::Future;
use tokio::time::{sleep, Duration};

pub async fn schedule_task<F>(duration: Duration, task: F)
where
    F: Future<Output = ()>,
    F: Send + 'static,
{
    if duration.is_zero() {
        return;
    }
    sleep(duration).await;
    task.await;
}
