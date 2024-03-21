use std::net::IpAddr;

use ipnet::IpNet;

// show only that many addresses, unless "--danger-zone" is set
const HARD_LIMIT: usize = 1_048_576;

pub(crate) fn calculate_all_ips(
    cidr: &str,
    all: bool,
    danger_zone: bool,
) -> Result<impl Iterator<Item = IpAddr>, String> {
    let ip_net = cidr
        .parse::<IpNet>()
        .map_err(|e| format!("'{cidr}' is an invalid CIDR: {e}"))?;

    let ips: Box<dyn Iterator<Item = IpAddr>> = if all && danger_zone {
        Box::new(ip_net.hosts())
    } else if all {
        Box::new(ip_net.hosts().take(HARD_LIMIT))
    } else {
        let mut hosts = ip_net.hosts();

        Box::new(hosts.next().into_iter().chain(hosts.last()))
    };

    Ok(ips)
}

#[cfg(test)]
mod tests {
    use crate::ips::HARD_LIMIT;

    use super::calculate_all_ips;
    use pretty_assertions::assert_eq;
    use std::net::IpAddr;

    fn ip_addr(value: &str) -> IpAddr {
        value.parse::<IpAddr>().unwrap()
    }

    #[test]
    fn test_calculate_all_ips_v4_show_first_and_last() {
        let ips = calculate_all_ips("10.10.10.1/24", false, false)
            .unwrap()
            .collect::<Vec<IpAddr>>();

        assert_eq!(ips, vec![ip_addr("10.10.10.1"), ip_addr("10.10.10.254")]);

        let ips = calculate_all_ips("10.10.10.1/32", false, false)
            .unwrap()
            .collect::<Vec<IpAddr>>();

        assert_eq!(ips, vec![ip_addr("10.10.10.1")]);
    }

    #[test]
    fn test_calculate_all_ips_v4_show_all() {
        let ips = calculate_all_ips("10.10.10.1/8", true, false).unwrap();

        assert_eq!(ips.count(), HARD_LIMIT);
    }

    #[test]
    fn test_calculate_all_ips_v4_show_all_in_danger_zone() {
        let mut ips = calculate_all_ips("10.10.10.0/8", true, true).unwrap();

        // network address and broadcast address are excluded
        assert_eq!(ips.by_ref().next().unwrap(), ip_addr("10.0.0.1"));
        assert_eq!(ips.by_ref().last().unwrap(), ip_addr("10.255.255.254"));
    }

    #[test]
    fn test_calculate_all_ips_v6_show_first_and_last() {
        let ips = calculate_all_ips("2001:db8::/113", false, false)
            .unwrap()
            .collect::<Vec<IpAddr>>();

        assert_eq!(ips, vec![ip_addr("2001:db8::"), ip_addr("2001:db8::7fff")]);

        let ips = calculate_all_ips("2001:db8::/128", false, false)
            .unwrap()
            .collect::<Vec<IpAddr>>();

        assert_eq!(ips, vec![ip_addr("2001:db8::")]);
    }

    #[test]
    fn test_calculate_all_ips_v6_show_all() {
        let ips = calculate_all_ips("2001:db8::/106", true, false).unwrap();

        assert_eq!(ips.count(), HARD_LIMIT);
    }

    #[test]
    fn test_calculate_all_ips_v6_show_all_in_danger_zone() {
        let mut ips = calculate_all_ips("2001:db8::/106", true, true).unwrap();

        // ipv6 has no broadcast address
        assert_eq!(ips.by_ref().next().unwrap(), ip_addr("2001:db8::"));
        assert_eq!(ips.by_ref().last().unwrap(), ip_addr("2001:db8::3f:ffff"));
    }
}
