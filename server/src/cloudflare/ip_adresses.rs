use std::{
    collections::HashSet,
    net::IpAddr
};

use ipnetwork::Ipv4Network;
use tracing::{error, info};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CloudflareIpAddresses {
    addresses: HashSet<IpAddr>,
}

impl CloudflareIpAddresses {
    pub fn new(
        ranges: Vec<String>,
    ) -> Self {
        let mut ip_addresses = HashSet::new();
        for range in ranges {
            let network: Ipv4Network = range.parse().unwrap();
            for ip in network.iter() {
                ip_addresses.insert(IpAddr::V4(ip));
            }
        }
        info!("Added {} Cloudflare IP addresses", ip_addresses.len());
        Self {
            addresses: ip_addresses,
        }
    }

    pub async fn new_from_cloudflare_api() -> Result<Self> {
        let url = "https://www.cloudflare.com/ips-v4/";
        let response = reqwest::get(url).await;
        match response {
            Ok(ref response) => {
                if !response.status().is_success() {
                    error!("Failed to fetch Cloudflare IP addresses from Cloudflare API: {}", response.status());
                } else {
                    info!("Fetched Cloudflare IP addresses from Cloudflare API");
                }
            }
            Err(ref e) => {
                error!("Failed to fetch Cloudflare IP addresses from Cloudflare API: {}", e);
            }
        };
        let ranges: Vec<String> = response?.text().await?.lines().map(|s| s.to_string()).collect();
        Ok(Self::new(ranges))
    }

    pub fn is_cloudflare_ip(&self, ip: impl Into<IpAddr>) -> bool {
        self.addresses.contains(&ip.into())
    }

}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use std::net::Ipv4Addr;
    #[test]
    fn test_ipv4_generation() {
        let ranges = vec!["173.245.48.0/20".to_string(), "141.101.64.0/18".to_string(), "104.16.0.0/13".to_string()];
        let cloudflare_ips = CloudflareIpAddresses::new(ranges);
        assert_eq!(cloudflare_ips.addresses.len(), 	4096 + 16384 + 524288);
    }

    #[tokio::test]
    async fn test_cloudflare_api() {
        let cloudflare_ips = CloudflareIpAddresses::new_from_cloudflare_api().await.unwrap();
        assert!(cloudflare_ips.addresses.len() > 0);
    }

    #[test]
    fn test_is_cloudflare_ip() {
        let ranges = vec!["173.245.48.0/20".to_string(), "141.101.64.0/18".to_string(), "104.16.0.0/13".to_string()];
        let cloudflare_ips = CloudflareIpAddresses::new(ranges);
        assert!(cloudflare_ips.is_cloudflare_ip(IpAddr::V4(Ipv4Addr::new(173, 245, 48, 0))));
    }


    // WARNING: This benchmark takes up ~16gb of ram
    #[bench]
    fn bench_is_cloudflare_ip(b: &mut test::Bencher) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let cloudflare_ips = runtime.block_on(CloudflareIpAddresses::new_from_cloudflare_api()).unwrap();
        let test_ips: Ipv4Network = "0.0.0.0/0".parse().unwrap();
        let mut test_ips = test_ips.iter().collect::<Vec<Ipv4Addr>>();

        b.iter(|| {
            cloudflare_ips.is_cloudflare_ip(test_ips.pop().unwrap());
        });
    }

}