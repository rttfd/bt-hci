//! Link Control commands [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fe2a33d3-28f4-9fd1-4d08-62286985c05e)

use crate::{
    cmd,
    param::{
        AllowRoleSwitch, AuthenticationRequirements, BdAddr, ClockOffset, ConnHandle, DisconnectReason,
        EnhancedAcceptSynchronousConnectionRequestParams, EnhancedSetupSynchronousConnectionParams, IoCapability,
        KeyFlag, OobDataPresent, PacketType, PageScanRepetitionMode, RejectReason, RetransmissionEffort, Role, Status,
        StatusBdAddrReturn, SyncPacketType, VoiceSetting,
    },
};

cmd! {
    /// Inquiry command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-2db7bf11-f361-99bd-6161-dc9696f86c6b)
    Inquiry(LINK_CONTROL, 0x0001) {
        InquiryParams {
            lap: [u8; 3],
            inquiry_length: u8,
            num_responses: u8,
        }
        Return = ();
    }
}

cmd! {
    /// Disconnect command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-6bb8119e-aa67-d517-db2a-7470c35fbf4a)
    Disconnect(LINK_CONTROL, 0x0006) {
        DisconnectParams {
            handle: ConnHandle,
            reason: DisconnectReason,
        }
        Return = ();
    }
}

cmd! {
    /// Read Remote Version Information command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ebf3c9ac-0bfa-0ed0-c014-8f8691ea3fe5)
    ReadRemoteVersionInformation(LINK_CONTROL, 0x001d) {
        Params = ConnHandle;
    }
}

cmd! {
    /// Remote Name Request command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-cbd9cb09-59fd-9739-2570-8fae93d45bd7)
    ///
    /// Initiates a remote name request procedure for the specified Bluetooth device.
    RemoteNameRequest(LINK_CONTROL, 0x0019) {
        RemoteNameRequestParams {
            bd_addr: BdAddr,
            page_scan_repetition_mode: PageScanRepetitionMode,
            reserved: u8, // Reserved, shall be set to 0x00.
            clock_offset: ClockOffset,
        }
        Return = ();
    }
}

cmd! {
    /// Create Connection command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4150eaa8-3d28-1113-68cf-5bae5bae78fd)
    ///
    /// Initiates a connection to a remote Bluetooth device.
    CreateConnection(LINK_CONTROL, 0x0005) {
        CreateConnectionParams {
            bd_addr: BdAddr,
            packet_type: PacketType,
            page_scan_repetition_mode: PageScanRepetitionMode,
            reserved: u8, // Reserved, shall be set to 0x00.
            clock_offset: ClockOffset,
            allow_role_switch: AllowRoleSwitch,
        }
        Return = ();
    }
}

cmd! {
    /// Authentication Requested command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-904095aa-072e-02c1-023a-e16571079cd2)
    ///
    /// Initiates authentication (pairing) for the given connection handle.
    AuthenticationRequested(LINK_CONTROL, 0x0011) {
        Params = ConnHandle;
        Return = ();
    }
}

cmd! {
    /// Link Key Request Negative Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-1ca1324a-dd2c-15b6-2ccf-b469b18dbd3d)
    ///
    /// Used to respond to a Link Key Request event when no key is available.
    LinkKeyRequestNegativeReply(LINK_CONTROL, 0x000c) {
        Params = BdAddr;
        Return = BdAddr;
    }
}

cmd! {
    /// PIN Code Request Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-55d6cf30-d90e-f769-5176-e44ac3c3292e)
    ///
    /// Used to reply to a PIN Code Request event with the PIN code.
    PinCodeRequestReply(LINK_CONTROL, 0x000d) {
        PinCodeRequestReplyParams {
            bd_addr: BdAddr,
            pin_code_len: u8,
            pin_code: [u8; 16],
        }
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// Inquiry Cancel command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-75fbb334-4adc-d07a-bd5c-80b85f6e7074)
    ///
    /// Stops the current Inquiry if the BR/EDR Controller is in Inquiry Mode.
    InquiryCancel(LINK_CONTROL, 0x0002) {
        Params = ();
        Return = Status;
    }
}

cmd! {
    /// Periodic Inquiry Mode command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c6b7334e-1150-5351-fddb-92187e6c14e5)
    ///
    /// Configures the BR/EDR Controller to perform automatic periodic inquiries.
    PeriodicInquiryMode(LINK_CONTROL, 0x0003) {
        PeriodicInquiryModeParams {
            max_period_length: u16,
            min_period_length: u16,
            lap: [u8; 3],
            inquiry_length: u8,
            num_responses: u8,
        }
        Return = Status;
    }
}

cmd! {
    /// Exit Periodic Inquiry Mode command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-dc524a7f-f72a-d8dd-32db-de9c963078b0)
    ///
    /// Ends the Periodic Inquiry mode when the local device is in Periodic Inquiry Mode.
    ExitPeriodicInquiryMode(LINK_CONTROL, 0x0004) {
        Params = ();
        Return = Status;
    }
}

cmd! {
    /// Create Connection Cancel command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d16958d4-6ba2-3c28-2a24-3b6170aa73e0)
    ///
    /// Cancels the Create Connection command that was previously issued.
    CreateConnectionCancel(LINK_CONTROL, 0x0008) {
        Params = BdAddr;
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// Accept Connection Request command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-0404fc5c-fe34-1754-0c80-99eebcd27435)
    ///
    /// Used to accept a new incoming connection request
    AcceptConnectionRequest(LINK_CONTROL, 0x0009) {
        AcceptConnectionRequestParams {
            bd_addr: BdAddr,
            role: Role,
        }
        Return = ();
    }
}

cmd! {
    /// Reject Connection Request command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8bf88653-3ade-d1c3-400a-dc463f79e81c)
    ///
    /// Used to reject an incoming connection request.
    RejectConnectionRequest(LINK_CONTROL, 0x000a) {
        RejectConnectionRequestParams {
            bd_addr: BdAddr,
            reason: RejectReason,
        }
        Return = ();
    }
}

cmd! {
    /// Link Key Request Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-fcc241d3-b098-3bb3-3885-a1897a0252d2)
    ///
    /// Used to respond to a Link Key Request event with the stored link key.
    LinkKeyRequestReply(LINK_CONTROL, 0x000b) {
        LinkKeyRequestReplyParams {
            bd_addr: BdAddr,
            link_key: [u8; 16],
        }
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// PIN Code Request Negative Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d4120104-7be4-c701-ddd4-38dcfc064181)
    ///
    /// Used to respond to a PIN Code Request event when no PIN is available or authentication is rejected.
    PinCodeRequestNegativeReply(LINK_CONTROL, 0x000e) {
        Params = BdAddr;
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// Change Connection Packet Type command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ba6ba228-088f-6cc6-cd19-f12fc6fe1473)
    ///
    /// Changes which packet types can be used for a connection that is currently established.
    ChangeConnectionPacketType(LINK_CONTROL, 0x000f) {
        ChangeConnectionPacketTypeParams {
            handle: ConnHandle,
            packet_type: PacketType,
        }
        Return = ();
    }
}

cmd! {
    /// Set Connection Encryption command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-0dd32c20-9eda-0ee0-b15f-cf896c9a1df5)
    ///
    /// Used to enable or disable encryption on a connection after authentication.
    SetConnectionEncryption(LINK_CONTROL, 0x0013) {
        SetConnectionEncryptionParams {
            handle: ConnHandle,
            encryption_enable: bool,
        }
        Return = ();
    }
}

cmd! {
    /// Change Connection Link Key command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-f5fe9df2-765c-4877-65c3-cd945c4eaace)
    ///
    /// Forces the master device to change the link key to a new one.
    ChangeConnectionLinkKey(LINK_CONTROL, 0x0015) {
        Params = ConnHandle;
        Return = ();
    }
}

cmd! {
    /// Link Key Selection command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-025df664-313b-5394-f697-48702de64624)
    ///
    /// Forces the device to use the temporary link key or the semi-permanent link keys.
    LinkKeySelection(LINK_CONTROL, 0x0017) {
        LinkKeySelectionParams {
            key_flag: KeyFlag,
        }
        Return = ();
    }
}

cmd! {
    /// Remote Name Request Cancel command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5acd877b-9043-7cff-82f9-2aa406610643)
    ///
    /// Cancels an ongoing Remote Name Request procedure.
    RemoteNameRequestCancel(LINK_CONTROL, 0x001a) {
        Params = BdAddr;
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// Read Remote Supported Features command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-86223376-28a9-e454-15f2-e420aee8c462)
    ///
    /// Requests the supported features from a remote device.
    ReadRemoteSupportedFeatures(LINK_CONTROL, 0x001b) {
        Params = ConnHandle;
        Return = ();
    }
}

cmd! {
    /// Read Remote Extended Features command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-99e6584f-76ad-c60c-845e-f9d11b0b3d4e)
    ///
    /// Requests the extended features from a remote device for a specific page.
    ReadRemoteExtendedFeatures(LINK_CONTROL, 0x001c) {
        ReadRemoteExtendedFeaturesParams {
            handle: ConnHandle,
            page_number: u8,
        }
        Return = ();
    }
}

cmd! {
    /// Read Clock Offset command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7942db19-4d63-4322-cabe-00b3a6e81915)
    ///
    /// Reads the clock offset of a remote device.
    ReadClockOffset(LINK_CONTROL, 0x001f) {
        Params = ConnHandle;
        Return = ();
    }
}

cmd! {
    /// Read LMP Handle command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ef969f37-51b8-faab-4c73-42b3da5570c2)
    ///
    /// Reads the current LMP Handle associated with the Connection_Handle.
    ReadLmpHandle(LINK_CONTROL, 0x0020) {
        Params = ConnHandle;
        ReadLmpHandleReturn {
            status: Status,
            handle: ConnHandle,
            lmp_handle: u8,
            reserved: u32,
        }
    }
}

cmd! {
    /// Setup Synchronous Connection command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-55d33060-6340-3068-fd72-de602df735a1)
    ///
    /// Adds a new or modifies an existing synchronous logical transport (SCO or eSCO) on a physical link.
    SetupSynchronousConnection(LINK_CONTROL, 0x0028) {
        SetupSynchronousConnectionParams {
            handle: ConnHandle,
            transmit_bandwidth: u32,
            receive_bandwidth: u32,
            max_latency: u16,
            voice_setting: VoiceSetting,
            retransmission_effort: RetransmissionEffort,
            packet_type: SyncPacketType,
        }
        Return = ();
    }
}

cmd! {
    /// Accept Synchronous Connection Request command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-14983bc8-7617-096f-0b3c-ded9a0d225e6)
    ///
    /// Accepts an incoming request for a synchronous connection.
    AcceptSynchronousConnectionRequest(LINK_CONTROL, 0x0029) {
        AcceptSynchronousConnectionRequestParams {
            bd_addr: BdAddr,
            transmit_bandwidth: u32,
            receive_bandwidth: u32,
            max_latency: u16,
            voice_setting: VoiceSetting,
            retransmission_effort: RetransmissionEffort,
            packet_type: SyncPacketType,
        }
        Return = ();
    }
}

cmd! {
    /// Reject Synchronous Connection Request command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8b9406f9-bfa5-cb1d-9926-d14e2e24f8ae)
    ///
    /// Declines an incoming request for a synchronous connection.
    RejectSynchronousConnectionRequest(LINK_CONTROL, 0x002a) {
        RejectSynchronousConnectionRequestParams {
            bd_addr: BdAddr,
            reason: RejectReason,
        }
        Return = ();
    }
}

cmd! {
    /// IO Capability Request Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-063323a1-51b0-a373-8e29-84f9d0e0263e)
    ///
    /// Reply to an IO Capability Request event with the current I/O capabilities of the Host.
    IoCapabilityRequestReply(LINK_CONTROL, 0x002b) {
        IoCapabilityRequestReplyParams {
            bd_addr: BdAddr,
            io_capability: IoCapability,
            oob_data_present: OobDataPresent,
            authentication_requirements: AuthenticationRequirements,
        }
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// User Confirmation Request Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b88e1ed6-d8d8-6472-4b4e-c8467f4b0d9c)
    ///
    /// Reply to a User Confirmation Request event indicating that the user selected "yes".
    UserConfirmationRequestReply(LINK_CONTROL, 0x002c) {
        Params = BdAddr;
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// User Confirmation Request Negative Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-ac00352f-832c-cc33-b873-2c158e372653)
    ///
    /// Reply to a User Confirmation Request event indicating that the user selected "no".
    UserConfirmationRequestNegativeReply(LINK_CONTROL, 0x002d) {
        Params = BdAddr;
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// User Passkey Request Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-b1ddf2d8-1c4f-1870-40d5-f153f3e4b8de)
    ///
    /// Reply to a User Passkey Request event with the passkey entered by the user.
    UserPasskeyRequestReply(LINK_CONTROL, 0x002e) {
        UserPasskeyRequestReplyParams {
            bd_addr: BdAddr,
            numeric_value: u32,
        }
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// User Passkey Request Negative Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8ac80157-c838-4082-12aa-4c8c9f7a2f44)
    ///
    /// Reply to a User Passkey Request event indicating the Host could not provide a passkey.
    UserPasskeyRequestNegativeReply(LINK_CONTROL, 0x002f) {
        Params = BdAddr;
        Return = StatusBdAddrReturn;
    }
}
cmd! {
    /// Remote OOB Data Request Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-161bcaad-0f08-d936-9e30-53e9043f9ccc)
    ///
    /// Reply to a Remote OOB Data Request event with the C and R values received via OOB transfer.
    RemoteOobDataRequestReply(LINK_CONTROL, 0x0030) {
        RemoteOobDataRequestReplyParams {
            bd_addr: BdAddr,
            c: [u8; 16],
            r: [u8; 16],
        }
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// Remote OOB Data Request Negative Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a9f09c32-e2e6-81ba-29e7-25a123c98d14)
    ///
    /// Reply to a Remote OOB Data Request event that the Host does not have the C and R values.
    RemoteOobDataRequestNegativeReply(LINK_CONTROL, 0x0033) {
        Params = BdAddr;
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// IO Capability Request Negative Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-7bf12908-76d8-4e5a-d728-ada028167713)
    ///
    /// Reject a pairing attempt after an IO Capability Request event has been received.
    IoCapabilityRequestNegativeReply(LINK_CONTROL, 0x0034) {
        IoCapabilityRequestNegativeReplyParams {
            bd_addr: BdAddr,
            reason: Status,
        }
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// Enhanced Setup Synchronous Connection command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-78100e4f-3531-ee07-671a-69ad93fbe6e4)
    ///
    /// Adds a new or modifies an existing synchronous logical transport (SCO or eSCO) on a physical link with enhanced parameters.
    EnhancedSetupSynchronousConnection(LINK_CONTROL, 0x003d) {
        Params = EnhancedSetupSynchronousConnectionParams;
        Return = ();
    }
}

cmd! {
    /// Enhanced Accept Synchronous Connection Request command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-a0023a66-07b5-2ffc-05d3-7c39ee1cc9c5)
    ///
    /// Accepts an incoming request for a synchronous connection with enhanced parameters.
    EnhancedAcceptSynchronousConnectionRequest(LINK_CONTROL, 0x003e) {
        Params = EnhancedAcceptSynchronousConnectionRequestParams;
        Return = ();
    }
}

cmd! {
    /// Truncated Page command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-c1936c59-ec3d-6e85-9038-56afa8d0fc45)
    ///
    /// Pages the BR/EDR Controller and then aborts the paging sequence after an ID response.
    TruncatedPage(LINK_CONTROL, 0x003f) {
       TruncatedPageParams {
            bd_addr: BdAddr,
            page_scan_repetition_mode: PageScanRepetitionMode,
            clock_offset: ClockOffset,
        }
        Return = ();
    }
}

cmd! {
    /// Truncated Page Cancel command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-4ace6cc5-f527-8a3d-fa2b-21e9908dd10f)
    ///
    /// Requests cancellation of an ongoing Truncated Page process.
    TruncatedPageCancel(LINK_CONTROL, 0x0040) {
        Params = BdAddr;
        Return = StatusBdAddrReturn;
    }
}

cmd! {
    /// Set Connectionless Peripheral Broadcast command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-5f51425b-b5b3-6ae6-00e6-0c7d09352035)
    ///
    /// Used to enable or disable Connectionless Peripheral Broadcast mode in the BR/EDR Controller.
    SetConnectionlessPeripheralBroadcast(LINK_CONTROL, 0x0041) {
        SetConnectionlessPeripheralBroadcastParams {
            enable: bool,
            lt_addr: u8,
            lpo_allowed: bool,
            packet_type: PacketType,
            interval_min: u16,
            interval_max: u16,
            supervision_timeout: u16,
        }
        SetConnectionlessPeripheralBroadcastReturn {
            status: Status,
            lt_addr: u8,
            interval: u16,
        }
    }
}

cmd! {
    /// Set Connectionless Peripheral Broadcast Receive command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-d4a2f59e-1c08-3d28-1b3d-0b1519dd4b5a)
    ///
    /// Used to enable or disable the reception of Connectionless Peripheral Broadcast packets.
    SetConnectionlessPeripheralBroadcastReceive(LINK_CONTROL, 0x0042) {
        SetConnectionlessPeripheralBroadcastReceiveParams {
            enable: bool,
            bd_addr: BdAddr,
            lt_addr: u8,
            interval: u16,
            clock_offset: u32,
            next_broadcast_instant: u32,
            supervision_timeout: u16,
            remote_timing_accuracy: u8,
            skip: u8,
            packet_type: PacketType,
            afh_channel_map: [u8; 10],
        }
        SetConnectionlessPeripheralBroadcastReceiveReturn {
            status: Status,
            bd_addr: BdAddr,
            lt_addr: u8,
        }
    }
}

cmd! {
    /// Start Synchronization Train command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-55896f81-cd4b-f318-c3b8-85f369feac6a)
    ///
    /// Used to start the synchronization train.
    StartSynchronizationTrain(LINK_CONTROL, 0x0043) {
        Params = ();
        Return = ();
    }
}

cmd! {
    /// Receive Synchronization Train command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-8dda9429-f279-a65c-9f9e-777be087eed7)
    ///
    /// Used to begin listening for the synchronization train from the device with the specified BD_ADDR.
    ReceiveSynchronizationTrain(LINK_CONTROL, 0x0044) {
        ReceiveSynchronizationTrainParams {
            bd_addr: BdAddr,
            sync_scan_timeout: u16,
            sync_scan_window: u16,
            sync_scan_interval: u16,
        }
        Return = ();
    }
}

cmd! {
    /// Remote OOB Extended Data Request Reply command [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface/host-controller-interface-functional-specification.html#UUID-46c8dd9e-c31c-8bb5-2e8a-71abb75caada)
    ///
    /// Reply to a Remote OOB Data Request event with the C and R values received via OOB transfer for both P-192 and P-256.
    RemoteOobExtendedDataRequestReply(LINK_CONTROL, 0x0045) {
        RemoteOobExtendedDataRequestReplyParams {
            bd_addr: BdAddr,
            c_192: [u8; 16],
            r_192: [u8; 16],
            c_256: [u8; 16],
            r_256: [u8; 16],
        }
        Return = StatusBdAddrReturn;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::{Cmd, OpcodeGroup};
    use crate::param::DisconnectReason;

    const OGF: OpcodeGroup = OpcodeGroup(0x01);

    #[test]
    fn test_inquiry_command() {
        // Test that we can create the command with direct parameters
        let _cmd = Inquiry::new(
            [0x9e, 0x8b, 0x33], // General/Unlimited Inquiry Access Code
            0x08,               // 10.24 seconds
            0x00,               // Unlimited number of responses
        );

        assert_eq!(Inquiry::OPCODE.cmd(), 0x0001);
    }

    #[test]
    fn test_inquiry_cancel_command() {
        let _cmd = InquiryCancel::new();

        assert_eq!(InquiryCancel::OPCODE.group(), OGF);
        assert_eq!(InquiryCancel::OPCODE.cmd(), 0x0002);
    }

    #[test]
    fn test_disconnect_command() {
        let _cmd = Disconnect::new(ConnHandle::new(0x0001), DisconnectReason::AuthenticationFailure);

        assert_eq!(Disconnect::OPCODE.group(), OGF);
        assert_eq!(Disconnect::OPCODE.cmd(), 0x0006);
    }

    #[test]
    fn test_create_connection_command() {
        let _cmd = CreateConnection::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            PacketType::new()
                .set_dh1_may_be_used(true)
                .set_dm3_may_be_used(true)
                .set_dh3_may_be_used(true),
            PageScanRepetitionMode::R2,
            0x00, // reserved
            ClockOffset::new(),
            AllowRoleSwitch::Allowed,
        );

        assert_eq!(CreateConnection::OPCODE.group(), OGF);
        assert_eq!(CreateConnection::OPCODE.cmd(), 0x0005);
    }

    #[test]
    fn test_accept_connection_request_command() {
        let _cmd = AcceptConnectionRequest::new(BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]), Role::Peripheral);

        assert_eq!(AcceptConnectionRequest::OPCODE.group(), OGF);
        assert_eq!(AcceptConnectionRequest::OPCODE.cmd(), 0x0009);
    }

    #[test]
    fn test_reject_connection_request_command() {
        let _cmd = RejectConnectionRequest::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            RejectReason::LimitedResources,
        );

        assert_eq!(RejectConnectionRequest::OPCODE.group(), OGF);
        assert_eq!(RejectConnectionRequest::OPCODE.cmd(), 0x000a);
    }

    #[test]
    fn test_link_key_request_reply_command() {
        let _cmd = LinkKeyRequestReply::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
            ],
        );

        assert_eq!(LinkKeyRequestReply::OPCODE.group(), OGF);
        assert_eq!(LinkKeyRequestReply::OPCODE.cmd(), 0x000b);
    }

    #[test]
    fn test_link_key_request_negative_reply_command() {
        let bd_addr = BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]);
        let _cmd = LinkKeyRequestNegativeReply::new(bd_addr);

        assert_eq!(LinkKeyRequestNegativeReply::OPCODE.group(), OGF);
        assert_eq!(LinkKeyRequestNegativeReply::OPCODE.cmd(), 0x000c);
    }

    #[test]
    fn test_pin_code_request_reply_command() {
        let _cmd = PinCodeRequestReply::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            4,
            [b'1', b'2', b'3', b'4', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        assert_eq!(PinCodeRequestReply::OPCODE.group(), OGF);
        assert_eq!(PinCodeRequestReply::OPCODE.cmd(), 0x000d);
    }

    #[test]
    fn test_authentication_requested_command() {
        let _cmd = AuthenticationRequested::new(ConnHandle::new(0x0001));

        assert_eq!(AuthenticationRequested::OPCODE.group(), OGF);
        assert_eq!(AuthenticationRequested::OPCODE.cmd(), 0x0011);
    }

    #[test]
    fn test_set_connection_encryption_command() {
        let _cmd = SetConnectionEncryption::new(ConnHandle::new(0x0001), true);

        assert_eq!(SetConnectionEncryption::OPCODE.group(), OGF);
        assert_eq!(SetConnectionEncryption::OPCODE.cmd(), 0x0013);
    }

    #[test]
    fn test_change_connection_link_key_command() {
        let _cmd = ChangeConnectionLinkKey::new(ConnHandle::new(0x0001));

        assert_eq!(ChangeConnectionLinkKey::OPCODE.group(), OGF);
        assert_eq!(ChangeConnectionLinkKey::OPCODE.cmd(), 0x0015);
    }

    #[test]
    fn test_remote_name_request_command() {
        let _cmd = RemoteNameRequest::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            PageScanRepetitionMode::R2,
            0x00, // reserved
            ClockOffset::new().set_clock_offset_0(true),
        );

        assert_eq!(RemoteNameRequest::OPCODE.group(), OGF);
        assert_eq!(RemoteNameRequest::OPCODE.cmd(), 0x0019);
    }

    #[test]
    fn test_read_remote_supported_features_command() {
        let _cmd = ReadRemoteSupportedFeatures::new(ConnHandle::new(0x0001));

        assert_eq!(ReadRemoteSupportedFeatures::OPCODE.group(), OGF);
        assert_eq!(ReadRemoteSupportedFeatures::OPCODE.cmd(), 0x001b);
    }

    #[test]
    fn test_read_remote_extended_features_command() {
        let _cmd = ReadRemoteExtendedFeatures::new(ConnHandle::new(0x0001), 0x01);

        assert_eq!(ReadRemoteExtendedFeatures::OPCODE.group(), OGF);
        assert_eq!(ReadRemoteExtendedFeatures::OPCODE.cmd(), 0x001c);
    }

    #[test]
    fn test_read_remote_version_information_command() {
        let _cmd = ReadRemoteVersionInformation::new(ConnHandle::new(0x0001));

        assert_eq!(ReadRemoteVersionInformation::OPCODE.group(), OGF);
        assert_eq!(ReadRemoteVersionInformation::OPCODE.cmd(), 0x001d);
    }

    #[test]
    fn test_read_clock_offset_command() {
        let _cmd = ReadClockOffset::new(ConnHandle::new(0x0001));

        assert_eq!(ReadClockOffset::OPCODE.group(), OGF);
        assert_eq!(ReadClockOffset::OPCODE.cmd(), 0x001f);
    }

    #[test]
    fn test_read_lmp_handle_command() {
        let _cmd = ReadLmpHandle::new(ConnHandle::new(0x0001));

        assert_eq!(ReadLmpHandle::OPCODE.group(), OGF);
        assert_eq!(ReadLmpHandle::OPCODE.cmd(), 0x0020);
    }

    #[test]
    fn test_setup_synchronous_connection_command() {
        let _cmd = SetupSynchronousConnection::new(
            ConnHandle::new(0x0001),
            8000,
            8000,
            0x000a,
            VoiceSetting::new().set_air_coding_format_0(true),
            RetransmissionEffort::OptimizePowerConsumption,
            SyncPacketType::new().set_ev3_may_be_used(true),
        );

        assert_eq!(SetupSynchronousConnection::OPCODE.group(), OGF);
        assert_eq!(SetupSynchronousConnection::OPCODE.cmd(), 0x0028);
    }

    #[test]
    fn test_accept_synchronous_connection_request_command() {
        let _cmd = AcceptSynchronousConnectionRequest::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            8000,
            8000,
            0x000a,
            VoiceSetting::new().set_air_coding_format_0(true),
            RetransmissionEffort::OptimizePowerConsumption,
            SyncPacketType::new().set_ev3_may_be_used(true),
        );

        assert_eq!(AcceptSynchronousConnectionRequest::OPCODE.group(), OGF);
        assert_eq!(AcceptSynchronousConnectionRequest::OPCODE.cmd(), 0x0029);
    }

    #[test]
    fn test_reject_synchronous_connection_request_command() {
        let _cmd = RejectSynchronousConnectionRequest::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            RejectReason::LimitedResources,
        );

        assert_eq!(RejectSynchronousConnectionRequest::OPCODE.group(), OGF);
        assert_eq!(RejectSynchronousConnectionRequest::OPCODE.cmd(), 0x002a);
    }

    #[test]
    fn test_io_capability_request_reply_command() {
        let _cmd = IoCapabilityRequestReply::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            IoCapability::DisplayYesNo,
            OobDataPresent::NotPresent,
            AuthenticationRequirements::MitmRequiredGeneralBonding,
        );

        assert_eq!(IoCapabilityRequestReply::OPCODE.group(), OGF);
        assert_eq!(IoCapabilityRequestReply::OPCODE.cmd(), 0x002b);
    }

    #[test]
    fn test_user_confirmation_request_reply_command() {
        let _cmd = UserConfirmationRequestReply::new(BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]));

        assert_eq!(UserConfirmationRequestReply::OPCODE.group(), OGF);
        assert_eq!(UserConfirmationRequestReply::OPCODE.cmd(), 0x002c);
    }

    #[test]
    fn test_user_passkey_request_reply_command() {
        let _cmd = UserPasskeyRequestReply::new(BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]), 123456);

        assert_eq!(UserPasskeyRequestReply::OPCODE.group(), OGF);
        assert_eq!(UserPasskeyRequestReply::OPCODE.cmd(), 0x002e);
    }

    #[test]
    fn test_remote_oob_data_request_reply_command() {
        let _cmd = RemoteOobDataRequestReply::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
            ],
            [
                0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
            ],
        );

        assert_eq!(RemoteOobDataRequestReply::OPCODE.group(), OGF);
        assert_eq!(RemoteOobDataRequestReply::OPCODE.cmd(), 0x0030);
    }

    #[test]
    fn test_truncated_page_command() {
        let _cmd = TruncatedPage::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            PageScanRepetitionMode::R2,
            ClockOffset::new(),
        );

        assert_eq!(TruncatedPage::OPCODE.group(), OGF);
        assert_eq!(TruncatedPage::OPCODE.cmd(), 0x003f);
    }

    #[test]
    fn test_truncated_page_cancel_command() {
        let _cmd = TruncatedPageCancel::new(BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]));

        assert_eq!(TruncatedPageCancel::OPCODE.group(), OGF);
        assert_eq!(TruncatedPageCancel::OPCODE.cmd(), 0x0040);
    }

    #[test]
    fn test_start_synchronization_train_command() {
        let _cmd = StartSynchronizationTrain::new();

        assert_eq!(StartSynchronizationTrain::OPCODE.group(), OGF);
        assert_eq!(StartSynchronizationTrain::OPCODE.cmd(), 0x0043);
    }

    #[test]
    fn test_receive_synchronization_train_command() {
        let _cmd = ReceiveSynchronizationTrain::new(
            BdAddr::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]),
            0x2000, // 20.48 seconds
            0x0012, // 11.25 ms
            0x0800, // 1.28 seconds
        );

        assert_eq!(ReceiveSynchronizationTrain::OPCODE.group(), OGF);
        assert_eq!(ReceiveSynchronizationTrain::OPCODE.cmd(), 0x0044);
    }

    #[test]
    fn test_enhanced_setup_synchronous_connection_command() {
        let params = EnhancedSetupSynchronousConnectionParams {
            handle: ConnHandle::new(0x0001),
            transmit_bandwidth: 8000,
            receive_bandwidth: 8000,
            transmit_coding_format: [0x00, 0x00, 0x00, 0x00, 0x00], // u-law log
            receive_coding_format: [0x00, 0x00, 0x00, 0x00, 0x00],  // u-law log
            transmit_codec_frame_size: 0x003c,
            receive_codec_frame_size: 0x003c,
            input_bandwidth: 8000,
            output_bandwidth: 8000,
            input_coding_format: [0x00, 0x00, 0x00, 0x00, 0x00],
            output_coding_format: [0x00, 0x00, 0x00, 0x00, 0x00],
            input_coded_data_size: 0x0008,
            output_coded_data_size: 0x0008,
            input_pcm_data_format: 0x00,
            output_pcm_data_format: 0x00,
            input_pcm_sample_payload_msb_position: 0x00,
            output_pcm_sample_payload_msb_position: 0x00,
            input_data_path: 0x00,
            output_data_path: 0x00,
            input_transport_unit_size: 0x00,
            output_transport_unit_size: 0x00,
            max_latency: 0x000a,
            packet_type: SyncPacketType::new().set_ev3_may_be_used(true),
            retransmission_effort: RetransmissionEffort::OptimizePowerConsumption,
        };

        let _cmd = EnhancedSetupSynchronousConnection::new(params);

        assert_eq!(EnhancedSetupSynchronousConnection::OPCODE.group(), OGF);
        assert_eq!(EnhancedSetupSynchronousConnection::OPCODE.cmd(), 0x003d);
    }
}
