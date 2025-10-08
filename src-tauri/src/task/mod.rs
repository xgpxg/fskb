use crate::server;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn start() -> anyhow::Result<()> {
    let sched = JobScheduler::new().await?;

    let print_pool_status = Job::new_async("every 1 minutes", |_, _| {
        Box::pin(server::mcp::sync::sync_all_mcp_servers())
    })?;
    sched.add(print_pool_status).await?;

    sched.start().await?;

    Ok(())
}
