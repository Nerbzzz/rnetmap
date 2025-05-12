use std::collections::BTreeSet;
use std::net::IpAddr;
use ipnetwork::IpNetwork;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseListError {
    #[error("invalid number: {0}")]
    InvalidNumber(String),
    #[error("invalid range: {0}")]
    InvalidRange(String)
}

#[derive(Debug, Error)]
pub enum ResolveTargetError {
    #[error("invalid IP address: {0}")]
    InvalidIp(#[from] std::net::AddrParseError),
    #[error("invalid CIDR notation: {0}")]
    InvalidCidr(#[from] ipnetwork::IpNetworkError),
    #[error("invalid range format: {0}")]
    InvalidRange(String)
}

/// Parses a comma-separated list of ports and port ranges into a Vec<u16>.
///
/// Supports:
/// - All ports: `*`  → [0, 1, ..., 65535] 
/// - Single: `80` → [80]
/// - Ranges: `8000-8005` → [8000, 8001, ..., 8005]
/// - Mixed: `22,80,8000-8005`
pub fn parse_ports(list: &str) -> Result<Vec<u16>, ParseListError> {
    let mut ports = BTreeSet::new();
    let list = list.trim();

    if list == "*" {
        ports.extend(1..=65535);
        return Ok(ports.into_iter().collect());
    }

    for token in list.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        let mut parts = token.splitn(2, '-');
        let start: u16 = parts.next().unwrap().parse()
            .map_err(|_| ParseListError::InvalidNumber(token.to_string()))?;

        if let Some(end_str) = parts.next() {
            let end: u16 = end_str.parse()
                .map_err(|_| ParseListError::InvalidNumber(end_str.to_string()))?;

            if start == 0 || end == 0 || start > end {
                return Err(ParseListError::InvalidRange(token.to_string()));
            }
            ports.extend(start..=end);
        } else {
            if start == 0 {
                return Err(ParseListError::InvalidRange(token.to_string()));
            }
            ports.insert(start);
        }
    }
    Ok(ports.into_iter().collect())
}

/// Parses a comma-separated list of IPs and IP ranges into a Vec<IpAddr>.
///
/// Supports:
/// - Single IP: `10.0.0.5` → [10.0.0.5]
/// - Ranges: `10.0.0.5-10.0.0.8` → [10.0.0.5, 10.0.0.6, ..., 10.0.0.8]
/// - CIDR: `10.0.0.0/24` → [10.0.0.1, 10.0.0.2, ..., 10.0.0.254]
/// - Mixed: `10.0.0.5,10.0.0.7-10.0.0.8,10.0.0.0/28`
pub async fn parse_ips(target: &str) -> Result<Vec<IpAddr>, ResolveTargetError> {
    let mut addrs = BTreeSet::new();
    for part in target.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        if part.contains('/') {
            // CIDR notation
            let network: IpNetwork = part.parse()?;
            for ip in network.iter().map(IpAddr::from) {
                addrs.insert(ip);
            }
        } else if part.contains('-') {
            // Range notation
            let mut bounds = part.splitn(2, '-')
                .map(str::trim);
            let start: IpAddr = bounds
                .next()
                .ok_or_else(|| ResolveTargetError::InvalidRange(part.to_string()))?
                .parse()?;
            let end: IpAddr = bounds
                .next()
                .ok_or_else(|| ResolveTargetError::InvalidRange(part.to_string()))?
                .parse()?;
            // Only IPv4 ranges supported as of right now
            match (start, end) {
                (IpAddr::V4(s), IpAddr::V4(e)) => {
                    let s = u32::from(s);
                    let e = u32::from(e);
                    if s > e {
                        return Err(ResolveTargetError::InvalidRange(part.to_string()));
                    }
                    for n in s..=e {
                        addrs.insert(IpAddr::V4(n.into()));
                    }
                }
                _ => return Err(ResolveTargetError::InvalidRange(part.to_string())),
            }
        } else {
            // Single IP
            let ip: IpAddr = part.parse()?;
            addrs.insert(ip);
        }
    }
    Ok(addrs.into_iter().collect())
}
