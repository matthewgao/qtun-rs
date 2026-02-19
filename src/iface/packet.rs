//! IP Packet handling

use std::net::Ipv4Addr;

/// IP Packet wrapper
#[derive(Debug, Clone)]
pub struct PacketIP {
    data: Vec<u8>,
}

impl PacketIP {
    /// Create a new PacketIP with the given size
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
        }
    }

    /// Create a PacketIP from existing bytes
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Get the source IP address (IPv4)
    pub fn source_ip(&self) -> Ipv4Addr {
        if self.data.len() >= 16 {
            Ipv4Addr::new(self.data[12], self.data[13], self.data[14], self.data[15])
        } else {
            Ipv4Addr::new(0, 0, 0, 0)
        }
    }

    /// Get the destination IP address (IPv4)
    pub fn destination_ip(&self) -> Ipv4Addr {
        if self.data.len() >= 20 {
            Ipv4Addr::new(self.data[16], self.data[17], self.data[18], self.data[19])
        } else {
            Ipv4Addr::new(0, 0, 0, 0)
        }
    }

    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Get mutable reference to underlying bytes
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Get the length of the packet
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the packet is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Resize the packet data
    pub fn resize(&mut self, new_len: usize) {
        self.data.resize(new_len, 0);
    }

    /// Truncate the packet to the given length
    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }

    /// Set the packet data from a slice
    pub fn set_data(&mut self, data: &[u8]) {
        self.data.clear();
        self.data.extend_from_slice(data);
    }
}

impl AsRef<[u8]> for PacketIP {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl AsMut<[u8]> for PacketIP {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl From<Vec<u8>> for PacketIP {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl From<PacketIP> for Vec<u8> {
    fn from(pkt: PacketIP) -> Self {
        pkt.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_ip() {
        let mut pkt = PacketIP::new(20);
        // Set source IP at offset 12-15
        pkt.as_bytes_mut()[12] = 192;
        pkt.as_bytes_mut()[13] = 168;
        pkt.as_bytes_mut()[14] = 1;
        pkt.as_bytes_mut()[15] = 1;
        // Set dest IP at offset 16-19
        pkt.as_bytes_mut()[16] = 10;
        pkt.as_bytes_mut()[17] = 0;
        pkt.as_bytes_mut()[18] = 0;
        pkt.as_bytes_mut()[19] = 1;

        assert_eq!(pkt.source_ip(), Ipv4Addr::new(192, 168, 1, 1));
        assert_eq!(pkt.destination_ip(), Ipv4Addr::new(10, 0, 0, 1));
    }
}
