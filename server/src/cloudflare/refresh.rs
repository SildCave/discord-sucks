use std::{sync::Arc, time::{Duration, Instant}};

use tokio::sync::RwLock;
use tracing::{error, info};

use super::CloudflareIpAddresses;



pub async fn cloudflare_ip_refresh_cron_job(
    cloudflare_ip_addresses: Arc<RwLock<CloudflareIpAddresses>>,
    interval: Duration,
) {
    tokio::time::sleep(
        Duration::from_secs(10)
    ).await;
    loop {
        let cloudflare_ip_addresses = cloudflare_ip_addresses.clone();
        let duration = Instant::now();
        let result = CloudflareIpAddresses::new_from_cloudflare_api().await;
        match result {
            Ok(new_cloudflare_ip_addresses) => {
                let mut cloudflare_ip_addresses = cloudflare_ip_addresses.write().await;
                *cloudflare_ip_addresses = new_cloudflare_ip_addresses;
                drop(cloudflare_ip_addresses);
                info!("Refreshed Cloudflare IP addresses, took: {:?}", duration.elapsed());
            }
            Err(e) => {
                error!("Failed to refresh Cloudflare IP addresses: {}", e);
            }
        }
        tokio::time::sleep(interval).await;
    }
}
