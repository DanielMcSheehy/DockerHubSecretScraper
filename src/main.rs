mod docker;
use tokio_cron_scheduler::{Job, JobScheduler};

static DOCKER_API_URL: &str = "https://hub.docker.com/api/content/v1/products/search?page_size=100&q=%2B&source=community&type=image%2Cbundle&sort=updated_at";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sched = JobScheduler::new();

    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();

    // cron every 30 seconds
    let four_s_job_async = Job::new_async("1/4 * * * * *", |_uuid, _l| Box::pin(async move {
             let secrets = docker::process(DOCKER_API_URL).await;
             docker::export_secrets(secrets.unwrap()).await.unwrap();
      })).unwrap();

    sched.add(four_s_job_async).unwrap();

    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    })).unwrap();

    sched.start().await.unwrap();
    
    Ok(())
}
