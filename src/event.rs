//! HCI events [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d21276b6-83d0-cbc3-8295-6ff23b70a0c5)

use crate::cmd::{Opcode, SyncCmd};
use crate::param::{
    param, AuthenticationRequirements, BdAddr, ClockOffset, ConnHandle, ConnHandleCompletedPackets, ConnectionLinkType,
    CoreSpecificationVersion, Error, FlowDirection, IoCapability, KeyFlag, LinkKeyType, LinkType, LmpFeatureMask,
    LmpMaxSlots, NotificationType, OobDataPresent, PacketType, PageScanRepetitionMode, RemainingBytes, ServiceType,
    Status,
};
use crate::{FromHciBytes, FromHciBytesError, ReadHci, ReadHciError};

/// Classic Bluetooth events
pub mod classic;
pub mod le;

pub use classic::*;
use le::LeEvent;

/// A trait for objects which contain the parameters for a specific HCI event
pub trait EventParams<'a>: FromHciBytes<'a> {
    /// The event code these parameters are for
    const EVENT_CODE: u8;
}

param! {
    /// The header of an HCI event packet.
    struct EventPacketHeader {
        code: u8,
        params_len: u8,
    }
}

macro_rules! events {
    (
        $(
            $(#[$attrs:meta])*
            struct $name:ident$(<$life:lifetime>)?($code:expr) {
                $(
                    $(#[$field_attrs:meta])*
                    $field:ident: $ty:ty
                ),*
                $(,)?
            }
        )+
    ) => {
        /// An Event HCI packet
        #[non_exhaustive]
        #[derive(Debug, Clone, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Event<'a> {
            $(
                #[allow(missing_docs)]
                $name($name$(<$life>)?),
            )+
            #[allow(missing_docs)]
            Le(LeEvent<'a>),
            /// An event with an unknown code value
            Unknown {
                /// The event code
                code: u8,
                /// The bytes of the event parameters
                params: &'a [u8]
            },
        }

        impl<'a> Event<'a> {
            fn from_header_hci_bytes(header: EventPacketHeader, data: &'a [u8]) -> Result<(Self, &'a [u8]), FromHciBytesError> {
                let (data, rest) = if data.len() < usize::from(header.params_len) {
                    return Err(FromHciBytesError::InvalidSize);
                } else {
                    data.split_at(usize::from(header.params_len))
                };

                match header.code {
                    $($code => $name::from_hci_bytes_complete(data).map(|x| (Self::$name(x), rest)),)+
                    0x3e => LeEvent::from_hci_bytes_complete(data).map(|x| (Self::Le(x), rest)),
                    _ => {
                        Ok((Self::Unknown { code: header.code, params: data }, rest))
                    }
                }
            }
        }

        $(
            $(#[$attrs])*
            #[derive(Debug, Clone, Copy, Hash)]
            #[cfg_attr(feature = "defmt", derive(defmt::Format))]
            /// $name
            pub struct $name$(<$life>)? {
                $(
                    /// $field
                    $(#[$field_attrs])*
                    pub $field: $ty,
                )*
            }

            #[automatically_derived]
            impl<'a> $crate::FromHciBytes<'a> for $name$(<$life>)? {
                #[allow(unused_variables)]
                fn from_hci_bytes(data: &'a [u8]) -> Result<(Self, &'a [u8]), $crate::FromHciBytesError> {
                    let total = 0;
                    $(
                        let ($field, data) = <$ty as $crate::FromHciBytes>::from_hci_bytes(data)?;
                    )*
                    Ok((Self {
                        $($field,)*
                    }, data))
                }
            }

            #[automatically_derived]
            impl<'a> $crate::event::EventParams<'a> for $name$(<$life>)? {
                const EVENT_CODE: u8 = $code;
            }
        )+
    };
}

events! {
    /// Disconnection Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-332adb1f-b5ac-5289-82a2-c51a59d533e7)
    struct DisconnectionComplete(0x05) {
        status: Status,
        handle: ConnHandle,
        reason: Status,
    }

    /// Inquiry Result event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3467df70-1d7a-73c5-5a3e-8689dba5523f)
    struct InquiryResult<'a>(0x02) {
        num_responses: u8,
        /// All remaining bytes for this event (contains all fields for all responses)
        bytes: RemainingBytes<'a>,
    }

    /// Extended Inquiry Result event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e3e8c7bc-2262-14f4-b6f5-2eeb0b25aa4f)
    struct ExtendedInquiryResult<'a>(0x2f) {
        num_responses: u8,
        bd_addr: BdAddr,
        page_scan_repetition_mode: PageScanRepetitionMode,
        reserved: u8,
        class_of_device: [u8; 3],
        clock_offset: ClockOffset,
        rssi: i8,
        eir_data: RemainingBytes<'a>,
    }

    /// Inquiry Result with RSSI event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c2550565-1c65-a514-6cf0-3d55c8943dab)
    struct InquiryResultWithRssi<'a>(0x22) {
        num_responses: u8,
        /// All remaining bytes for this event (contains all fields for all responses)
        bytes: RemainingBytes<'a>,
    }

    /// Encryption Change (v1) event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7b7d27f0-1a33-ff57-5b97-7d49a04cea26)
    struct EncryptionChangeV1(0x08) {
        status: Status,
        handle: ConnHandle,
        enabled: bool,
    }

    /// Encryption Change (v2) event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7b7d27f0-1a33-ff57-5b97-7d49a04cea26)
    struct EncryptionChangeV2(0x59) {
        status: Status,
        handle: ConnHandle,
        encryption_enabled: bool,
        encryption_key_size: u8,
    }

    /// Read Remote Version Information Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-81ed98a1-98b1-dae5-a3f5-bb7bc69d39b7)
    struct ReadRemoteVersionInformationComplete(0x0c) {
        status: Status,
        handle: ConnHandle,
        version: CoreSpecificationVersion,
        company_id: u16,
        subversion: u16,
    }

    /// Command Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-76d31a33-1a9e-07bc-87c4-8ebffee065fd)
    struct CommandComplete<'a>(0x0e) {
        num_hci_cmd_pkts: u8,
        cmd_opcode: Opcode,
        status: Status, // All return parameters have status as the first field
        return_param_bytes: RemainingBytes<'a>,
    }

    /// Command Status event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4d87067c-be74-d2ff-d5c4-86416bf7af91)
    struct CommandStatus(0x0f) {
        status: Status,
        num_hci_cmd_pkts: u8,
        cmd_opcode: Opcode,
    }

    /// Hardware Error event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2479a101-ae3b-5b5d-f3d4-4776af39a377)
    struct HardwareError(0x10) {
        hardware_code: u8,
    }

    /// Number Of Completed Packets event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9ccbff85-45ce-9c0d-6d0c-2e6e5af52b0e)
    struct NumberOfCompletedPackets<'a>(0x13) {
        completed_packets: &'a [ConnHandleCompletedPackets],
    }

    /// Data Buffer Overflow event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e15e12c7-d29a-8c25-349f-af6206c2ae57)
    struct DataBufferOverflow(0x1a) {
        link_type: LinkType,
    }

    /// Encryption Key Refresh Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a321123c-83a5-7baf-6971-05edd1241357)
    struct EncryptionKeyRefreshComplete(0x30) {
        status: Status,
        handle: ConnHandle,
    }

    /// Authenticated Payload Timeout Expired event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6cfdff94-ace8-294c-6af9-d90d94653e19)
    struct AuthenticatedPayloadTimeoutExpired(0x57) {
        handle: ConnHandle,
    }

    /// Return Link Keys event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e0d451f2-4b53-cdf0-efb5-926e08b27cd2)
    struct ReturnLinkKeys<'a>(0x15) {
        num_keys: u8,
        bd_addr: RemainingBytes<'a>, // Num_Keys × 6 octets
        link_key: RemainingBytes<'a>, // Num_Keys × 16 octets, always zero
    }

    /// Vendor event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f209cdf7-0496-8bcd-b7e1-500831511378)
    struct Vendor<'a>(0xff) {
        params: RemainingBytes<'a>,
    }

    /// Remote Name Request Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1e2ccd32-b73f-7f00-f4ff-25b2235aaf02)
    struct RemoteNameRequestComplete<'a>(0x07) {
        status: Status,
        bd_addr: BdAddr,
        remote_name: RemainingBytes<'a>, // 248 bytes max, null-terminated string
    }

    /// Remote Host Supported Features Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ba740ba0-44d8-d028-0a67-1abab648f6dd)
    struct RemoteHostSupportedFeaturesNotification(0x3d) {
        bd_addr: BdAddr,
        features: LmpFeatureMask,
    }

    /// Connection Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ebb06dd7-356e-605c-cbc1-d06dc00f1d2b)
    struct ConnectionComplete(0x03) {
        status: Status,
        handle: ConnHandle,
        bd_addr: BdAddr,
        link_type: ConnectionLinkType,
        encryption_enabled: bool,
    }

    /// Link Key Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-58400663-f69d-a482-13af-ec558a3f4c03)
    struct LinkKeyRequest(0x17) {
        bd_addr: BdAddr,
    }

    /// PIN Code Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7666987c-9040-9aaa-cad6-96941b46d2b5)
    struct PinCodeRequest(0x16) {
        bd_addr: BdAddr,
    }

    /// Link Key Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ef6b7301-ab4b-cce6-1fa4-c053c3cd1585)
    struct LinkKeyNotification(0x18) {
        bd_addr: BdAddr,
        link_key: [u8; 16],
        key_type: LinkKeyType,
    }

    /// Authentication Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-00ede464-d351-7076-e82a-d8f4b30ee594)
    struct AuthenticationComplete(0x06) {
        status: Status,
        handle: ConnHandle,
    }

    /// Inquiry Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cde759f8-6c4d-2dd4-7053-1657125ded74)
    struct InquiryComplete(0x01) {
        status: Status,
    }

    /// Connection Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3115f164-ffcd-9451-09ef-0ed3809889eb)
    struct ConnectionRequest(0x04) {
        bd_addr: BdAddr,
        class_of_device: [u8; 3],
        link_type: ConnectionLinkType,
    }

    /// Change Connection Link Key Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8d639c74-ec4f-24e3-4e39-952ce11fba57)
    struct ChangeConnectionLinkKeyComplete(0x09) {
        status: Status,
        handle: ConnHandle,
    }

    /// Link Key Type Changed event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-9eb2cea6-248a-e017-2c09-2797aba08cbf)
    struct LinkKeyTypeChanged(0x0a) {
        status: Status,
        handle: ConnHandle,
        key_flag: KeyFlag,
    }

    /// Read Remote Supported Features Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e191dc19-453a-d0e0-2317-2406ffc4d512)
    struct ReadRemoteSupportedFeaturesComplete(0x0b) {
        status: Status,
        handle: ConnHandle,
        lmp_features: LmpFeatureMask,
    }

    /// Max Slots Change event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1561feb4-2f2e-4ec1-1db7-57ec1dc436e2)
    struct MaxSlotsChange(0x1b) {
        handle: ConnHandle,
        lmp_max_slots: LmpMaxSlots,
    }

    /// Read Clock Offset Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-3ab4cf46-7f13-7901-4abb-af026ebee703)
    struct ReadClockOffsetComplete(0x1c) {
        status: Status,
        handle: ConnHandle,
        clock_offset: ClockOffset,
    }

    /// Connection Packet Type Changed event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-864f38f0-afde-bd09-09ff-c5bb5fefca62)
    struct ConnectionPacketTypeChanged(0x1d) {
        status: Status,
        handle: ConnHandle,
        packet_type: PacketType,
    }

    /// QoS Violation event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cc019a24-4654-aa9f-dcd3-2a286e7cdd55)
    struct QosViolation(0x1e) {
        handle: ConnHandle,
    }

    /// Page Scan Repetition Mode Change event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a535391f-0ecf-ff2d-dfad-1e61b021c0d6)
    struct PageScanRepetitionModeChange(0x20) {
        bd_addr: BdAddr,
        page_scan_repetition_mode: PageScanRepetitionMode,
    }

    /// Flow Specification Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-65dffc5b-4c38-ce5c-a618-0d7a3e145012)
    struct FlowSpecificationComplete(0x21) {
        status: Status,
        handle: ConnHandle,
        unused: u8,
        flow_direction: FlowDirection,
        service_type: ServiceType,
        token_rate: u32,
        token_bucket_size: u32,
        peak_bandwidth: u32,
        access_latency: u32,
    }

    /// Read Remote Extended Features Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6a78850c-d310-761a-74cb-abe8b2ddddd8)
    struct ReadRemoteExtendedFeaturesComplete(0x23) {
        status: Status,
        handle: ConnHandle,
        page_number: u8,
        max_page_number: u8,
        lmp_features: LmpFeatureMask,
    }

    /// Synchronous Connection Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a625aef5-c3d7-12e9-39e4-b8f3386150bb)
    struct SynchronousConnectionComplete(0x2c) {
        status: Status,
        handle: ConnHandle,
        bd_addr: BdAddr,
        link_type: ConnectionLinkType,
        transmission_interval: u8,
        retransmission_window: u8,
        rx_packet_length: u16,
        tx_packet_length: u16,
        air_mode: u8,
    }

    /// Synchronous Connection Changed event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5774f3c9-81e9-24e6-86a6-6f6935ee8998)
    struct SynchronousConnectionChanged(0x2d) {
        status: Status,
        handle: ConnHandle,
        transmission_interval: u8,
        retransmission_window: u8,
        rx_packet_length: u16,
        tx_packet_length: u16,
    }

    /// Sniff Subrating event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4c17aedf-cddf-0844-39cc-3dcfddd34ec0)
    struct SniffSubrating(0x2e) {
        handle: ConnHandle,
        max_tx_latency: u16,
        max_rx_latency: u16,
        min_remote_timeout: u16,
        min_local_timeout: u16,
    }

    /// IO Capability Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-343681e1-ca08-8d4c-79c3-e4b2c86ecba1)
    struct IoCapabilityRequest(0x31) {
        bd_addr: BdAddr,
    }

    /// IO Capability Response event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7f2cf1ee-49ba-de05-5f26-f54925363197)
    struct IoCapabilityResponse(0x32) {
        bd_addr: BdAddr,
        io_capability: IoCapability,
        oob_data_present: OobDataPresent,
        authentication_requirements: AuthenticationRequirements,
    }

    /// User Confirmation Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e7014a9e-718f-6aa8-d657-35b547e9c5d6)
    struct UserConfirmationRequest(0x33) {
        bd_addr: BdAddr,
        numeric_value: u32,
    }

    /// User Passkey Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2f7a60c5-e17f-28f2-699c-e5943e488ec9)
    struct UserPasskeyRequest(0x34) {
        bd_addr: BdAddr,
    }

    /// Remote OOB Data Request event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c5c5d906-fdde-b062-2bc6-6f5b5de25066)
    struct RemoteOobDataRequest(0x35) {
        bd_addr: BdAddr,
    }

    /// Simple Pairing Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-e07a2674-e7bf-c963-0a41-9a997c940d26)
    struct SimplePairingComplete(0x36) {
        status: Status,
        bd_addr: BdAddr,
    }

    /// User Passkey Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-39f61a47-537a-e0d9-4aa4-ccab280ebc99)
    struct UserPasskeyNotification(0x3b) {
        bd_addr: BdAddr,
        passkey: u32,
    }

    /// Keypress Notification event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a96ae6b4-2259-c16d-0098-34a5e551284e)
    struct KeypressNotification(0x3c) {
        bd_addr: BdAddr,
        notification_type: NotificationType,
    }

    /// Link Supervision Timeout Changed event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4d4758a0-eab4-25b8-f05e-2bbdcc6384cc)
    struct LinkSupervisionTimeoutChanged(0x38) {
        handle: ConnHandle,
        link_supervision_timeout: u16,
    }

    /// Enhanced Flush Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-0102cc4f-0f30-c3cb-fd0a-f70ac2d5ed08)
    struct EnhancedFlushComplete(0x39) {
        handle: ConnHandle,
    }

    /// Truncated Page Complete event [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-99bf982f-4155-cdc5-eef4-3a198afd5d09)
    struct TruncatedPageComplete(0x50) {
        status: Status,
        bd_addr: BdAddr,
    }
}

impl<'de> FromHciBytes<'de> for Event<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (header, data) = EventPacketHeader::from_hci_bytes(data)?;
        Self::from_header_hci_bytes(header, data)
    }
}

impl<'de> ReadHci<'de> for Event<'de> {
    const MAX_LEN: usize = 257;

    fn read_hci<R: embedded_io::Read>(mut reader: R, buf: &'de mut [u8]) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 2];
        reader.read_exact(&mut header)?;
        let (header, _) = EventPacketHeader::from_hci_bytes(&header)?;
        let params_len = usize::from(header.params_len);
        if buf.len() < params_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(params_len);
            reader.read_exact(buf)?;
            let (pkt, _) = Self::from_header_hci_bytes(header, buf)?;
            Ok(pkt)
        }
    }

    async fn read_hci_async<R: embedded_io_async::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 2];
        reader.read_exact(&mut header).await?;
        let (header, _) = EventPacketHeader::from_hci_bytes(&header)?;
        let params_len = usize::from(header.params_len);
        if buf.len() < params_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(params_len);
            reader.read_exact(buf).await?;
            let (pkt, _) = Self::from_header_hci_bytes(header, buf)?;
            Ok(pkt)
        }
    }
}

impl CommandComplete<'_> {
    /// Gets the connection handle associated with the command that has completed.
    ///
    /// For commands that return the connection handle provided as a parameter as
    /// their first return parameter, this will be valid even if `status` is an error.
    pub fn handle<C: SyncCmd>(&self) -> Result<C::Handle, FromHciBytesError> {
        C::return_handle(&self.return_param_bytes)
    }

    /// Gets a result with the return parameters for `C` or an `Error` if `status` is
    /// an error.
    ///
    /// # Panics
    ///
    /// May panic if `C::OPCODE` is not equal to `self.cmd_opcode`.
    pub fn to_result<C: SyncCmd>(&self) -> Result<C::Return, Error> {
        self.status
            .to_result()
            .and_then(|_| self.return_params::<C>().or(Err(Error::INVALID_HCI_PARAMETERS)))
    }

    /// Parses the return parameters for `C` from this event. This may fail if `status`
    /// is an error.
    ///
    /// # Panics
    ///
    /// May panic if `C::OPCODE` is not equal to `self.cmd_opcode`.
    pub fn return_params<C: SyncCmd>(&self) -> Result<C::Return, FromHciBytesError> {
        assert_eq!(self.cmd_opcode, C::OPCODE);
        C::Return::from_hci_bytes(&self.return_param_bytes).and_then(|(params, rest)| {
            if rest.is_empty() {
                Ok(params)
            } else {
                Err(FromHciBytesError::InvalidSize)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsHciBytes;

    #[test]
    fn test_inquiry_complete_from_bytes() {
        let data = [0x00];
        let (evt, rest) = InquiryComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_connection_complete_from_bytes() {
        let data = [0x00, 0x01, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x01, 0x01];
        let (evt, rest) = ConnectionComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.link_type, ConnectionLinkType::Acl);
        assert!(evt.encryption_enabled);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_disconnection_complete_from_bytes() {
        let data = [0x00, 0x01, 0x00, 0x13];
        let (evt, rest) = DisconnectionComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.reason, Status::REMOTE_USER_TERMINATED_CONN);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_authentication_complete_from_bytes() {
        let data = [0x00, 0x01, 0x00];
        let (evt, rest) = AuthenticationComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_encryption_change_v1_from_bytes() {
        let data = [0x00, 0x01, 0x00, 0x01];
        let (evt, rest) = EncryptionChangeV1::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert!(evt.enabled);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_read_remote_supported_features_complete_from_bytes() {
        let data = [0x00, 0x01, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let (evt, rest) = ReadRemoteSupportedFeaturesComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_read_remote_version_information_complete_from_bytes() {
        // Bluetooth HCI multi-byte fields are little-endian.
        // Data: [status, handle (LE), version, company_id (LE), subversion (LE)]
        let data = [0x00, 0x01, 0x00, 0x09, 0x34, 0x12, 0x78, 0x56];
        let (evt, rest) = ReadRemoteVersionInformationComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.version, crate::param::CoreSpecificationVersion::VERSION_5_0);
        assert_eq!(evt.company_id, 0x1234); // 0x34, 0x12 (LE)
        assert_eq!(evt.subversion, 0x5678); // 0x78, 0x56 (LE)
        assert!(rest.is_empty());

        // Negative test: swapped company_id/subversion bytes should not match
        let data_be = [0x00, 0x01, 0x00, 0x09, 0x12, 0x34, 0x56, 0x78];
        let (evt_be, _) = ReadRemoteVersionInformationComplete::from_hci_bytes(&data_be).unwrap();
        assert_ne!(evt_be.company_id, 0x1234);
        assert_ne!(evt_be.subversion, 0x5678);
    }

    #[test]
    fn test_command_status_from_bytes() {
        let data = [0x00, 0x01, 0x34, 0x12];
        let (evt, rest) = CommandStatus::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.num_hci_cmd_pkts, 1);
        assert_eq!(evt.cmd_opcode.to_raw(), 0x1234);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_command_complete_from_bytes() {
        let data = [0x01, 0x34, 0x12, 0x00, 0x02, 0x03];
        let (evt, rest) = CommandComplete::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.num_hci_cmd_pkts, 1);
        assert_eq!(evt.cmd_opcode.to_raw(), 0x1234);
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.return_param_bytes.as_hci_bytes(), &[0x02, 0x03]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_connection_request_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x10, 0x20, 0x30, 0x01];
        let (evt, rest) = ConnectionRequest::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.class_of_device, [0x10, 0x20, 0x30]);
        assert_eq!(evt.link_type, ConnectionLinkType::Acl);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_remote_name_request_complete_from_bytes() {
        let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x54, 0x65, 0x73, 0x74, 0x00];
        let (evt, rest) = RemoteNameRequestComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.remote_name.as_hci_bytes(), &[0x54, 0x65, 0x73, 0x74, 0x00]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_change_connection_link_key_complete_from_bytes() {
        let data = [0x00, 0x01, 0x00];
        let (evt, rest) = ChangeConnectionLinkKeyComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_link_key_type_changed_from_bytes() {
        let data = [0x00, 0x01, 0x00, 0x01];
        let (evt, rest) = LinkKeyTypeChanged::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.key_flag, KeyFlag::Temporary);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_hardware_error_from_bytes() {
        let data = [0x42];
        let (evt, rest) = HardwareError::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.hardware_code, 0x42);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_pin_code_request_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let (evt, rest) = PinCodeRequest::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_link_key_request_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let (evt, rest) = LinkKeyRequest::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_link_key_notification_from_bytes() {
        let data = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // BD_ADDR
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, // Link Key (16 bytes)
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x05, // Key Type (AuthenticatedCombinationKeyP192)
        ];
        let (evt, rest) = LinkKeyNotification::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(
            evt.link_key,
            [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f]
        );
        assert_eq!(evt.key_type, crate::param::LinkKeyType::AuthenticatedCombinationKeyP192);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_data_buffer_overflow_from_bytes() {
        let data = [0x01]; // ACL data link type
        let (evt, rest) = DataBufferOverflow::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.link_type, crate::param::LinkType::AclData);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_max_slots_change_from_bytes() {
        let data = [0x01, 0x00, 0x05];
        let (evt, rest) = MaxSlotsChange::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.lmp_max_slots, crate::param::LmpMaxSlots::FiveSlots);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_read_clock_offset_complete_from_bytes() {
        let data = [0x00, 0x01, 0x00, 0x34, 0x12];
        let (evt, rest) = ReadClockOffsetComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        // The test data [0x34, 0x12] represents the clock offset bytes
        assert_eq!(evt.clock_offset.as_hci_bytes(), &[0x34, 0x12]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_connection_packet_type_changed_from_bytes() {
        let data = [0x00, 0x01, 0x00, 0x34, 0x12];
        let (evt, rest) = ConnectionPacketTypeChanged::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        // The test data [0x34, 0x12] represents the packet type bytes
        assert_eq!(evt.packet_type.as_hci_bytes(), &[0x34, 0x12]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_qos_violation_from_bytes() {
        let data = [0x01, 0x00];
        let (evt, rest) = QosViolation::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.handle.raw(), 0x0001);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_page_scan_repetition_mode_change_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x01];
        let (evt, rest) = PageScanRepetitionModeChange::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.page_scan_repetition_mode, crate::param::PageScanRepetitionMode::R1);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_flow_specification_complete_from_bytes() {
        let data = [
            0x00, // status
            0x01, 0x00, // handle
            0x00, // unused
            0x01, // flow_direction
            0x02, // service_type
            0x34, 0x12, 0x00, 0x00, // token_rate (LE)
            0x78, 0x56, 0x00, 0x00, // token_bucket_size (LE)
            0xBC, 0x9A, 0x00, 0x00, // peak_bandwidth (LE)
            0xF0, 0xDE, 0x00, 0x00, // access_latency (LE)
        ];
        let (evt, rest) = FlowSpecificationComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.unused, 0);
        assert_eq!(evt.flow_direction, crate::param::FlowDirection::Incoming);
        assert_eq!(evt.service_type, crate::param::ServiceType::Guaranteed);
        assert_eq!(evt.token_rate, 0x1234);
        assert_eq!(evt.token_bucket_size, 0x5678);
        assert_eq!(evt.peak_bandwidth, 0x9ABC);
        assert_eq!(evt.access_latency, 0xDEF0);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_read_remote_extended_features_complete_from_bytes() {
        let data = [
            0x00, 0x01, 0x00, 0x01, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];
        let (evt, rest) = ReadRemoteExtendedFeaturesComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.page_number, 1);
        assert_eq!(evt.max_page_number, 2);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_synchronous_connection_complete_from_bytes() {
        let data = [
            0x00, // status
            0x01, 0x00, // handle
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x01, // link_type
            0x08, // transmission_interval
            0x04, // retransmission_window
            0x3C, 0x00, // rx_packet_length (LE)
            0x3C, 0x00, // tx_packet_length (LE)
            0x02, // air_mode
        ];
        let (evt, rest) = SynchronousConnectionComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.link_type, crate::param::ConnectionLinkType::Acl);
        assert_eq!(evt.transmission_interval, 8);
        assert_eq!(evt.retransmission_window, 4);
        assert_eq!(evt.rx_packet_length, 60);
        assert_eq!(evt.tx_packet_length, 60);
        assert_eq!(evt.air_mode, 2);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_synchronous_connection_changed_from_bytes() {
        let data = [
            0x00, // status
            0x01, 0x00, // handle
            0x08, // transmission_interval
            0x04, // retransmission_window
            0x3C, 0x00, // rx_packet_length (LE)
            0x3C, 0x00, // tx_packet_length (LE)
        ];
        let (evt, rest) = SynchronousConnectionChanged::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.transmission_interval, 8);
        assert_eq!(evt.retransmission_window, 4);
        assert_eq!(evt.rx_packet_length, 60);
        assert_eq!(evt.tx_packet_length, 60);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_sniff_subrating_from_bytes() {
        let data = [
            0x01, 0x00, // handle
            0x20, 0x03, // max_tx_latency (LE)
            0x20, 0x03, // max_rx_latency (LE)
            0x40, 0x06, // min_remote_timeout (LE)
            0x40, 0x06, // min_local_timeout (LE)
        ];
        let (evt, rest) = SniffSubrating::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.max_tx_latency, 800);
        assert_eq!(evt.max_rx_latency, 800);
        assert_eq!(evt.min_remote_timeout, 1600);
        assert_eq!(evt.min_local_timeout, 1600);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_extended_inquiry_result_from_bytes() {
        let data = [
            0x01, // num_responses
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            0x01, // page_scan_repetition_mode
            0x00, // reserved
            0x10, 0x20, 0x30, // class_of_device
            0x34, 0x12, // clock_offset (LE)
            0x80, // rssi (-128)
            0x05, 0x09, 0x54, 0x65, 0x73, 0x74, // EIR data (example)
        ];
        let (evt, rest) = ExtendedInquiryResult::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.num_responses, 1);
        assert_eq!(evt.bd_addr, BdAddr::new([1, 2, 3, 4, 5, 6]));
        assert_eq!(evt.page_scan_repetition_mode, PageScanRepetitionMode::R1);
        assert_eq!(evt.reserved, 0);
        assert_eq!(evt.class_of_device, [0x10, 0x20, 0x30]);
        // The test data [0x34, 0x12] represents the clock offset bytes
        assert_eq!(evt.clock_offset.as_hci_bytes(), &[0x34, 0x12]);
        assert_eq!(evt.rssi, -128);
        assert_eq!(evt.eir_data.as_hci_bytes(), &[0x05, 0x09, 0x54, 0x65, 0x73, 0x74]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_encryption_key_refresh_complete_from_bytes() {
        let data = [0x00, 0x01, 0x00];
        let (evt, rest) = EncryptionKeyRefreshComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_io_capability_request_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let (evt, rest) = IoCapabilityRequest::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_io_capability_response_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x01, 0x00, 0x02];
        let (evt, rest) = IoCapabilityResponse::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.io_capability, crate::param::IoCapability::DisplayYesNo);
        assert_eq!(evt.oob_data_present, crate::param::OobDataPresent::NotPresent);
        assert_eq!(
            evt.authentication_requirements,
            crate::param::AuthenticationRequirements::MitmNotRequiredDedicatedBonding
        );
        assert!(rest.is_empty());
    }

    #[test]
    fn test_user_confirmation_request_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x78, 0x56, 0x34, 0x12];
        let (evt, rest) = UserConfirmationRequest::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.numeric_value, 0x1234_5678);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_user_passkey_request_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let (evt, rest) = UserPasskeyRequest::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_remote_oob_data_request_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let (evt, rest) = RemoteOobDataRequest::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_simple_pairing_complete_from_bytes() {
        let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let (evt, rest) = SimplePairingComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_user_passkey_notification_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x87, 0x65, 0x43, 0x21];
        let (evt, rest) = UserPasskeyNotification::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(evt.passkey, 0x2143_6587);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_keypress_notification_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x01];
        let (evt, rest) = KeypressNotification::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(
            evt.notification_type,
            crate::param::NotificationType::PasskeyDigitEntered
        );
        assert!(rest.is_empty());
    }

    #[test]
    fn test_remote_host_supported_features_notification_from_bytes() {
        let data = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        ];
        let (evt, rest) = RemoteHostSupportedFeaturesNotification::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);

        // Verify the LmpFeatureMask by checking its raw bytes representation
        assert_eq!(
            evt.features.as_hci_bytes(),
            &[0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17]
        );
        assert!(rest.is_empty());
    }

    #[test]
    fn test_link_supervision_timeout_changed_from_bytes() {
        let data = [0x01, 0x00, 0x34, 0x12];
        let (evt, rest) = LinkSupervisionTimeoutChanged::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.handle.raw(), 0x0001);
        assert_eq!(evt.link_supervision_timeout, 0x1234);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_enhanced_flush_complete_from_bytes() {
        let data = [0x01, 0x00];
        let (evt, rest) = EnhancedFlushComplete::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.handle.raw(), 0x0001);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_truncated_page_complete_from_bytes() {
        let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let (evt, rest) = TruncatedPageComplete::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.bd_addr.raw(), [1, 2, 3, 4, 5, 6]);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_authenticated_payload_timeout_expired_from_bytes() {
        let data = [0x01, 0x00];
        let (evt, rest) = AuthenticatedPayloadTimeoutExpired::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.handle.raw(), 0x0001);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_encryption_change_v2_from_bytes() {
        let data = [0x00, 0x01, 0x00, 0x01, 0x10];
        let (evt, rest) = EncryptionChangeV2::from_hci_bytes(&data).unwrap();
        assert!(evt.status.to_result().is_ok());
        assert_eq!(evt.handle.raw(), 0x0001);
        assert!(evt.encryption_enabled);
        assert_eq!(evt.encryption_key_size, 16);
        assert!(rest.is_empty());
    }

    #[test]
    fn test_vendor_from_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let (evt, rest) = Vendor::from_hci_bytes(&data).unwrap();
        assert_eq!(evt.params.as_hci_bytes(), &[0x01, 0x02, 0x03, 0x04]);
        assert!(rest.is_empty());
    }
}
