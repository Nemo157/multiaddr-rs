use std::ops::Add;

use Segment;

/// A decoded multiaddr.
#[derive(Eq, PartialEq, Clone)]
pub struct MultiAddr {
    segments: Vec<Segment>,
}

impl MultiAddr {
    /// Create a new `MultiAddr` from the given segments.
    pub fn new(segments: Vec<Segment>) -> MultiAddr {
        MultiAddr {
            segments: segments,
        }
    }

    /// Get a reference to the segments that make up this `MultiAddr`.
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// Attempt to split off the last component of this `MultiAddr`, if this
    /// address is empty will return `None`, otherwise returns tuple with a
    /// `MultiAddr` containing all except the last segment and the last segment
    /// by itself.
    ///
    /// An example usecase is in IPFS, peer addresses are formatted as
    /// `/<routing info>/ipfs/<peer id hash>`, e.g.
    /// `/ip4/104.131.131.82/tcp/4001/ipfs/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ`
    /// refers to a peer identified by the multihash
    /// `QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ`
    /// and accessible at `/ip4/104.131.131.82/tcp/4001`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use maddr::{ MultiAddr, Segment };
    /// let addr: MultiAddr = "/ip4/104.131.131.82/tcp/4001/ipfs/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ".parse().unwrap();
    /// if let Some((addr, Segment::Ipfs(hash))) = addr.split_off_last() {
    ///     println!("Peer {} is accessible at {}", hash, addr);
    /// }
    /// ```
    pub fn split_off_last(mut self) -> Option<(MultiAddr, Segment)> {
        self.segments.pop().map(|tail| (self, tail))
    }
}

impl<T> From<T> for MultiAddr where T: Into<Segment> {
    fn from(segment: T) -> MultiAddr {
        MultiAddr::new(vec![segment.into()])
    }
}

impl<T> Add<T> for MultiAddr where T: Into<MultiAddr> {
    type Output = MultiAddr;

    fn add(mut self, rhs: T) -> MultiAddr {
        self.segments.extend_from_slice(&rhs.into().segments);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use {MultiAddr, Segment};

    #[test]
    fn from_ip4() {
        assert_eq!(
            MultiAddr::new(vec![Segment::IP4(Ipv4Addr::new(1, 2, 3, 4))]),
            Ipv4Addr::new(1, 2, 3, 4).into());
    }

    #[test]
    fn add() {
        assert_eq!(
            MultiAddr::new(vec![
                Segment::IP4(Ipv4Addr::new(1, 2, 3, 4)),
                Segment::Tcp(22),
            ]),
            MultiAddr::from(Ipv4Addr::new(1, 2, 3, 4)) + Segment::Tcp(22));
    }
}
