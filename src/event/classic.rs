use crate::{
    event::{InquiryResult, InquiryResultWithRssi},
    param::{BdAddr, ClockOffset, PageScanRepetitionMode},
    AsHciBytes, FromHciBytes,
};

/// Struct representing a single parsed inquiry result item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InquiryResultItem {
    /// Bluetooth Device Address (BD_ADDR) of the device found
    pub bd_addr: BdAddr,
    /// Page scan repetition mode of the device
    pub page_scan_repetition_mode: Option<PageScanRepetitionMode>,
    /// Class of device (CoD) of the device found
    pub class_of_device: Option<[u8; 3]>,
    /// Clock offset of the device found
    pub clock_offset: Option<ClockOffset>,
    /// Received Signal Strength Indicator (RSSI) of the device found
    /// This field is only present in `InquiryResultWithRssi`
    pub rssi: Option<i8>,
}

/// Iterator over inquiry result items
pub struct InquiryResultIter<'a> {
    bytes: &'a [u8],
    num_responses: usize,
    idx: usize,
    kind: InquiryResultKind,
}

/// Kind of inquiry result, indicating whether it includes RSSI or not
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InquiryResultKind {
    /// Standard inquiry result without RSSI
    Standard,
    /// Inquiry result with RSSI
    WithRssi,
}

impl<'a> InquiryResultIter<'a> {
    /// Creates a new iterator for standard inquiry results
    pub fn new_standard(bytes: &'a [u8], num_responses: usize) -> Self {
        InquiryResultIter {
            bytes,
            num_responses,
            idx: 0,
            kind: InquiryResultKind::Standard,
        }
    }

    /// Creates a new iterator for inquiry results with RSSI
    pub fn new_with_rssi(bytes: &'a [u8], num_responses: usize) -> Self {
        InquiryResultIter {
            bytes,
            num_responses,
            idx: 0,
            kind: InquiryResultKind::WithRssi,
        }
    }
}

impl<'a> Iterator for InquiryResultIter<'a> {
    type Item = InquiryResultItem;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.num_responses {
            return None;
        }

        let i = self.idx;
        let n = self.num_responses;

        let bd_addr_size = n * 6;
        let page_scan_size = n * 1;
        let class_size = n * 3;
        let clock_size = n * 2;

        let reserved_size = match self.kind {
            InquiryResultKind::Standard => n * 2,
            InquiryResultKind::WithRssi => n * 1,
        };

        let bd_addr_off = i * 6;
        let page_scan_off = bd_addr_size + i;
        let class_off = bd_addr_size + page_scan_size + reserved_size + i * 3;
        let clock_off = bd_addr_size + page_scan_size + reserved_size + class_size + i * 2;

        if self.bytes.len() < bd_addr_off + 6 {
            return None;
        }

        let bd_addr = BdAddr::new([
            self.bytes[bd_addr_off],
            self.bytes[bd_addr_off + 1],
            self.bytes[bd_addr_off + 2],
            self.bytes[bd_addr_off + 3],
            self.bytes[bd_addr_off + 4],
            self.bytes[bd_addr_off + 5],
        ]);

        let page_scan_repetition_mode = self
            .bytes
            .get(page_scan_off)
            .and_then(|b| PageScanRepetitionMode::from_hci_bytes(&[*b]).ok().map(|(m, _)| m));

        let class_of_device = self.bytes.get(class_off..class_off + 3).map(|s| [s[0], s[1], s[2]]);

        let clock_offset = self
            .bytes
            .get(clock_off..clock_off + 2)
            .and_then(|s| ClockOffset::from_hci_bytes(s).ok().map(|(c, _)| c));

        let rssi = if self.kind == InquiryResultKind::WithRssi {
            let rssi_off = bd_addr_size + page_scan_size + reserved_size + class_size + clock_size + i;
            self.bytes.get(rssi_off).map(|b| *b as i8)
        } else {
            None
        };

        self.idx += 1;

        Some(InquiryResultItem {
            bd_addr,
            page_scan_repetition_mode,
            class_of_device,
            clock_offset,
            rssi,
        })
    }
}

impl<'a> InquiryResult<'a> {
    /// Returns an iterator over all valid inquiry result items.
    pub fn iter(&self) -> InquiryResultIter {
        let bytes = self.bytes.as_hci_bytes();
        let n = self.num_responses as usize;
        InquiryResultIter::new_standard(bytes, n)
    }
}

impl<'a> InquiryResultWithRssi<'a> {
    /// Returns an iterator over all valid inquiry result items.
    pub fn iter(&self) -> InquiryResultIter {
        let bytes = self.bytes.as_hci_bytes();
        let n = self.num_responses as usize;
        InquiryResultIter::new_with_rssi(bytes, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inquiry_result_access() {
        // Test data for InquiryResult with 2 responses
        let data = [
            0x02, // num_responses = 2
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // addr 1
            0x11, 0x12, 0x13, 0x14, 0x15, 0x16, // addr 2
            0x01, 0x02, // R1, R2
            0x00, 0x00, 0x00, 0x00, // reserved
            0x20, 0x04, 0x00, 0x30, 0x05, 0x01, // class of device
            0x34, 0x12, 0x78, 0x56, // clock offsets
        ];
        let (inquiry_result, _) = InquiryResult::from_hci_bytes(&data).unwrap();
        let mut iter = inquiry_result.iter();
        let item1 = iter.next().unwrap();
        assert_eq!(item1.bd_addr.as_hci_bytes(), &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        assert_eq!(item1.page_scan_repetition_mode, Some(PageScanRepetitionMode::R1));
        assert_eq!(item1.class_of_device, Some([0x20, 0x04, 0x00]));
        assert_eq!(item1.clock_offset.unwrap().as_hci_bytes(), &[0x34, 0x12]);
        assert_eq!(item1.rssi, None);
        let item2 = iter.next().unwrap();
        assert_eq!(item2.bd_addr.as_hci_bytes(), &[0x11, 0x12, 0x13, 0x14, 0x15, 0x16]);
        assert_eq!(item2.page_scan_repetition_mode, Some(PageScanRepetitionMode::R2));
        assert_eq!(item2.class_of_device, Some([0x30, 0x05, 0x01]));
        assert_eq!(item2.clock_offset.unwrap().as_hci_bytes(), &[0x78, 0x56]);
        assert_eq!(item2.rssi, None);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_inquiry_result_with_rssi_access() {
        // Test data for InquiryResultWithRssi with 2 responses
        let data = [
            0x02, // num_responses = 2
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // addr 1
            0x11, 0x12, 0x13, 0x14, 0x15, 0x16, // addr 2
            0x01, 0x02, // R1, R2
            0x00, 0x00, // reserved
            0x20, 0x04, 0x00, 0x30, 0x05, 0x01, // class of device
            0x34, 0x12, 0x78, 0x56, // clock offsets
            0xF0, 0xE8, // RSSI
        ];
        let (inquiry_result, _) = InquiryResultWithRssi::from_hci_bytes(&data).unwrap();
        let mut iter = inquiry_result.iter();
        let item1 = iter.next().unwrap();
        assert_eq!(item1.bd_addr.as_hci_bytes(), &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        assert_eq!(item1.page_scan_repetition_mode, Some(PageScanRepetitionMode::R1));
        assert_eq!(item1.class_of_device, Some([0x20, 0x04, 0x00]));
        assert_eq!(item1.clock_offset.unwrap().as_hci_bytes(), &[0x34, 0x12]);
        assert_eq!(item1.rssi, Some(-16));
        let item2 = iter.next().unwrap();
        assert_eq!(item2.bd_addr.as_hci_bytes(), &[0x11, 0x12, 0x13, 0x14, 0x15, 0x16]);
        assert_eq!(item2.page_scan_repetition_mode, Some(PageScanRepetitionMode::R2));
        assert_eq!(item2.class_of_device, Some([0x30, 0x05, 0x01]));
        assert_eq!(item2.clock_offset.unwrap().as_hci_bytes(), &[0x78, 0x56]);
        assert_eq!(item2.rssi, Some(-24));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_simple_inquiry_result() {
        // Test with minimal data - just 1 response
        let data = [
            0x01, // num_responses = 1
            // BD address: 1 address, 6 bytes
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // Page scan repetition mode: 1 mode, 1 byte
            0x01, // R1
            // Reserved: 1 byte
            0x00, // Class of device: 1 device, 3 bytes
            0x20, 0x04, 0x00, // Clock offset: 1 offset, 2 bytes
            0x34, 0x12,
        ];

        let (inquiry_result, _) = InquiryResult::from_hci_bytes(&data).unwrap();
        assert_eq!(inquiry_result.num_responses, 1);

        // Test the first response
        let item = inquiry_result.iter().next();
        assert!(item.is_some());
        if let Some(item) = item {
            assert_eq!(item.bd_addr.as_hci_bytes(), &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        }
    }
}
