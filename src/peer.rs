use libc;
extern "C" {
    #[no_mangle]
    fn ntohl(__netlong: Uint32T) -> Uint32T;
    #[no_mangle]
    fn ntohs(__netshort: Uint16T) -> Uint16T;
    #[no_mangle]
    fn htonl(__hostlong: Uint32T) -> Uint32T;
    #[no_mangle]
    fn htons(__hostshort: Uint16T) -> Uint16T;
    /* * @defgroup callbacks ENet internal callbacks
    @{
    @ingroup private
*/
    #[no_mangle]
    fn enet_malloc(_: SizeT) -> *mut libc::c_void;
    #[no_mangle]
    fn enet_free(_: *mut libc::c_void);
    /* * @} */
    #[no_mangle]
    fn enet_packet_create(_: *const libc::c_void, _: SizeT, _: EnetUint32)
     -> *mut ENetPacket;
    #[no_mangle]
    fn enet_packet_destroy(_: *mut ENetPacket);
    #[no_mangle]
    fn enet_host_flush(_: *mut ENetHost);
    #[no_mangle]
    fn enet_protocol_command_size(_: enet_uint8) -> SizeT;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn enet_list_clear(_: *mut ENetList);
    #[no_mangle]
    fn enet_list_insert(_: ENetListIterator, _: *mut libc::c_void)
     -> ENetListIterator;
    #[no_mangle]
    fn enet_list_remove(_: ENetListIterator) -> *mut libc::c_void;
    #[no_mangle]
    fn enet_list_move(_: ENetListIterator, _: *mut libc::c_void,
                      _: *mut libc::c_void) -> ENetListIterator;
}
pub type SizeT = libc::c_ulong;
pub type __Uint16T = libc::c_ushort;
pub type __Uint32T = libc::c_uint;
pub type Uint16T = __Uint16T;
pub type Uint32T = __Uint32T;
/* * 
 @file  unix.h
 @brief ENet Unix header
*/
pub type ENetSocket = libc::c_int;
/* *< macro that converts host to net byte-order of a 16-bit value */
/* *< macro that converts host to net byte-order of a 32-bit value */
/* *< macro that converts net to host byte-order of a 16-bit value */
/* *< macro that converts net to host byte-order of a 32-bit value */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ENetBuffer {
    pub data: *mut libc::c_void,
    pub data_length: SizeT,
}
/* * 
 @file  types.h
 @brief type definitions for ENet
*/
/* *< unsigned 8-bit type  */
pub type enet_uint8 = libc::c_uchar;
/* *< unsigned 16-bit type */
pub type EnetUint16 = libc::c_ushort;
/* *< unsigned 32-bit type */
pub type EnetUint32 = libc::c_uint;
/* * 
 @file  protocol.h
 @brief ENet protocol
*/
pub type Unnamed = libc::c_uint;
pub const ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT: Unnamed = 1048576;
pub const ENET_PROTOCOL_MAXIMUM_PEER_ID: Unnamed = 4095;
pub const ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT: Unnamed = 255;
pub const ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT: Unnamed = 1;
pub const ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE: Unnamed = 65536;
pub const ENET_PROTOCOL_MINIMUM_WINDOW_SIZE: Unnamed = 4096;
pub const ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS: Unnamed = 32;
pub const ENET_PROTOCOL_MAXIMUM_MTU: Unnamed = 4096;
pub const ENET_PROTOCOL_MINIMUM_MTU: Unnamed = 576;
pub type _ENetProtocolCommand = libc::c_uint;
pub const ENET_PROTOCOL_COMMAND_MASK: _ENetProtocolCommand = 15;
pub const ENET_PROTOCOL_COMMAND_COUNT: _ENetProtocolCommand = 13;
pub const ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT: _ENetProtocolCommand
          =
    12;
pub const ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE: _ENetProtocolCommand = 11;
pub const ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT: _ENetProtocolCommand = 10;
pub const ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED: _ENetProtocolCommand = 9;
pub const ENET_PROTOCOL_COMMAND_SEND_FRAGMENT: _ENetProtocolCommand = 8;
pub const ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE: _ENetProtocolCommand = 7;
pub const ENET_PROTOCOL_COMMAND_SEND_RELIABLE: _ENetProtocolCommand = 6;
pub const ENET_PROTOCOL_COMMAND_PING: _ENetProtocolCommand = 5;
pub const ENET_PROTOCOL_COMMAND_DISCONNECT: _ENetProtocolCommand = 4;
pub const ENET_PROTOCOL_COMMAND_VERIFY_CONNECT: _ENetProtocolCommand = 3;
pub const ENET_PROTOCOL_COMMAND_CONNECT: _ENetProtocolCommand = 2;
pub const ENET_PROTOCOL_COMMAND_ACKNOWLEDGE: _ENetProtocolCommand = 1;
pub const ENET_PROTOCOL_COMMAND_NONE: _ENetProtocolCommand = 0;
pub type _ENetProtocolFlag = libc::c_uint;
pub const ENET_PROTOCOL_HEADER_SESSION_SHIFT: _ENetProtocolFlag = 12;
pub const ENET_PROTOCOL_HEADER_SESSION_MASK: _ENetProtocolFlag = 12288;
pub const ENET_PROTOCOL_HEADER_FLAG_MASK: _ENetProtocolFlag = 49152;
pub const ENET_PROTOCOL_HEADER_FLAG_SENT_TIME: _ENetProtocolFlag = 32768;
pub const ENET_PROTOCOL_HEADER_FLAG_COMPRESSED: _ENetProtocolFlag = 16384;
pub const ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED: _ENetProtocolFlag = 64;
pub const ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE: _ENetProtocolFlag = 128;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolHeader {
    pub peerID: EnetUint16,
    pub sentTime: EnetUint16,
}
pub type ENetProtocolHeader = _ENetProtocolHeader;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolCommandHeader {
    pub command: enet_uint8,
    pub channelID: enet_uint8,
    pub reliableSequenceNumber: EnetUint16,
}
pub type ENetProtocolCommandHeader = _ENetProtocolCommandHeader;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolAcknowledge {
    pub header: ENetProtocolCommandHeader,
    pub receivedReliableSequenceNumber: EnetUint16,
    pub receivedSentTime: EnetUint16,
}
pub type ENetProtocolAcknowledge = _ENetProtocolAcknowledge;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: EnetUint16,
    pub incomingSessionID: enet_uint8,
    pub outgoingSessionID: enet_uint8,
    pub mtu: EnetUint32,
    pub windowSize: EnetUint32,
    pub channelCount: EnetUint32,
    pub incomingBandwidth: EnetUint32,
    pub outgoingBandwidth: EnetUint32,
    pub packetThrottleInterval: EnetUint32,
    pub packetThrottleAcceleration: EnetUint32,
    pub packetThrottleDeceleration: EnetUint32,
    pub connectID: EnetUint32,
    pub data: EnetUint32,
}
pub type ENetProtocolConnect = _ENetProtocolConnect;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolVerifyConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: EnetUint16,
    pub incomingSessionID: enet_uint8,
    pub outgoingSessionID: enet_uint8,
    pub mtu: EnetUint32,
    pub windowSize: EnetUint32,
    pub channelCount: EnetUint32,
    pub incomingBandwidth: EnetUint32,
    pub outgoingBandwidth: EnetUint32,
    pub packetThrottleInterval: EnetUint32,
    pub packetThrottleAcceleration: EnetUint32,
    pub packetThrottleDeceleration: EnetUint32,
    pub connectID: EnetUint32,
}
pub type ENetProtocolVerifyConnect = _ENetProtocolVerifyConnect;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolBandwidthLimit {
    pub header: ENetProtocolCommandHeader,
    pub incomingBandwidth: EnetUint32,
    pub outgoingBandwidth: EnetUint32,
}
pub type ENetProtocolBandwidthLimit = _ENetProtocolBandwidthLimit;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolThrottleConfigure {
    pub header: ENetProtocolCommandHeader,
    pub packetThrottleInterval: EnetUint32,
    pub packetThrottleAcceleration: EnetUint32,
    pub packetThrottleDeceleration: EnetUint32,
}
pub type ENetProtocolThrottleConfigure = _ENetProtocolThrottleConfigure;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolDisconnect {
    pub header: ENetProtocolCommandHeader,
    pub data: EnetUint32,
}
pub type ENetProtocolDisconnect = _ENetProtocolDisconnect;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolPing {
    pub header: ENetProtocolCommandHeader,
}
pub type ENetProtocolPing = _ENetProtocolPing;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolSendReliable {
    pub header: ENetProtocolCommandHeader,
    pub data_length: EnetUint16,
}
pub type ENetProtocolSendReliable = _ENetProtocolSendReliable;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolSendUnreliable {
    pub header: ENetProtocolCommandHeader,
    pub unreliableSequenceNumber: EnetUint16,
    pub data_length: EnetUint16,
}
pub type ENetProtocolSendUnreliable = _ENetProtocolSendUnreliable;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolSendUnsequenced {
    pub header: ENetProtocolCommandHeader,
    pub unsequencedGroup: EnetUint16,
    pub data_length: EnetUint16,
}
pub type ENetProtocolSendUnsequenced = _ENetProtocolSendUnsequenced;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct _ENetProtocolSendFragment {
    pub header: ENetProtocolCommandHeader,
    pub startSequenceNumber: EnetUint16,
    pub data_length: EnetUint16,
    pub fragmentCount: EnetUint32,
    pub fragmentNumber: EnetUint32,
    pub totalLength: EnetUint32,
    pub fragmentOffset: EnetUint32,
}
pub type ENetProtocolSendFragment = _ENetProtocolSendFragment;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union _ENetProtocol {
    pub header: ENetProtocolCommandHeader,
    pub acknowledge: ENetProtocolAcknowledge,
    pub connect: ENetProtocolConnect,
    pub verifyConnect: ENetProtocolVerifyConnect,
    pub disconnect: ENetProtocolDisconnect,
    pub ping: ENetProtocolPing,
    pub sendReliable: ENetProtocolSendReliable,
    pub sendUnreliable: ENetProtocolSendUnreliable,
    pub sendUnsequenced: ENetProtocolSendUnsequenced,
    pub sendFragment: ENetProtocolSendFragment,
    pub bandwidthLimit: ENetProtocolBandwidthLimit,
    pub throttleConfigure: ENetProtocolThrottleConfigure,
}
pub type ENetProtocol = _ENetProtocol;
/* * 
 @file  list.h
 @brief ENet list management 
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetListNode {
    pub next: *mut _ENetListNode,
    pub previous: *mut _ENetListNode,
}
pub type ENetListNode = _ENetListNode;
pub type ENetListIterator = *mut ENetListNode;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetList {
    pub sentinel: ENetListNode,
}
pub type ENetList = _ENetList;
/* * An ENet host for communicating with peers.
  *
  * No fields should be modified unless otherwise stated.

    @sa enet_host_create()
    @sa enet_host_destroy()
    @sa enet_host_connect()
    @sa enet_host_service()
    @sa enet_host_flush()
    @sa enet_host_broadcast()
    @sa enet_host_compress()
    @sa enet_host_compress_with_range_coder()
    @sa enet_host_channel_limit()
    @sa enet_host_bandwidth_limit()
    @sa enet_host_bandwidth_throttle()
  */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetHost {
    pub socket: ENetSocket,
    pub address: ENetAddress,
    pub incomingBandwidth: EnetUint32,
    pub outgoingBandwidth: EnetUint32,
    pub bandwidthThrottleEpoch: EnetUint32,
    pub mtu: EnetUint32,
    pub randomSeed: EnetUint32,
    pub recalculateBandwidthLimits: libc::c_int,
    pub peers: *mut ENetPeer,
    pub peerCount: SizeT,
    pub channelLimit: SizeT,
    pub serviceTime: EnetUint32,
    pub dispatchQueue: ENetList,
    pub continueSending: libc::c_int,
    pub packetSize: SizeT,
    pub headerFlags: EnetUint16,
    pub commands: [ENetProtocol; 32],
    pub commandCount: SizeT,
    pub buffers: [ENetBuffer; 65],
    pub buffer_count: SizeT,
    pub checksum: ENetChecksumCallback,
    pub compressor: ENetCompressor,
    pub packetData: [[enet_uint8; 4096]; 2],
    pub receivedAddress: ENetAddress,
    pub receivedData: *mut enet_uint8,
    pub receivedDataLength: SizeT,
    pub totalSentData: EnetUint32,
    pub totalSentPackets: EnetUint32,
    pub totalReceivedData: EnetUint32,
    pub totalReceivedPackets: EnetUint32,
    pub intercept: ENetInterceptCallback,
    pub connectedPeers: SizeT,
    pub bandwidthLimitedPeers: SizeT,
    pub duplicatePeers: SizeT,
    pub maximumPacketSize: SizeT,
    pub maximumWaitingData: SizeT,
}
/* * Callback for intercepting received raw UDP packets. Should return 1 to intercept, 0 to ignore, or -1 to propagate an error. */
pub type ENetInterceptCallback
    =
    Option<unsafe extern "C" fn(_: *mut _ENetHost, _: *mut _ENetEvent)
               -> libc::c_int>;
/* *
 * An ENet event as returned by enet_host_service().
   
   @sa enet_host_service
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetEvent {
    pub type_0: ENetEventType,
    pub peer: *mut ENetPeer,
    pub channelID: enet_uint8,
    pub data: EnetUint32,
    pub packet: *mut ENetPacket,
}
pub type ENetPacket = _ENetPacket;
/* *
 * ENet packet structure.
 *
 * An ENet data packet that may be sent to or received from a peer. The shown 
 * fields should only be read and never modified. The data field contains the 
 * allocated data for the packet. The data_length fields specifies the length 
 * of the allocated data.  The flags field is either 0 (specifying no flags), 
 * or a bitwise-or of any combination of the following flags:
 *
 *    ENET_PACKET_FLAG_RELIABLE - packet must be received by the target peer
 *    and resend attempts should be made until the packet is delivered
 *
 *    ENET_PACKET_FLAG_UNSEQUENCED - packet will not be sequenced with other packets 
 *    (not supported for reliable packets)
 *
 *    ENET_PACKET_FLAG_NO_ALLOCATE - packet will not allocate data, and user must supply it instead
 *
 *    ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT - packet will be fragmented using unreliable
 *    (instead of reliable) sends if it exceeds the MTU
 *
 *    ENET_PACKET_FLAG_SENT - whether the packet has been sent from all queues it has been entered into
   @sa ENetPacketFlag
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetPacket {
    pub referenceCount: SizeT,
    pub flags: EnetUint32,
    pub data: *mut enet_uint8,
    pub data_length: SizeT,
    pub freeCallback: ENetPacketFreeCallback,
    pub userData: *mut libc::c_void,
}
pub type ENetPacketFreeCallback
    =
    Option<unsafe extern "C" fn(_: *mut _ENetPacket) -> ()>;
pub type ENetPeer = _ENetPeer;
/* *
 * An ENet peer which data packets may be sent or received from. 
 *
 * No fields should be modified unless otherwise specified. 
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetPeer {
    pub dispatchList: ENetListNode,
    pub host: *mut _ENetHost,
    pub outgoingPeerID: EnetUint16,
    pub incomingPeerID: EnetUint16,
    pub connectID: EnetUint32,
    pub outgoingSessionID: enet_uint8,
    pub incomingSessionID: enet_uint8,
    pub address: ENetAddress,
    pub data: *mut libc::c_void,
    pub state: ENetPeerState,
    pub channels: *mut ENetChannel,
    pub channelCount: SizeT,
    pub incomingBandwidth: EnetUint32,
    pub outgoingBandwidth: EnetUint32,
    pub incomingBandwidthThrottleEpoch: EnetUint32,
    pub outgoingBandwidthThrottleEpoch: EnetUint32,
    pub incomingDataTotal: EnetUint32,
    pub outgoingDataTotal: EnetUint32,
    pub lastSendTime: EnetUint32,
    pub lastReceiveTime: EnetUint32,
    pub nextTimeout: EnetUint32,
    pub earliestTimeout: EnetUint32,
    pub packetLossEpoch: EnetUint32,
    pub packetsSent: EnetUint32,
    pub packetsLost: EnetUint32,
    pub packetLoss: EnetUint32,
    pub packetLossVariance: EnetUint32,
    pub packetThrottle: EnetUint32,
    pub packetThrottleLimit: EnetUint32,
    pub packetThrottleCounter: EnetUint32,
    pub packetThrottleEpoch: EnetUint32,
    pub packetThrottleAcceleration: EnetUint32,
    pub packetThrottleDeceleration: EnetUint32,
    pub packetThrottleInterval: EnetUint32,
    pub pingInterval: EnetUint32,
    pub timeoutLimit: EnetUint32,
    pub timeoutMinimum: EnetUint32,
    pub timeoutMaximum: EnetUint32,
    pub lastRoundTripTime: EnetUint32,
    pub lowestRoundTripTime: EnetUint32,
    pub lastRoundTripTimeVariance: EnetUint32,
    pub highestRoundTripTimeVariance: EnetUint32,
    pub roundTripTime: EnetUint32,
    pub roundTripTimeVariance: EnetUint32,
    pub mtu: EnetUint32,
    pub windowSize: EnetUint32,
    pub reliableDataInTransit: EnetUint32,
    pub outgoingReliableSequenceNumber: EnetUint16,
    pub acknowledgements: ENetList,
    pub sentReliableCommands: ENetList,
    pub sentUnreliableCommands: ENetList,
    pub outgoingReliableCommands: ENetList,
    pub outgoingUnreliableCommands: ENetList,
    pub dispatchedCommands: ENetList,
    pub needsDispatch: libc::c_int,
    pub incomingUnsequencedGroup: EnetUint16,
    pub outgoingUnsequencedGroup: EnetUint16,
    pub unsequencedWindow: [EnetUint32; 32],
    pub eventData: EnetUint32,
    pub totalWaitingData: SizeT,
}
pub type ENetChannel = _ENetChannel;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetChannel {
    pub outgoingReliableSequenceNumber: EnetUint16,
    pub outgoingUnreliableSequenceNumber: EnetUint16,
    pub usedReliableWindows: EnetUint16,
    pub reliableWindows: [EnetUint16; 16],
    pub incomingReliableSequenceNumber: EnetUint16,
    pub incomingUnreliableSequenceNumber: EnetUint16,
    pub incomingReliableCommands: ENetList,
    pub incomingUnreliableCommands: ENetList,
}
pub type ENetPeerState = _ENetPeerState;
pub type _ENetPeerState = libc::c_uint;
pub const ENET_PEER_STATE_ZOMBIE: _ENetPeerState = 9;
pub const ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT: _ENetPeerState = 8;
pub const ENET_PEER_STATE_DISCONNECTING: _ENetPeerState = 7;
pub const ENET_PEER_STATE_DISCONNECT_LATER: _ENetPeerState = 6;
pub const ENET_PEER_STATE_CONNECTED: _ENetPeerState = 5;
pub const ENET_PEER_STATE_CONNECTION_SUCCEEDED: _ENetPeerState = 4;
pub const ENET_PEER_STATE_CONNECTION_PENDING: _ENetPeerState = 3;
pub const ENET_PEER_STATE_ACKNOWLEDGING_CONNECT: _ENetPeerState = 2;
pub const ENET_PEER_STATE_CONNECTING: _ENetPeerState = 1;
pub const ENET_PEER_STATE_DISCONNECTED: _ENetPeerState = 0;
pub type ENetAddress = _ENetAddress;
/* *
 * Portable internet address structure. 
 *
 * The host must be specified in network byte-order, and the port must be in host 
 * byte-order. The constant ENET_HOST_ANY may be used to specify the default 
 * server host. The constant ENET_HOST_BROADCAST may be used to specify the
 * broadcast address (255.255.255.255).  This makes sense for enet_host_connect,
 * but not for enet_host_create.  Once a server responds to a broadcast, the
 * address is updated from ENET_HOST_BROADCAST to the server's actual IP address.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetAddress {
    pub host: EnetUint32,
    pub port: EnetUint16,
}
pub type ENetEventType = _ENetEventType;
/* *
 * An ENet event type, as specified in @ref ENetEvent.
 */
pub type _ENetEventType = libc::c_uint;
/* * a packet has been received from a peer.  The peer field specifies the
     * peer which sent the packet.  The channelID field specifies the channel
     * number upon which the packet was received.  The packet field contains
     * the packet that was received; this packet must be destroyed with
     * enet_packet_destroy after use.
     */
pub const ENET_EVENT_TYPE_RECEIVE: _ENetEventType = 3;
/* * a peer has disconnected.  This event is generated on a successful 
     * completion of a disconnect initiated by enet_peer_disconnect, if 
     * a peer has timed out, or if a connection request intialized by 
     * enet_host_connect has timed out.  The peer field contains the peer 
     * which disconnected. The data field contains user supplied data 
     * describing the disconnection, or 0, if none is available.
     */
pub const ENET_EVENT_TYPE_DISCONNECT: _ENetEventType = 2;
/* * a connection request initiated by enet_host_connect has completed.  
     * The peer field contains the peer which successfully connected. 
     */
pub const ENET_EVENT_TYPE_CONNECT: _ENetEventType = 1;
/* * no event occurred within the specified time limit */
pub const ENET_EVENT_TYPE_NONE: _ENetEventType = 0;
pub type ENetCompressor = _ENetCompressor;
/* * An ENet packet compressor for compressing UDP packets before socket sends or receives.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetCompressor {
    pub context: *mut libc::c_void,
    pub compress: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: *const ENetBuffer, _: SizeT,
                                              _: SizeT, _: *mut enet_uint8,
                                              _: SizeT) -> SizeT>,
    pub decompress: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: *const enet_uint8,
                                                _: SizeT, _: *mut enet_uint8,
                                                _: SizeT) -> SizeT>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
}
/* * Callback that computes the checksum of the data held in buffers[0:buffer_count-1] */
pub type ENetChecksumCallback
    =
    Option<unsafe extern "C" fn(_: *const ENetBuffer, _: SizeT)
               -> EnetUint32>;
/* *
 * Packet flag bit constants.
 *
 * The host must be specified in network byte-order, and the port must be in
 * host byte-order. The constant ENET_HOST_ANY may be used to specify the
 * default server host.
 
   @sa ENetPacket
*/
pub type _ENetPacketFlag = libc::c_uint;
/* * whether the packet has been sent from all queues it has been entered into */
pub const ENET_PACKET_FLAG_SENT: _ENetPacketFlag = 256;
/* * packet will be fragmented using unreliable (instead of reliable) sends
     * if it exceeds the MTU */
pub const ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT: _ENetPacketFlag = 8;
/* * packet will not allocate data, and user must supply it instead */
pub const ENET_PACKET_FLAG_NO_ALLOCATE: _ENetPacketFlag = 4;
/* * packet will not be sequenced with other packets
     * not supported for reliable packets
     */
pub const ENET_PACKET_FLAG_UNSEQUENCED: _ENetPacketFlag = 2;
/* * packet must be received by the target peer and resend attempts should be
     * made until the packet is delivered */
pub const ENET_PACKET_FLAG_RELIABLE: _ENetPacketFlag = 1;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetAcknowledgement {
    pub acknowledgementList: ENetListNode,
    pub sentTime: EnetUint32,
    pub command: ENetProtocol,
}
pub type ENetAcknowledgement = _ENetAcknowledgement;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetOutgoingCommand {
    pub outgoingCommandList: ENetListNode,
    pub reliableSequenceNumber: EnetUint16,
    pub unreliableSequenceNumber: EnetUint16,
    pub sentTime: EnetUint32,
    pub roundTripTimeout: EnetUint32,
    pub roundTripTimeoutLimit: EnetUint32,
    pub fragmentOffset: EnetUint32,
    pub fragmentLength: EnetUint16,
    pub sendAttempts: EnetUint16,
    pub command: ENetProtocol,
    pub packet: *mut ENetPacket,
}
pub type ENetOutgoingCommand = _ENetOutgoingCommand;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetIncomingCommand {
    pub incomingCommandList: ENetListNode,
    pub reliableSequenceNumber: EnetUint16,
    pub unreliableSequenceNumber: EnetUint16,
    pub command: ENetProtocol,
    pub fragmentCount: EnetUint32,
    pub fragmentsRemaining: EnetUint32,
    pub fragments: *mut EnetUint32,
    pub packet: *mut ENetPacket,
}
pub type ENetIncomingCommand = _ENetIncomingCommand;
pub type Unnamed0 = libc::c_uint;
pub const ENET_PEER_FREE_RELIABLE_WINDOWS: Unnamed0 = 8;
pub const ENET_PEER_RELIABLE_WINDOW_SIZE: Unnamed0 = 4096;
pub const ENET_PEER_RELIABLE_WINDOWS: Unnamed0 = 16;
pub const ENET_PEER_FREE_UNSEQUENCED_WINDOWS: Unnamed0 = 32;
pub const ENET_PEER_UNSEQUENCED_WINDOW_SIZE: Unnamed0 = 1024;
pub const ENET_PEER_UNSEQUENCED_WINDOWS: Unnamed0 = 64;
pub const ENET_PEER_PING_INTERVAL: Unnamed0 = 500;
pub const ENET_PEER_TIMEOUT_MAXIMUM: Unnamed0 = 30000;
pub const ENET_PEER_TIMEOUT_MINIMUM: Unnamed0 = 5000;
pub const ENET_PEER_TIMEOUT_LIMIT: Unnamed0 = 32;
pub const ENET_PEER_WINDOW_SIZE_SCALE: Unnamed0 = 65536;
pub const ENET_PEER_PACKET_LOSS_INTERVAL: Unnamed0 = 10000;
pub const ENET_PEER_PACKET_LOSS_SCALE: Unnamed0 = 65536;
pub const ENET_PEER_PACKET_THROTTLE_INTERVAL: Unnamed0 = 5000;
pub const ENET_PEER_PACKET_THROTTLE_DECELERATION: Unnamed0 = 2;
pub const ENET_PEER_PACKET_THROTTLE_ACCELERATION: Unnamed0 = 2;
pub const ENET_PEER_PACKET_THROTTLE_COUNTER: Unnamed0 = 7;
pub const ENET_PEER_PACKET_THROTTLE_SCALE: Unnamed0 = 32;
pub const ENET_PEER_DEFAULT_PACKET_THROTTLE: Unnamed0 = 32;
pub const ENET_PEER_DEFAULT_ROUND_TRIP_TIME: Unnamed0 = 500;
pub const ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA: Unnamed0 = 33554432;
pub const ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE: Unnamed0 = 33554432;
pub const ENET_HOST_DEFAULT_MTU: Unnamed0 = 1400;
pub const ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL: Unnamed0 = 1000;
pub const ENET_HOST_SEND_BUFFER_SIZE: Unnamed0 = 262144;
pub const ENET_HOST_RECEIVE_BUFFER_SIZE: Unnamed0 = 262144;
pub type ENetHost = _ENetHost;
/* * 
 @file  peer.c
 @brief ENet peer management functions
*/
/* * @defgroup peer ENet peer functions 
    @{
*/
/* * Configures throttle parameter for a peer.

    Unreliable packets are dropped by ENet in response to the varying conditions
    of the Internet connection to the peer.  The throttle represents a probability
    that an unreliable packet should not be dropped and thus sent by ENet to the peer.
    The lowest mean round trip time from the sending of a reliable packet to the
    receipt of its acknowledgement is measured over an amount of time specified by
    the interval parameter in milliseconds.  If a measured round trip time happens to
    be significantly less than the mean round trip time measured over the interval, 
    then the throttle probability is increased to allow more traffic by an amount
    specified in the acceleration parameter, which is a ratio to the ENET_PEER_PACKET_THROTTLE_SCALE
    constant.  If a measured round trip time happens to be significantly greater than
    the mean round trip time measured over the interval, then the throttle probability
    is decreased to limit traffic by an amount specified in the deceleration parameter, which
    is a ratio to the ENET_PEER_PACKET_THROTTLE_SCALE constant.  When the throttle has
    a value of ENET_PEER_PACKET_THROTTLE_SCALE, no unreliable packets are dropped by 
    ENet, and so 100% of all unreliable packets will be sent.  When the throttle has a
    value of 0, all unreliable packets are dropped by ENet, and so 0% of all unreliable
    packets will be sent.  Intermediate values for the throttle represent intermediate
    probabilities between 0% and 100% of unreliable packets being sent.  The bandwidth
    limits of the local and foreign hosts are taken into account to determine a 
    sensible limit for the throttle probability above which it should not raise even in
    the best of conditions.

    @param peer peer to configure 
    @param interval interval, in milliseconds, over which to measure lowest mean RTT; the default value is ENET_PEER_PACKET_THROTTLE_INTERVAL.
    @param acceleration rate at which to increase the throttle probability as mean RTT declines
    @param deceleration rate at which to decrease the throttle probability as mean RTT increases
*/
#[no_mangle]
pub unsafe extern "C" fn enet_peer_throttle_configure(mut peer: *mut ENetPeer,
                                                      mut interval:
                                                          EnetUint32,
                                                      mut acceleration:
                                                          EnetUint32,
                                                      mut deceleration:
                                                          EnetUint32) {
    let mut command: ENetProtocol =
        _ENetProtocol{header:
                          ENetProtocolCommandHeader{command: 0,
                                                    channelID: 0,
                                                    reliableSequenceNumber:
                                                        0,},};
    (*peer).packetThrottleInterval = interval;
    (*peer).packetThrottleAcceleration = acceleration;
    (*peer).packetThrottleDeceleration = deceleration;
    command.header.command =
        (ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE as libc::c_int |
             ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int) as
            enet_uint8;
    command.header.channelID = 0xffi32 as enet_uint8;
    command.throttleConfigure.packetThrottleInterval = htonl(interval);
    command.throttleConfigure.packetThrottleAcceleration =
        htonl(acceleration);
    command.throttleConfigure.packetThrottleDeceleration =
        htonl(deceleration);
    enet_peer_queue_outgoing_command(peer, &mut command, 0 as *mut ENetPacket,
                                     0i32 as EnetUint32,
                                     0i32 as EnetUint16);
}
#[no_mangle]
pub unsafe extern "C" fn enet_peer_throttle(mut peer: *mut ENetPeer,
                                            mut rtt: EnetUint32)
 -> libc::c_int {
    if (*peer).lastRoundTripTime <= (*peer).lastRoundTripTimeVariance {
        (*peer).packetThrottle = (*peer).packetThrottleLimit
    } else if rtt < (*peer).lastRoundTripTime {
        (*peer).packetThrottle =
            ((*peer).packetThrottle as
                 libc::c_uint).wrapping_add((*peer).packetThrottleAcceleration)
                as EnetUint32 as EnetUint32;
        if (*peer).packetThrottle > (*peer).packetThrottleLimit {
            (*peer).packetThrottle = (*peer).packetThrottleLimit
        }
        return 1i32
    } else {
        if rtt >
               (*peer).lastRoundTripTime.wrapping_add((2i32 as
                                                           libc::c_uint).wrapping_mul((*peer).lastRoundTripTimeVariance))
           {
            if (*peer).packetThrottle > (*peer).packetThrottleDeceleration {
                (*peer).packetThrottle =
                    ((*peer).packetThrottle as
                         libc::c_uint).wrapping_sub((*peer).packetThrottleDeceleration)
                        as EnetUint32 as EnetUint32
            } else { (*peer).packetThrottle = 0i32 as EnetUint32 }
            return -1i32
        }
    }
    return 0i32;
}
/* * Queues a packet to be sent.
    @param peer destination for the packet
    @param channelID channel on which to send
    @param packet packet to send
    @retval 0 on success
    @retval < 0 on failure
*/
#[no_mangle]
pub unsafe extern "C" fn enet_peer_send(mut peer: *mut ENetPeer,
                                        mut channelID: enet_uint8,
                                        mut packet: *mut ENetPacket)
 -> libc::c_int {
    let mut channel: *mut ENetChannel =
        &mut *(*peer).channels.offset(channelID as isize) as *mut ENetChannel;
    let mut command: ENetProtocol =
        _ENetProtocol{header:
                          ENetProtocolCommandHeader{command: 0,
                                                    channelID: 0,
                                                    reliableSequenceNumber:
                                                        0,},};
    let mut fragmentLength: SizeT = 0;
    if (*peer).state as libc::c_uint !=
           ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint ||
           channelID as libc::c_ulong >= (*peer).channelCount ||
           (*packet).data_length > (*(*peer).host).maximumPacketSize {
        return -1i32
    }
    fragmentLength =
        ((*peer).mtu as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<ENetProtocolHeader>()
                                             as
                                             libc::c_ulong).wrapping_sub(::std::mem::size_of::<ENetProtocolSendFragment>()
                                                                             as
                                                                             libc::c_ulong);
    if (*(*peer).host).checksum.is_some() {
        fragmentLength =
            (fragmentLength as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<EnetUint32>()
                                                 as libc::c_ulong) as SizeT
                as SizeT
    }
    if (*packet).data_length > fragmentLength {
        let mut fragmentCount: EnetUint32 =
            (*packet).data_length.wrapping_add(fragmentLength).wrapping_sub(1i32
                                                                               as
                                                                               libc::c_ulong).wrapping_div(fragmentLength)
                as EnetUint32;
        let mut fragmentNumber: EnetUint32 = 0;
        let mut fragmentOffset: EnetUint32 = 0;
        let mut commandNumber: enet_uint8 = 0;
        let mut startSequenceNumber: EnetUint16 = 0;
        let mut fragments: ENetList =
            ENetList{sentinel:
                         ENetListNode{next: 0 as *mut _ENetListNode,
                                      previous: 0 as *mut _ENetListNode,},};
        let mut fragment: *mut ENetOutgoingCommand =
            0 as *mut ENetOutgoingCommand;
        if fragmentCount >
               ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as libc::c_int as
                   libc::c_uint {
            return -1i32
        }
        if (*packet).flags &
               (ENET_PACKET_FLAG_RELIABLE as libc::c_int |
                    ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as libc::c_int) as
                   libc::c_uint ==
               ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as libc::c_int as
                   libc::c_uint &&
               ((*channel).outgoingUnreliableSequenceNumber as libc::c_int) <
                   0xffffi32 {
            commandNumber =
                ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT as libc::c_int
                    as enet_uint8;
            startSequenceNumber =
                htons(((*channel).outgoingUnreliableSequenceNumber as
                           libc::c_int + 1i32) as Uint16T)
        } else {
            commandNumber =
                (ENET_PROTOCOL_COMMAND_SEND_FRAGMENT as libc::c_int |
                     ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int) as
                    enet_uint8;
            startSequenceNumber =
                htons(((*channel).outgoingReliableSequenceNumber as
                           libc::c_int + 1i32) as Uint16T)
        }
        enet_list_clear(&mut fragments);
        fragmentNumber = 0i32 as EnetUint32;
        fragmentOffset = 0i32 as EnetUint32;
        while (fragmentOffset as libc::c_ulong) < (*packet).data_length {
            if (*packet).data_length.wrapping_sub(fragmentOffset as
                                                     libc::c_ulong) <
                   fragmentLength {
                fragmentLength =
                    (*packet).data_length.wrapping_sub(fragmentOffset as
                                                          libc::c_ulong)
            }
            fragment =
                enet_malloc(::std::mem::size_of::<ENetOutgoingCommand>() as
                                libc::c_ulong) as *mut ENetOutgoingCommand;
            if fragment.is_null() {
                while !(fragments.sentinel.next ==
                            &mut fragments.sentinel as *mut ENetListNode) {
                    fragment =
                        enet_list_remove(fragments.sentinel.next) as
                            *mut ENetOutgoingCommand;
                    enet_free(fragment as *mut libc::c_void);
                }
                return -1i32
            }
            (*fragment).fragmentOffset = fragmentOffset;
            (*fragment).fragmentLength = fragmentLength as EnetUint16;
            (*fragment).packet = packet;
            (*fragment).command.header.command = commandNumber;
            (*fragment).command.header.channelID = channelID;
            (*fragment).command.sendFragment.startSequenceNumber =
                startSequenceNumber;
            (*fragment).command.sendFragment.data_length =
                htons(fragmentLength as Uint16T);
            (*fragment).command.sendFragment.fragmentCount =
                htonl(fragmentCount);
            (*fragment).command.sendFragment.fragmentNumber =
                htonl(fragmentNumber);
            (*fragment).command.sendFragment.totalLength =
                htonl((*packet).data_length as Uint32T);
            (*fragment).command.sendFragment.fragmentOffset =
                ntohl(fragmentOffset);
            enet_list_insert(&mut fragments.sentinel,
                             fragment as *mut libc::c_void);
            fragmentNumber = fragmentNumber.wrapping_add(1);
            fragmentOffset =
                (fragmentOffset as libc::c_ulong).wrapping_add(fragmentLength)
                    as EnetUint32 as EnetUint32
        }
        (*packet).referenceCount =
            ((*packet).referenceCount as
                 libc::c_ulong).wrapping_add(fragmentNumber as libc::c_ulong)
                as SizeT as SizeT;
        while !(fragments.sentinel.next ==
                    &mut fragments.sentinel as *mut ENetListNode) {
            fragment =
                enet_list_remove(fragments.sentinel.next) as
                    *mut ENetOutgoingCommand;
            enet_peer_setup_outgoing_command(peer, fragment);
        }
        return 0i32
    }
    command.header.channelID = channelID;
    if (*packet).flags &
           (ENET_PACKET_FLAG_RELIABLE as libc::c_int |
                ENET_PACKET_FLAG_UNSEQUENCED as libc::c_int) as libc::c_uint
           == ENET_PACKET_FLAG_UNSEQUENCED as libc::c_int as libc::c_uint {
        command.header.command =
            (ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as libc::c_int |
                 ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as libc::c_int) as
                enet_uint8;
        command.sendUnsequenced.data_length =
            htons((*packet).data_length as Uint16T)
    } else if 0 !=
                  (*packet).flags &
                      ENET_PACKET_FLAG_RELIABLE as libc::c_int as libc::c_uint
                  ||
                  (*channel).outgoingUnreliableSequenceNumber as libc::c_int
                      >= 0xffffi32 {
        command.header.command =
            (ENET_PROTOCOL_COMMAND_SEND_RELIABLE as libc::c_int |
                 ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int) as
                enet_uint8;
        command.sendReliable.data_length =
            htons((*packet).data_length as Uint16T)
    } else {
        command.header.command =
            ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE as libc::c_int as
                enet_uint8;
        command.sendUnreliable.data_length =
            htons((*packet).data_length as Uint16T)
    }
    if enet_peer_queue_outgoing_command(peer, &mut command, packet,
                                        0i32 as EnetUint32,
                                        (*packet).data_length as
                                            EnetUint16).is_null() {
        return -1i32
    }
    return 0i32;
}
/* * Attempts to dequeue any incoming queued packet.
    @param peer peer to dequeue packets from
    @param channelID holds the channel ID of the channel the packet was received on success
    @returns a pointer to the packet, or NULL if there are no available incoming queued packets
*/
#[no_mangle]
pub unsafe extern "C" fn enet_peer_receive(mut peer: *mut ENetPeer,
                                           mut channelID: *mut enet_uint8)
 -> *mut ENetPacket {
    let mut incomingCommand: *mut ENetIncomingCommand =
        0 as *mut ENetIncomingCommand;
    let mut packet: *mut ENetPacket = 0 as *mut ENetPacket;
    if (*peer).dispatchedCommands.sentinel.next ==
           &mut (*peer).dispatchedCommands.sentinel as *mut ENetListNode {
        return 0 as *mut ENetPacket
    }
    incomingCommand =
        enet_list_remove((*peer).dispatchedCommands.sentinel.next) as
            *mut ENetIncomingCommand;
    if !channelID.is_null() {
        *channelID = (*incomingCommand).command.header.channelID
    }
    packet = (*incomingCommand).packet;
    (*packet).referenceCount = (*packet).referenceCount.wrapping_sub(1);
    if !(*incomingCommand).fragments.is_null() {
        enet_free((*incomingCommand).fragments as *mut libc::c_void);
    }
    enet_free(incomingCommand as *mut libc::c_void);
    (*peer).totalWaitingData =
        ((*peer).totalWaitingData as
             libc::c_ulong).wrapping_sub((*packet).data_length) as SizeT as
            SizeT;
    return packet;
}
unsafe extern "C" fn enet_peer_reset_outgoing_commands(mut queue:
                                                           *mut ENetList) {
    let mut outgoingCommand: *mut ENetOutgoingCommand =
        0 as *mut ENetOutgoingCommand;
    while !((*queue).sentinel.next ==
                &mut (*queue).sentinel as *mut ENetListNode) {
        outgoingCommand =
            enet_list_remove((*queue).sentinel.next) as
                *mut ENetOutgoingCommand;
        if !(*outgoingCommand).packet.is_null() {
            (*(*outgoingCommand).packet).referenceCount =
                (*(*outgoingCommand).packet).referenceCount.wrapping_sub(1);
            if (*(*outgoingCommand).packet).referenceCount ==
                   0i32 as libc::c_ulong {
                enet_packet_destroy((*outgoingCommand).packet);
            }
        }
        enet_free(outgoingCommand as *mut libc::c_void);
    };
}
unsafe extern "C" fn enet_peer_remove_incoming_commands(mut queue:
                                                            *mut ENetList,
                                                        mut startCommand:
                                                            ENetListIterator,
                                                        mut endCommand:
                                                            ENetListIterator) {
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    currentCommand = startCommand;
    while currentCommand != endCommand {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        currentCommand = (*currentCommand).next;
        enet_list_remove(&mut (*incomingCommand).incomingCommandList);
        if !(*incomingCommand).packet.is_null() {
            (*(*incomingCommand).packet).referenceCount =
                (*(*incomingCommand).packet).referenceCount.wrapping_sub(1);
            if (*(*incomingCommand).packet).referenceCount ==
                   0i32 as libc::c_ulong {
                enet_packet_destroy((*incomingCommand).packet);
            }
        }
        if !(*incomingCommand).fragments.is_null() {
            enet_free((*incomingCommand).fragments as *mut libc::c_void);
        }
        enet_free(incomingCommand as *mut libc::c_void);
    };
}
unsafe extern "C" fn enet_peer_reset_incoming_commands(mut queue:
                                                           *mut ENetList) {
    enet_peer_remove_incoming_commands(queue, (*queue).sentinel.next,
                                       &mut (*queue).sentinel);
}
#[no_mangle]
pub unsafe extern "C" fn enet_peer_reset_queues(mut peer: *mut ENetPeer) {
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    if 0 != (*peer).needsDispatch {
        enet_list_remove(&mut (*peer).dispatchList);
        (*peer).needsDispatch = 0i32
    }
    while !((*peer).acknowledgements.sentinel.next ==
                &mut (*peer).acknowledgements.sentinel as *mut ENetListNode) {
        enet_free(enet_list_remove((*peer).acknowledgements.sentinel.next));
    }
    enet_peer_reset_outgoing_commands(&mut (*peer).sentReliableCommands);
    enet_peer_reset_outgoing_commands(&mut (*peer).sentUnreliableCommands);
    enet_peer_reset_outgoing_commands(&mut (*peer).outgoingReliableCommands);
    enet_peer_reset_outgoing_commands(&mut (*peer).outgoingUnreliableCommands);
    enet_peer_reset_incoming_commands(&mut (*peer).dispatchedCommands);
    if !(*peer).channels.is_null() &&
           (*peer).channelCount > 0i32 as libc::c_ulong {
        channel = (*peer).channels;
        while channel <
                  &mut *(*peer).channels.offset((*peer).channelCount as isize)
                      as *mut ENetChannel {
            enet_peer_reset_incoming_commands(&mut (*channel).incomingReliableCommands);
            enet_peer_reset_incoming_commands(&mut (*channel).incomingUnreliableCommands);
            channel = channel.offset(1isize)
        }
        enet_free((*peer).channels as *mut libc::c_void);
    }
    (*peer).channels = 0 as *mut ENetChannel;
    (*peer).channelCount = 0i32 as SizeT;
}
#[no_mangle]
pub unsafe extern "C" fn enet_peer_on_connect(mut peer: *mut ENetPeer) {
    if (*peer).state as libc::c_uint !=
           ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint &&
           (*peer).state as libc::c_uint !=
               ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
       {
        if (*peer).incomingBandwidth != 0i32 as libc::c_uint {
            (*(*peer).host).bandwidthLimitedPeers =
                (*(*peer).host).bandwidthLimitedPeers.wrapping_add(1)
        }
        (*(*peer).host).connectedPeers =
            (*(*peer).host).connectedPeers.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn enet_peer_on_disconnect(mut peer: *mut ENetPeer) {
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
       {
        if (*peer).incomingBandwidth != 0i32 as libc::c_uint {
            (*(*peer).host).bandwidthLimitedPeers =
                (*(*peer).host).bandwidthLimitedPeers.wrapping_sub(1)
        }
        (*(*peer).host).connectedPeers =
            (*(*peer).host).connectedPeers.wrapping_sub(1)
    };
}
/* * Forcefully disconnects a peer.
    @param peer peer to forcefully disconnect
    @remarks The foreign host represented by the peer is not notified of the disconnection and will timeout
    on its connection to the local host.
*/
#[no_mangle]
pub unsafe extern "C" fn enet_peer_reset(mut peer: *mut ENetPeer) {
    enet_peer_on_disconnect(peer);
    (*peer).outgoingPeerID =
        ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int as EnetUint16;
    (*peer).connectID = 0i32 as EnetUint32;
    (*peer).state = ENET_PEER_STATE_DISCONNECTED;
    (*peer).incomingBandwidth = 0i32 as EnetUint32;
    (*peer).outgoingBandwidth = 0i32 as EnetUint32;
    (*peer).incomingBandwidthThrottleEpoch = 0i32 as EnetUint32;
    (*peer).outgoingBandwidthThrottleEpoch = 0i32 as EnetUint32;
    (*peer).incomingDataTotal = 0i32 as EnetUint32;
    (*peer).outgoingDataTotal = 0i32 as EnetUint32;
    (*peer).lastSendTime = 0i32 as EnetUint32;
    (*peer).lastReceiveTime = 0i32 as EnetUint32;
    (*peer).nextTimeout = 0i32 as EnetUint32;
    (*peer).earliestTimeout = 0i32 as EnetUint32;
    (*peer).packetLossEpoch = 0i32 as EnetUint32;
    (*peer).packetsSent = 0i32 as EnetUint32;
    (*peer).packetsLost = 0i32 as EnetUint32;
    (*peer).packetLoss = 0i32 as EnetUint32;
    (*peer).packetLossVariance = 0i32 as EnetUint32;
    (*peer).packetThrottle =
        ENET_PEER_DEFAULT_PACKET_THROTTLE as libc::c_int as EnetUint32;
    (*peer).packetThrottleLimit =
        ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as EnetUint32;
    (*peer).packetThrottleCounter = 0i32 as EnetUint32;
    (*peer).packetThrottleEpoch = 0i32 as EnetUint32;
    (*peer).packetThrottleAcceleration =
        ENET_PEER_PACKET_THROTTLE_ACCELERATION as libc::c_int as EnetUint32;
    (*peer).packetThrottleDeceleration =
        ENET_PEER_PACKET_THROTTLE_DECELERATION as libc::c_int as EnetUint32;
    (*peer).packetThrottleInterval =
        ENET_PEER_PACKET_THROTTLE_INTERVAL as libc::c_int as EnetUint32;
    (*peer).pingInterval =
        ENET_PEER_PING_INTERVAL as libc::c_int as EnetUint32;
    (*peer).timeoutLimit =
        ENET_PEER_TIMEOUT_LIMIT as libc::c_int as EnetUint32;
    (*peer).timeoutMinimum =
        ENET_PEER_TIMEOUT_MINIMUM as libc::c_int as EnetUint32;
    (*peer).timeoutMaximum =
        ENET_PEER_TIMEOUT_MAXIMUM as libc::c_int as EnetUint32;
    (*peer).lastRoundTripTime =
        ENET_PEER_DEFAULT_ROUND_TRIP_TIME as libc::c_int as EnetUint32;
    (*peer).lowestRoundTripTime =
        ENET_PEER_DEFAULT_ROUND_TRIP_TIME as libc::c_int as EnetUint32;
    (*peer).lastRoundTripTimeVariance = 0i32 as EnetUint32;
    (*peer).highestRoundTripTimeVariance = 0i32 as EnetUint32;
    (*peer).roundTripTime =
        ENET_PEER_DEFAULT_ROUND_TRIP_TIME as libc::c_int as EnetUint32;
    (*peer).roundTripTimeVariance = 0i32 as EnetUint32;
    (*peer).mtu = (*(*peer).host).mtu;
    (*peer).reliableDataInTransit = 0i32 as EnetUint32;
    (*peer).outgoingReliableSequenceNumber = 0i32 as EnetUint16;
    (*peer).windowSize =
        ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as EnetUint32;
    (*peer).incomingUnsequencedGroup = 0i32 as EnetUint16;
    (*peer).outgoingUnsequencedGroup = 0i32 as EnetUint16;
    (*peer).eventData = 0i32 as EnetUint32;
    (*peer).totalWaitingData = 0i32 as SizeT;
    memset((*peer).unsequencedWindow.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[EnetUint32; 32]>() as libc::c_ulong);
    enet_peer_reset_queues(peer);
}
/* * Sends a ping request to a peer.
    @param peer destination for the ping request
    @remarks ping requests factor into the mean round trip time as designated by the 
    roundTripTime field in the ENetPeer structure.  ENet automatically pings all connected
    peers at regular intervals, however, this function may be called to ensure more
    frequent ping requests.
*/
#[no_mangle]
pub unsafe extern "C" fn enet_peer_ping(mut peer: *mut ENetPeer) {
    let mut command: ENetProtocol =
        _ENetProtocol{header:
                          ENetProtocolCommandHeader{command: 0,
                                                    channelID: 0,
                                                    reliableSequenceNumber:
                                                        0,},};
    if (*peer).state as libc::c_uint !=
           ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint {
        return
    }
    command.header.command =
        (ENET_PROTOCOL_COMMAND_PING as libc::c_int |
             ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int) as
            enet_uint8;
    command.header.channelID = 0xffi32 as enet_uint8;
    enet_peer_queue_outgoing_command(peer, &mut command, 0 as *mut ENetPacket,
                                     0i32 as EnetUint32,
                                     0i32 as EnetUint16);
}
/* * Sets the interval at which pings will be sent to a peer. 
    
    Pings are used both to monitor the liveness of the connection and also to dynamically
    adjust the throttle during periods of low traffic so that the throttle has reasonable
    responsiveness during traffic spikes.

    @param peer the peer to adjust
    @param pingInterval the interval at which to send pings; defaults to ENET_PEER_PING_INTERVAL if 0
*/
#[no_mangle]
pub unsafe extern "C" fn enet_peer_ping_interval(mut peer: *mut ENetPeer,
                                                 mut pingInterval:
                                                     EnetUint32) {
    (*peer).pingInterval =
        if 0 != pingInterval {
            pingInterval
        } else { ENET_PEER_PING_INTERVAL as libc::c_int as libc::c_uint };
}
/* * Sets the timeout parameters for a peer.

    The timeout parameter control how and when a peer will timeout from a failure to acknowledge
    reliable traffic. Timeout values use an exponential backoff mechanism, where if a reliable
    packet is not acknowledge within some multiple of the average RTT plus a variance tolerance, 
    the timeout will be doubled until it reaches a set limit. If the timeout is thus at this
    limit and reliable packets have been sent but not acknowledged within a certain minimum time 
    period, the peer will be disconnected. Alternatively, if reliable packets have been sent
    but not acknowledged for a certain maximum time period, the peer will be disconnected regardless
    of the current timeout limit value.
    
    @param peer the peer to adjust
    @param timeoutLimit the timeout limit; defaults to ENET_PEER_TIMEOUT_LIMIT if 0
    @param timeoutMinimum the timeout minimum; defaults to ENET_PEER_TIMEOUT_MINIMUM if 0
    @param timeoutMaximum the timeout maximum; defaults to ENET_PEER_TIMEOUT_MAXIMUM if 0
*/
#[no_mangle]
pub unsafe extern "C" fn enet_peer_timeout(mut peer: *mut ENetPeer,
                                           mut timeoutLimit: EnetUint32,
                                           mut timeoutMinimum: EnetUint32,
                                           mut timeoutMaximum: EnetUint32) {
    (*peer).timeoutLimit =
        if 0 != timeoutLimit {
            timeoutLimit
        } else { ENET_PEER_TIMEOUT_LIMIT as libc::c_int as libc::c_uint };
    (*peer).timeoutMinimum =
        if 0 != timeoutMinimum {
            timeoutMinimum
        } else { ENET_PEER_TIMEOUT_MINIMUM as libc::c_int as libc::c_uint };
    (*peer).timeoutMaximum =
        if 0 != timeoutMaximum {
            timeoutMaximum
        } else { ENET_PEER_TIMEOUT_MAXIMUM as libc::c_int as libc::c_uint };
}
/* * Force an immediate disconnection from a peer.
    @param peer peer to disconnect
    @param data data describing the disconnection
    @remarks No ENET_EVENT_DISCONNECT event will be generated. The foreign peer is not
    guaranteed to receive the disconnect notification, and is reset immediately upon
    return from this function.
*/
#[no_mangle]
pub unsafe extern "C" fn enet_peer_disconnect_now(mut peer: *mut ENetPeer,
                                                  mut data: EnetUint32) {
    let mut command: ENetProtocol =
        _ENetProtocol{header:
                          ENetProtocolCommandHeader{command: 0,
                                                    channelID: 0,
                                                    reliableSequenceNumber:
                                                        0,},};
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint {
        return
    }
    if (*peer).state as libc::c_uint !=
           ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint &&
           (*peer).state as libc::c_uint !=
               ENET_PEER_STATE_DISCONNECTING as libc::c_int as libc::c_uint {
        enet_peer_reset_queues(peer);
        command.header.command =
            (ENET_PROTOCOL_COMMAND_DISCONNECT as libc::c_int |
                 ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as libc::c_int) as
                enet_uint8;
        command.header.channelID = 0xffi32 as enet_uint8;
        command.disconnect.data = htonl(data);
        enet_peer_queue_outgoing_command(peer, &mut command,
                                         0 as *mut ENetPacket,
                                         0i32 as EnetUint32,
                                         0i32 as EnetUint16);
        enet_host_flush((*peer).host);
    }
    enet_peer_reset(peer);
}
/* * Request a disconnection from a peer.
    @param peer peer to request a disconnection
    @param data data describing the disconnection
    @remarks An ENET_EVENT_DISCONNECT event will be generated by enet_host_service()
    once the disconnection is complete.
*/
#[no_mangle]
pub unsafe extern "C" fn enet_peer_disconnect(mut peer: *mut ENetPeer,
                                              mut data: EnetUint32) {
    let mut command: ENetProtocol =
        _ENetProtocol{header:
                          ENetProtocolCommandHeader{command: 0,
                                                    channelID: 0,
                                                    reliableSequenceNumber:
                                                        0,},};
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_DISCONNECTING as libc::c_int as libc::c_uint ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT as libc::c_int as
                   libc::c_uint ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint {
        return
    }
    enet_peer_reset_queues(peer);
    command.header.command =
        ENET_PROTOCOL_COMMAND_DISCONNECT as libc::c_int as enet_uint8;
    command.header.channelID = 0xffi32 as enet_uint8;
    command.disconnect.data = htonl(data);
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
       {
        command.header.command =
            (command.header.command as libc::c_int |
                 ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int) as
                enet_uint8
    } else {
        command.header.command =
            (command.header.command as libc::c_int |
                 ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as libc::c_int) as
                enet_uint8
    }
    enet_peer_queue_outgoing_command(peer, &mut command, 0 as *mut ENetPacket,
                                     0i32 as EnetUint32,
                                     0i32 as EnetUint16);
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
       {
        enet_peer_on_disconnect(peer);
        (*peer).state = ENET_PEER_STATE_DISCONNECTING
    } else { enet_host_flush((*peer).host); enet_peer_reset(peer); };
}
/* * Request a disconnection from a peer, but only after all queued outgoing packets are sent.
    @param peer peer to request a disconnection
    @param data data describing the disconnection
    @remarks An ENET_EVENT_DISCONNECT event will be generated by enet_host_service()
    once the disconnection is complete.
*/
#[no_mangle]
pub unsafe extern "C" fn enet_peer_disconnect_later(mut peer: *mut ENetPeer,
                                                    mut data: EnetUint32) {
    if ((*peer).state as libc::c_uint ==
            ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint ||
            (*peer).state as libc::c_uint ==
                ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                    libc::c_uint) &&
           !((*peer).outgoingReliableCommands.sentinel.next ==
                 &mut (*peer).outgoingReliableCommands.sentinel as
                     *mut ENetListNode &&
                 (*peer).outgoingUnreliableCommands.sentinel.next ==
                     &mut (*peer).outgoingUnreliableCommands.sentinel as
                         *mut ENetListNode &&
                 (*peer).sentReliableCommands.sentinel.next ==
                     &mut (*peer).sentReliableCommands.sentinel as
                         *mut ENetListNode) {
        (*peer).state = ENET_PEER_STATE_DISCONNECT_LATER;
        (*peer).eventData = data
    } else { enet_peer_disconnect(peer, data); };
}
#[no_mangle]
pub unsafe extern "C" fn enet_peer_queue_acknowledgement(mut peer:
                                                             *mut ENetPeer,
                                                         mut command:
                                                             *const ENetProtocol,
                                                         mut sentTime:
                                                             EnetUint16)
 -> *mut ENetAcknowledgement {
    let mut acknowledgement: *mut ENetAcknowledgement =
        0 as *mut ENetAcknowledgement;
    if ((*command).header.channelID as libc::c_ulong) < (*peer).channelCount {
        let mut channel: *mut ENetChannel =
            &mut *(*peer).channels.offset((*command).header.channelID as
                                              isize) as *mut ENetChannel;
        let mut reliableWindow: EnetUint16 =
            ((*command).header.reliableSequenceNumber as libc::c_int /
                 ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as
                EnetUint16;
        let mut currentWindow: EnetUint16 =
            ((*channel).incomingReliableSequenceNumber as libc::c_int /
                 ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as
                EnetUint16;
        if ((*command).header.reliableSequenceNumber as libc::c_int) <
               (*channel).incomingReliableSequenceNumber as libc::c_int {
            reliableWindow =
                (reliableWindow as libc::c_int +
                     ENET_PEER_RELIABLE_WINDOWS as libc::c_int) as EnetUint16
        }
        if reliableWindow as libc::c_int >=
               currentWindow as libc::c_int +
                   ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int - 1i32 &&
               reliableWindow as libc::c_int <=
                   currentWindow as libc::c_int +
                       ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int {
            return 0 as *mut ENetAcknowledgement
        }
    }
    acknowledgement =
        enet_malloc(::std::mem::size_of::<ENetAcknowledgement>() as
                        libc::c_ulong) as *mut ENetAcknowledgement;
    if acknowledgement.is_null() { return 0 as *mut ENetAcknowledgement }
    (*peer).outgoingDataTotal =
        ((*peer).outgoingDataTotal as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<ENetProtocolAcknowledge>()
                                             as libc::c_ulong) as EnetUint32
            as EnetUint32;
    (*acknowledgement).sentTime = sentTime as EnetUint32;
    (*acknowledgement).command = *command;
    enet_list_insert(&mut (*peer).acknowledgements.sentinel,
                     acknowledgement as *mut libc::c_void);
    return acknowledgement;
}
#[no_mangle]
pub unsafe extern "C" fn enet_peer_setup_outgoing_command(mut peer:
                                                              *mut ENetPeer,
                                                          mut outgoingCommand:
                                                              *mut ENetOutgoingCommand) {
    let mut channel: *mut ENetChannel =
        &mut *(*peer).channels.offset((*outgoingCommand).command.header.channelID
                                          as isize) as *mut ENetChannel;
    (*peer).outgoingDataTotal =
        ((*peer).outgoingDataTotal as
             libc::c_ulong).wrapping_add(enet_protocol_command_size((*outgoingCommand).command.header.command).wrapping_add((*outgoingCommand).fragmentLength
                                                                                                                                as
                                                                                                                                libc::c_ulong))
            as EnetUint32 as EnetUint32;
    if (*outgoingCommand).command.header.channelID as libc::c_int == 0xffi32 {
        (*peer).outgoingReliableSequenceNumber =
            (*peer).outgoingReliableSequenceNumber.wrapping_add(1);
        (*outgoingCommand).reliableSequenceNumber =
            (*peer).outgoingReliableSequenceNumber;
        (*outgoingCommand).unreliableSequenceNumber = 0i32 as EnetUint16
    } else if 0 !=
                  (*outgoingCommand).command.header.command as libc::c_int &
                      ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int {
        (*channel).outgoingReliableSequenceNumber =
            (*channel).outgoingReliableSequenceNumber.wrapping_add(1);
        (*channel).outgoingUnreliableSequenceNumber = 0i32 as EnetUint16;
        (*outgoingCommand).reliableSequenceNumber =
            (*channel).outgoingReliableSequenceNumber;
        (*outgoingCommand).unreliableSequenceNumber = 0i32 as EnetUint16
    } else if 0 !=
                  (*outgoingCommand).command.header.command as libc::c_int &
                      ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as libc::c_int {
        (*peer).outgoingUnsequencedGroup =
            (*peer).outgoingUnsequencedGroup.wrapping_add(1);
        (*outgoingCommand).reliableSequenceNumber = 0i32 as EnetUint16;
        (*outgoingCommand).unreliableSequenceNumber = 0i32 as EnetUint16
    } else {
        if (*outgoingCommand).fragmentOffset == 0i32 as libc::c_uint {
            (*channel).outgoingUnreliableSequenceNumber =
                (*channel).outgoingUnreliableSequenceNumber.wrapping_add(1)
        }
        (*outgoingCommand).reliableSequenceNumber =
            (*channel).outgoingReliableSequenceNumber;
        (*outgoingCommand).unreliableSequenceNumber =
            (*channel).outgoingUnreliableSequenceNumber
    }
    (*outgoingCommand).sendAttempts = 0i32 as EnetUint16;
    (*outgoingCommand).sentTime = 0i32 as EnetUint32;
    (*outgoingCommand).roundTripTimeout = 0i32 as EnetUint32;
    (*outgoingCommand).roundTripTimeoutLimit = 0i32 as EnetUint32;
    (*outgoingCommand).command.header.reliableSequenceNumber =
        htons((*outgoingCommand).reliableSequenceNumber);
    match (*outgoingCommand).command.header.command as libc::c_int &
              ENET_PROTOCOL_COMMAND_MASK as libc::c_int {
        7 => {
            (*outgoingCommand).command.sendUnreliable.unreliableSequenceNumber
                = htons((*outgoingCommand).unreliableSequenceNumber)
        }
        9 => {
            (*outgoingCommand).command.sendUnsequenced.unsequencedGroup =
                htons((*peer).outgoingUnsequencedGroup)
        }
        _ => { }
    }
    if 0 !=
           (*outgoingCommand).command.header.command as libc::c_int &
               ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int {
        enet_list_insert(&mut (*peer).outgoingReliableCommands.sentinel,
                         outgoingCommand as *mut libc::c_void);
    } else {
        enet_list_insert(&mut (*peer).outgoingUnreliableCommands.sentinel,
                         outgoingCommand as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn enet_peer_queue_outgoing_command(mut peer:
                                                              *mut ENetPeer,
                                                          mut command:
                                                              *const ENetProtocol,
                                                          mut packet:
                                                              *mut ENetPacket,
                                                          mut offset:
                                                              EnetUint32,
                                                          mut length:
                                                              EnetUint16)
 -> *mut ENetOutgoingCommand {
    let mut outgoingCommand: *mut ENetOutgoingCommand =
        enet_malloc(::std::mem::size_of::<ENetOutgoingCommand>() as
                        libc::c_ulong) as *mut ENetOutgoingCommand;
    if outgoingCommand.is_null() { return 0 as *mut ENetOutgoingCommand }
    (*outgoingCommand).command = *command;
    (*outgoingCommand).fragmentOffset = offset;
    (*outgoingCommand).fragmentLength = length;
    (*outgoingCommand).packet = packet;
    if !packet.is_null() {
        (*packet).referenceCount = (*packet).referenceCount.wrapping_add(1)
    }
    enet_peer_setup_outgoing_command(peer, outgoingCommand);
    return outgoingCommand;
}
#[no_mangle]
pub unsafe extern "C" fn enet_peer_dispatch_incoming_unreliable_commands(mut peer:
                                                                             *mut ENetPeer,
                                                                         mut channel:
                                                                             *mut ENetChannel) {
    let mut droppedCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut startCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut current_block_22: u64;
    currentCommand = (*channel).incomingUnreliableCommands.sentinel.next;
    startCommand = currentCommand;
    droppedCommand = startCommand;
    while currentCommand !=
              &mut (*channel).incomingUnreliableCommands.sentinel as
                  *mut ENetListNode {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if !((*incomingCommand).command.header.command as libc::c_int &
                 ENET_PROTOCOL_COMMAND_MASK as libc::c_int ==
                 ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as libc::c_int) {
            if (*incomingCommand).reliableSequenceNumber as libc::c_int ==
                   (*channel).incomingReliableSequenceNumber as libc::c_int {
                if (*incomingCommand).fragmentsRemaining <=
                       0i32 as libc::c_uint {
                    (*channel).incomingUnreliableSequenceNumber =
                        (*incomingCommand).unreliableSequenceNumber;
                    current_block_22 = 16668937799742929182;
                } else {
                    if startCommand != currentCommand {
                        enet_list_move(&mut (*peer).dispatchedCommands.sentinel,
                                       startCommand as *mut libc::c_void,
                                       (*currentCommand).previous as
                                           *mut libc::c_void);
                        if 0 == (*peer).needsDispatch {
                            enet_list_insert(&mut (*(*peer).host).dispatchQueue.sentinel,
                                             &mut (*peer).dispatchList as
                                                 *mut ENetListNode as
                                                 *mut libc::c_void);
                            (*peer).needsDispatch = 1i32
                        }
                        droppedCommand = currentCommand
                    } else if droppedCommand != currentCommand {
                        droppedCommand = (*currentCommand).previous
                    }
                    current_block_22 = 16924917904204750491;
                }
            } else {
                let mut reliableWindow: EnetUint16 =
                    ((*incomingCommand).reliableSequenceNumber as libc::c_int
                         / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as
                        EnetUint16;
                let mut currentWindow: EnetUint16 =
                    ((*channel).incomingReliableSequenceNumber as libc::c_int
                         / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as
                        EnetUint16;
                if ((*incomingCommand).reliableSequenceNumber as libc::c_int)
                       <
                       (*channel).incomingReliableSequenceNumber as
                           libc::c_int {
                    reliableWindow =
                        (reliableWindow as libc::c_int +
                             ENET_PEER_RELIABLE_WINDOWS as libc::c_int) as
                            EnetUint16
                }
                if reliableWindow as libc::c_int >=
                       currentWindow as libc::c_int &&
                       (reliableWindow as libc::c_int) <
                           currentWindow as libc::c_int +
                               ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int
                               - 1i32 {
                    break ;
                }
                droppedCommand = (*currentCommand).next;
                if startCommand != currentCommand {
                    enet_list_move(&mut (*peer).dispatchedCommands.sentinel,
                                   startCommand as *mut libc::c_void,
                                   (*currentCommand).previous as
                                       *mut libc::c_void);
                    if 0 == (*peer).needsDispatch {
                        enet_list_insert(&mut (*(*peer).host).dispatchQueue.sentinel,
                                         &mut (*peer).dispatchList as
                                             *mut ENetListNode as
                                             *mut libc::c_void);
                        (*peer).needsDispatch = 1i32
                    }
                }
                current_block_22 = 16924917904204750491;
            }
            match current_block_22 {
                16668937799742929182 => { }
                _ => { startCommand = (*currentCommand).next }
            }
        }
        currentCommand = (*currentCommand).next
    }
    if startCommand != currentCommand {
        enet_list_move(&mut (*peer).dispatchedCommands.sentinel,
                       startCommand as *mut libc::c_void,
                       (*currentCommand).previous as *mut libc::c_void);
        if 0 == (*peer).needsDispatch {
            enet_list_insert(&mut (*(*peer).host).dispatchQueue.sentinel,
                             &mut (*peer).dispatchList as *mut ENetListNode as
                                 *mut libc::c_void);
            (*peer).needsDispatch = 1i32
        }
        droppedCommand = currentCommand
    }
    enet_peer_remove_incoming_commands(&mut (*channel).incomingUnreliableCommands,
                                       (*channel).incomingUnreliableCommands.sentinel.next,
                                       droppedCommand);
}
#[no_mangle]
pub unsafe extern "C" fn enet_peer_dispatch_incoming_reliable_commands(mut peer:
                                                                           *mut ENetPeer,
                                                                       mut channel:
                                                                           *mut ENetChannel) {
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    currentCommand = (*channel).incomingReliableCommands.sentinel.next;
    while currentCommand !=
              &mut (*channel).incomingReliableCommands.sentinel as
                  *mut ENetListNode {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if (*incomingCommand).fragmentsRemaining > 0i32 as libc::c_uint ||
               (*incomingCommand).reliableSequenceNumber as libc::c_int !=
                   ((*channel).incomingReliableSequenceNumber as libc::c_int +
                        1i32) as EnetUint16 as libc::c_int {
            break ;
        }
        (*channel).incomingReliableSequenceNumber =
            (*incomingCommand).reliableSequenceNumber;
        if (*incomingCommand).fragmentCount > 0i32 as libc::c_uint {
            (*channel).incomingReliableSequenceNumber =
                ((*channel).incomingReliableSequenceNumber as
                     libc::c_uint).wrapping_add((*incomingCommand).fragmentCount.wrapping_sub(1i32
                                                                                                  as
                                                                                                  libc::c_uint))
                    as EnetUint16 as EnetUint16
        }
        currentCommand = (*currentCommand).next
    }
    if currentCommand == (*channel).incomingReliableCommands.sentinel.next {
        return
    }
    (*channel).incomingUnreliableSequenceNumber = 0i32 as EnetUint16;
    enet_list_move(&mut (*peer).dispatchedCommands.sentinel,
                   (*channel).incomingReliableCommands.sentinel.next as
                       *mut libc::c_void,
                   (*currentCommand).previous as *mut libc::c_void);
    if 0 == (*peer).needsDispatch {
        enet_list_insert(&mut (*(*peer).host).dispatchQueue.sentinel,
                         &mut (*peer).dispatchList as *mut ENetListNode as
                             *mut libc::c_void);
        (*peer).needsDispatch = 1i32
    }
    if !((*channel).incomingUnreliableCommands.sentinel.next ==
             &mut (*channel).incomingUnreliableCommands.sentinel as
                 *mut ENetListNode) {
        enet_peer_dispatch_incoming_unreliable_commands(peer, channel);
    };
}
#[no_mangle]
pub unsafe extern "C" fn enet_peer_queue_incoming_command(mut peer:
                                                              *mut ENetPeer,
                                                          mut command:
                                                              *const ENetProtocol,
                                                          mut data:
                                                              *const libc::c_void,
                                                          mut data_length:
                                                              SizeT,
                                                          mut flags:
                                                              EnetUint32,
                                                          mut fragmentCount:
                                                              EnetUint32)
 -> *mut ENetIncomingCommand {
    let mut current_block: u64;
    static mut dummyCommand: ENetIncomingCommand =
        ENetIncomingCommand{incomingCommandList:
                                ENetListNode{next: 0 as *mut _ENetListNode,
                                             previous:
                                                 0 as *mut _ENetListNode,},
                            reliableSequenceNumber: 0,
                            unreliableSequenceNumber: 0,
                            command:
                                _ENetProtocol{header:
                                                  ENetProtocolCommandHeader{command:
                                                                                0,
                                                                            channelID:
                                                                                0,
                                                                            reliableSequenceNumber:
                                                                                0,},},
                            fragmentCount: 0,
                            fragmentsRemaining: 0,
                            fragments:
                                0 as *const EnetUint32 as *mut EnetUint32,
                            packet:
                                0 as *const ENetPacket as *mut ENetPacket,};
    let mut channel: *mut ENetChannel =
        &mut *(*peer).channels.offset((*command).header.channelID as isize) as
            *mut ENetChannel;
    let mut unreliableSequenceNumber: EnetUint32 = 0i32 as EnetUint32;
    let mut reliableSequenceNumber: EnetUint32 = 0i32 as EnetUint32;
    let mut reliableWindow: EnetUint16 = 0;
    let mut currentWindow: EnetUint16 = 0;
    let mut incomingCommand: *mut ENetIncomingCommand =
        0 as *mut ENetIncomingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut packet: *mut ENetPacket = 0 as *mut ENetPacket;
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint {
        current_block = 4944626207523371504;
    } else {
        if (*command).header.command as libc::c_int &
               ENET_PROTOCOL_COMMAND_MASK as libc::c_int !=
               ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as libc::c_int {
            reliableSequenceNumber =
                (*command).header.reliableSequenceNumber as EnetUint32;
            reliableWindow =
                reliableSequenceNumber.wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE
                                                        as libc::c_int as
                                                        libc::c_uint) as
                    EnetUint16;
            currentWindow =
                ((*channel).incomingReliableSequenceNumber as libc::c_int /
                     ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as
                    EnetUint16;
            if reliableSequenceNumber <
                   (*channel).incomingReliableSequenceNumber as libc::c_uint {
                reliableWindow =
                    (reliableWindow as libc::c_int +
                         ENET_PEER_RELIABLE_WINDOWS as libc::c_int) as
                        EnetUint16
            }
            if (reliableWindow as libc::c_int) < currentWindow as libc::c_int
                   ||
                   reliableWindow as libc::c_int >=
                       currentWindow as libc::c_int +
                           ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int -
                           1i32 {
                current_block = 4944626207523371504;
            } else { current_block = 5399440093318478209; }
        } else { current_block = 5399440093318478209; }
        match current_block {
            4944626207523371504 => { }
            _ => {
                match (*command).header.command as libc::c_int &
                          ENET_PROTOCOL_COMMAND_MASK as libc::c_int {
                    8 | 6 => {
                        current_block = 2916768294295805375;
                        match current_block {
                            5122324059762049690 => {
                                currentCommand =
                                    &mut (*channel).incomingUnreliableCommands.sentinel;
                                current_block = 2122094917359643297;
                            }
                            2916768294295805375 => {
                                if reliableSequenceNumber ==
                                       (*channel).incomingReliableSequenceNumber
                                           as libc::c_uint {
                                    current_block = 4944626207523371504;
                                } else {
                                    currentCommand =
                                        (*channel).incomingReliableCommands.sentinel.previous;
                                    loop  {
                                        if !(currentCommand !=
                                                 &mut (*channel).incomingReliableCommands.sentinel
                                                     as *mut ENetListNode) {
                                            current_block =
                                                2122094917359643297;
                                            break ;
                                        }
                                        incomingCommand =
                                            currentCommand as
                                                *mut ENetIncomingCommand;
                                        if reliableSequenceNumber >=
                                               (*channel).incomingReliableSequenceNumber
                                                   as libc::c_uint {
                                            if ((*incomingCommand).reliableSequenceNumber
                                                    as libc::c_int) <
                                                   (*channel).incomingReliableSequenceNumber
                                                       as libc::c_int {
                                                current_block =
                                                    1054647088692577877;
                                            } else {
                                                current_block =
                                                    5783071609795492627;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber
                                                   as libc::c_int >=
                                                   (*channel).incomingReliableSequenceNumber
                                                       as libc::c_int {
                                                current_block =
                                                    2122094917359643297;
                                                break ;
                                            }
                                            current_block =
                                                5783071609795492627;
                                        }
                                        match current_block {
                                            5783071609795492627 => {
                                                if (*incomingCommand).reliableSequenceNumber
                                                       as libc::c_uint <=
                                                       reliableSequenceNumber
                                                   {
                                                    if ((*incomingCommand).reliableSequenceNumber
                                                            as libc::c_uint) <
                                                           reliableSequenceNumber
                                                       {
                                                        current_block =
                                                            2122094917359643297;
                                                        break ;
                                                    } else {
                                                        current_block =
                                                            4944626207523371504;
                                                        break ;
                                                    }
                                                }
                                            }
                                            _ => { }
                                        }
                                        currentCommand =
                                            (*currentCommand).previous
                                    }
                                }
                            }
                            _ => {
                                unreliableSequenceNumber =
                                    ntohs((*command).sendUnreliable.unreliableSequenceNumber)
                                        as EnetUint32;
                                if reliableSequenceNumber ==
                                       (*channel).incomingReliableSequenceNumber
                                           as libc::c_uint &&
                                       unreliableSequenceNumber <=
                                           (*channel).incomingUnreliableSequenceNumber
                                               as libc::c_uint {
                                    current_block = 4944626207523371504;
                                } else {
                                    currentCommand =
                                        (*channel).incomingUnreliableCommands.sentinel.previous;
                                    loop  {
                                        if !(currentCommand !=
                                                 &mut (*channel).incomingUnreliableCommands.sentinel
                                                     as *mut ENetListNode) {
                                            current_block =
                                                2122094917359643297;
                                            break ;
                                        }
                                        incomingCommand =
                                            currentCommand as
                                                *mut ENetIncomingCommand;
                                        if !((*command).header.command as
                                                 libc::c_int &
                                                 ENET_PROTOCOL_COMMAND_MASK as
                                                     libc::c_int ==
                                                 ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED
                                                     as libc::c_int) {
                                            if reliableSequenceNumber >=
                                                   (*channel).incomingReliableSequenceNumber
                                                       as libc::c_uint {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                        as libc::c_int) <
                                                       (*channel).incomingReliableSequenceNumber
                                                           as libc::c_int {
                                                    current_block =
                                                        5689316957504528238;
                                                } else {
                                                    current_block =
                                                        11048769245176032998;
                                                }
                                            } else {
                                                if (*incomingCommand).reliableSequenceNumber
                                                       as libc::c_int >=
                                                       (*channel).incomingReliableSequenceNumber
                                                           as libc::c_int {
                                                    current_block =
                                                        2122094917359643297;
                                                    break ;
                                                }
                                                current_block =
                                                    11048769245176032998;
                                            }
                                            match current_block {
                                                5689316957504528238 => { }
                                                _ => {
                                                    if ((*incomingCommand).reliableSequenceNumber
                                                            as libc::c_uint) <
                                                           reliableSequenceNumber
                                                       {
                                                        current_block =
                                                            2122094917359643297;
                                                        break ;
                                                    }
                                                    if !((*incomingCommand).reliableSequenceNumber
                                                             as libc::c_uint >
                                                             reliableSequenceNumber)
                                                       {
                                                        if (*incomingCommand).unreliableSequenceNumber
                                                               as libc::c_uint
                                                               <=
                                                               unreliableSequenceNumber
                                                           {
                                                            if ((*incomingCommand).unreliableSequenceNumber
                                                                    as
                                                                    libc::c_uint)
                                                                   <
                                                                   unreliableSequenceNumber
                                                               {
                                                                current_block
                                                                    =
                                                                    2122094917359643297;
                                                                break ;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    4944626207523371504;
                                                                break ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        currentCommand =
                                            (*currentCommand).previous
                                    }
                                }
                            }
                        }
                        match current_block {
                            4944626207523371504 => { }
                            _ => {
                                if (*peer).totalWaitingData >=
                                       (*(*peer).host).maximumWaitingData {
                                    current_block = 3740873486607446794;
                                } else {
                                    packet =
                                        enet_packet_create(data, data_length,
                                                           flags);
                                    if packet.is_null() {
                                        current_block = 3740873486607446794;
                                    } else {
                                        incomingCommand =
                                            enet_malloc(::std::mem::size_of::<ENetIncomingCommand>()
                                                            as libc::c_ulong)
                                                as *mut ENetIncomingCommand;
                                        if incomingCommand.is_null() {
                                            current_block =
                                                3740873486607446794;
                                        } else {
                                            (*incomingCommand).reliableSequenceNumber
                                                =
                                                (*command).header.reliableSequenceNumber;
                                            (*incomingCommand).unreliableSequenceNumber
                                                =
                                                (unreliableSequenceNumber &
                                                     0xffffi32 as
                                                         libc::c_uint) as
                                                    EnetUint16;
                                            (*incomingCommand).command =
                                                *command;
                                            (*incomingCommand).fragmentCount =
                                                fragmentCount;
                                            (*incomingCommand).fragmentsRemaining
                                                = fragmentCount;
                                            (*incomingCommand).packet =
                                                packet;
                                            (*incomingCommand).fragments =
                                                0 as *mut EnetUint32;
                                            if fragmentCount >
                                                   0i32 as libc::c_uint {
                                                if fragmentCount <=
                                                       ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT
                                                           as libc::c_int as
                                                           libc::c_uint {
                                                    (*incomingCommand).fragments
                                                        =
                                                        enet_malloc((fragmentCount.wrapping_add(31i32
                                                                                                    as
                                                                                                    libc::c_uint).wrapping_div(32i32
                                                                                                                                   as
                                                                                                                                   libc::c_uint)
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<EnetUint32>()
                                                                                                         as
                                                                                                         libc::c_ulong))
                                                            as
                                                            *mut EnetUint32
                                                }
                                                if (*incomingCommand).fragments.is_null()
                                                   {
                                                    enet_free(incomingCommand
                                                                  as
                                                                  *mut libc::c_void);
                                                    current_block =
                                                        3740873486607446794;
                                                } else {
                                                    memset((*incomingCommand).fragments
                                                               as
                                                               *mut libc::c_void,
                                                           0i32,
                                                           (fragmentCount.wrapping_add(31i32
                                                                                           as
                                                                                           libc::c_uint).wrapping_div(32i32
                                                                                                                          as
                                                                                                                          libc::c_uint)
                                                                as
                                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<EnetUint32>()
                                                                                                as
                                                                                                libc::c_ulong));
                                                    current_block =
                                                        5597585068398118923;
                                                }
                                            } else {
                                                current_block =
                                                    5597585068398118923;
                                            }
                                            match current_block {
                                                3740873486607446794 => { }
                                                _ => {
                                                    if !packet.is_null() {
                                                        (*packet).referenceCount
                                                            =
                                                            (*packet).referenceCount.wrapping_add(1);
                                                        (*peer).totalWaitingData
                                                            =
                                                            ((*peer).totalWaitingData
                                                                 as
                                                                 libc::c_ulong).wrapping_add((*packet).data_length)
                                                                as SizeT as
                                                                SizeT
                                                    }
                                                    enet_list_insert((*currentCommand).next,
                                                                     incomingCommand
                                                                         as
                                                                         *mut libc::c_void);
                                                    match (*command).header.command
                                                              as libc::c_int &
                                                              ENET_PROTOCOL_COMMAND_MASK
                                                                  as
                                                                  libc::c_int
                                                        {
                                                        8 | 6 => {
                                                            enet_peer_dispatch_incoming_reliable_commands(peer,
                                                                                                          channel);
                                                        }
                                                        _ => {
                                                            enet_peer_dispatch_incoming_unreliable_commands(peer,
                                                                                                            channel);
                                                        }
                                                    }
                                                    return incomingCommand
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    7 | 12 => {
                        current_block = 340123238355120661;
                        match current_block {
                            5122324059762049690 => {
                                currentCommand =
                                    &mut (*channel).incomingUnreliableCommands.sentinel;
                                current_block = 2122094917359643297;
                            }
                            2916768294295805375 => {
                                if reliableSequenceNumber ==
                                       (*channel).incomingReliableSequenceNumber
                                           as libc::c_uint {
                                    current_block = 4944626207523371504;
                                } else {
                                    currentCommand =
                                        (*channel).incomingReliableCommands.sentinel.previous;
                                    loop  {
                                        if !(currentCommand !=
                                                 &mut (*channel).incomingReliableCommands.sentinel
                                                     as *mut ENetListNode) {
                                            current_block =
                                                2122094917359643297;
                                            break ;
                                        }
                                        incomingCommand =
                                            currentCommand as
                                                *mut ENetIncomingCommand;
                                        if reliableSequenceNumber >=
                                               (*channel).incomingReliableSequenceNumber
                                                   as libc::c_uint {
                                            if ((*incomingCommand).reliableSequenceNumber
                                                    as libc::c_int) <
                                                   (*channel).incomingReliableSequenceNumber
                                                       as libc::c_int {
                                                current_block =
                                                    1054647088692577877;
                                            } else {
                                                current_block =
                                                    5783071609795492627;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber
                                                   as libc::c_int >=
                                                   (*channel).incomingReliableSequenceNumber
                                                       as libc::c_int {
                                                current_block =
                                                    2122094917359643297;
                                                break ;
                                            }
                                            current_block =
                                                5783071609795492627;
                                        }
                                        match current_block {
                                            5783071609795492627 => {
                                                if (*incomingCommand).reliableSequenceNumber
                                                       as libc::c_uint <=
                                                       reliableSequenceNumber
                                                   {
                                                    if ((*incomingCommand).reliableSequenceNumber
                                                            as libc::c_uint) <
                                                           reliableSequenceNumber
                                                       {
                                                        current_block =
                                                            2122094917359643297;
                                                        break ;
                                                    } else {
                                                        current_block =
                                                            4944626207523371504;
                                                        break ;
                                                    }
                                                }
                                            }
                                            _ => { }
                                        }
                                        currentCommand =
                                            (*currentCommand).previous
                                    }
                                }
                            }
                            _ => {
                                unreliableSequenceNumber =
                                    ntohs((*command).sendUnreliable.unreliableSequenceNumber)
                                        as EnetUint32;
                                if reliableSequenceNumber ==
                                       (*channel).incomingReliableSequenceNumber
                                           as libc::c_uint &&
                                       unreliableSequenceNumber <=
                                           (*channel).incomingUnreliableSequenceNumber
                                               as libc::c_uint {
                                    current_block = 4944626207523371504;
                                } else {
                                    currentCommand =
                                        (*channel).incomingUnreliableCommands.sentinel.previous;
                                    loop  {
                                        if !(currentCommand !=
                                                 &mut (*channel).incomingUnreliableCommands.sentinel
                                                     as *mut ENetListNode) {
                                            current_block =
                                                2122094917359643297;
                                            break ;
                                        }
                                        incomingCommand =
                                            currentCommand as
                                                *mut ENetIncomingCommand;
                                        if !((*command).header.command as
                                                 libc::c_int &
                                                 ENET_PROTOCOL_COMMAND_MASK as
                                                     libc::c_int ==
                                                 ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED
                                                     as libc::c_int) {
                                            if reliableSequenceNumber >=
                                                   (*channel).incomingReliableSequenceNumber
                                                       as libc::c_uint {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                        as libc::c_int) <
                                                       (*channel).incomingReliableSequenceNumber
                                                           as libc::c_int {
                                                    current_block =
                                                        5689316957504528238;
                                                } else {
                                                    current_block =
                                                        11048769245176032998;
                                                }
                                            } else {
                                                if (*incomingCommand).reliableSequenceNumber
                                                       as libc::c_int >=
                                                       (*channel).incomingReliableSequenceNumber
                                                           as libc::c_int {
                                                    current_block =
                                                        2122094917359643297;
                                                    break ;
                                                }
                                                current_block =
                                                    11048769245176032998;
                                            }
                                            match current_block {
                                                5689316957504528238 => { }
                                                _ => {
                                                    if ((*incomingCommand).reliableSequenceNumber
                                                            as libc::c_uint) <
                                                           reliableSequenceNumber
                                                       {
                                                        current_block =
                                                            2122094917359643297;
                                                        break ;
                                                    }
                                                    if !((*incomingCommand).reliableSequenceNumber
                                                             as libc::c_uint >
                                                             reliableSequenceNumber)
                                                       {
                                                        if (*incomingCommand).unreliableSequenceNumber
                                                               as libc::c_uint
                                                               <=
                                                               unreliableSequenceNumber
                                                           {
                                                            if ((*incomingCommand).unreliableSequenceNumber
                                                                    as
                                                                    libc::c_uint)
                                                                   <
                                                                   unreliableSequenceNumber
                                                               {
                                                                current_block
                                                                    =
                                                                    2122094917359643297;
                                                                break ;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    4944626207523371504;
                                                                break ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        currentCommand =
                                            (*currentCommand).previous
                                    }
                                }
                            }
                        }
                        match current_block {
                            4944626207523371504 => { }
                            _ => {
                                if (*peer).totalWaitingData >=
                                       (*(*peer).host).maximumWaitingData {
                                    current_block = 3740873486607446794;
                                } else {
                                    packet =
                                        enet_packet_create(data, data_length,
                                                           flags);
                                    if packet.is_null() {
                                        current_block = 3740873486607446794;
                                    } else {
                                        incomingCommand =
                                            enet_malloc(::std::mem::size_of::<ENetIncomingCommand>()
                                                            as libc::c_ulong)
                                                as *mut ENetIncomingCommand;
                                        if incomingCommand.is_null() {
                                            current_block =
                                                3740873486607446794;
                                        } else {
                                            (*incomingCommand).reliableSequenceNumber
                                                =
                                                (*command).header.reliableSequenceNumber;
                                            (*incomingCommand).unreliableSequenceNumber
                                                =
                                                (unreliableSequenceNumber &
                                                     0xffffi32 as
                                                         libc::c_uint) as
                                                    EnetUint16;
                                            (*incomingCommand).command =
                                                *command;
                                            (*incomingCommand).fragmentCount =
                                                fragmentCount;
                                            (*incomingCommand).fragmentsRemaining
                                                = fragmentCount;
                                            (*incomingCommand).packet =
                                                packet;
                                            (*incomingCommand).fragments =
                                                0 as *mut EnetUint32;
                                            if fragmentCount >
                                                   0i32 as libc::c_uint {
                                                if fragmentCount <=
                                                       ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT
                                                           as libc::c_int as
                                                           libc::c_uint {
                                                    (*incomingCommand).fragments
                                                        =
                                                        enet_malloc((fragmentCount.wrapping_add(31i32
                                                                                                    as
                                                                                                    libc::c_uint).wrapping_div(32i32
                                                                                                                                   as
                                                                                                                                   libc::c_uint)
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<EnetUint32>()
                                                                                                         as
                                                                                                         libc::c_ulong))
                                                            as
                                                            *mut EnetUint32
                                                }
                                                if (*incomingCommand).fragments.is_null()
                                                   {
                                                    enet_free(incomingCommand
                                                                  as
                                                                  *mut libc::c_void);
                                                    current_block =
                                                        3740873486607446794;
                                                } else {
                                                    memset((*incomingCommand).fragments
                                                               as
                                                               *mut libc::c_void,
                                                           0i32,
                                                           (fragmentCount.wrapping_add(31i32
                                                                                           as
                                                                                           libc::c_uint).wrapping_div(32i32
                                                                                                                          as
                                                                                                                          libc::c_uint)
                                                                as
                                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<EnetUint32>()
                                                                                                as
                                                                                                libc::c_ulong));
                                                    current_block =
                                                        5597585068398118923;
                                                }
                                            } else {
                                                current_block =
                                                    5597585068398118923;
                                            }
                                            match current_block {
                                                3740873486607446794 => { }
                                                _ => {
                                                    if !packet.is_null() {
                                                        (*packet).referenceCount
                                                            =
                                                            (*packet).referenceCount.wrapping_add(1);
                                                        (*peer).totalWaitingData
                                                            =
                                                            ((*peer).totalWaitingData
                                                                 as
                                                                 libc::c_ulong).wrapping_add((*packet).data_length)
                                                                as SizeT as
                                                                SizeT
                                                    }
                                                    enet_list_insert((*currentCommand).next,
                                                                     incomingCommand
                                                                         as
                                                                         *mut libc::c_void);
                                                    match (*command).header.command
                                                              as libc::c_int &
                                                              ENET_PROTOCOL_COMMAND_MASK
                                                                  as
                                                                  libc::c_int
                                                        {
                                                        8 | 6 => {
                                                            enet_peer_dispatch_incoming_reliable_commands(peer,
                                                                                                          channel);
                                                        }
                                                        _ => {
                                                            enet_peer_dispatch_incoming_unreliable_commands(peer,
                                                                                                            channel);
                                                        }
                                                    }
                                                    return incomingCommand
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    9 => {
                        current_block = 5122324059762049690;
                        match current_block {
                            5122324059762049690 => {
                                currentCommand =
                                    &mut (*channel).incomingUnreliableCommands.sentinel;
                                current_block = 2122094917359643297;
                            }
                            2916768294295805375 => {
                                if reliableSequenceNumber ==
                                       (*channel).incomingReliableSequenceNumber
                                           as libc::c_uint {
                                    current_block = 4944626207523371504;
                                } else {
                                    currentCommand =
                                        (*channel).incomingReliableCommands.sentinel.previous;
                                    loop  {
                                        if !(currentCommand !=
                                                 &mut (*channel).incomingReliableCommands.sentinel
                                                     as *mut ENetListNode) {
                                            current_block =
                                                2122094917359643297;
                                            break ;
                                        }
                                        incomingCommand =
                                            currentCommand as
                                                *mut ENetIncomingCommand;
                                        if reliableSequenceNumber >=
                                               (*channel).incomingReliableSequenceNumber
                                                   as libc::c_uint {
                                            if ((*incomingCommand).reliableSequenceNumber
                                                    as libc::c_int) <
                                                   (*channel).incomingReliableSequenceNumber
                                                       as libc::c_int {
                                                current_block =
                                                    1054647088692577877;
                                            } else {
                                                current_block =
                                                    5783071609795492627;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber
                                                   as libc::c_int >=
                                                   (*channel).incomingReliableSequenceNumber
                                                       as libc::c_int {
                                                current_block =
                                                    2122094917359643297;
                                                break ;
                                            }
                                            current_block =
                                                5783071609795492627;
                                        }
                                        match current_block {
                                            5783071609795492627 => {
                                                if (*incomingCommand).reliableSequenceNumber
                                                       as libc::c_uint <=
                                                       reliableSequenceNumber
                                                   {
                                                    if ((*incomingCommand).reliableSequenceNumber
                                                            as libc::c_uint) <
                                                           reliableSequenceNumber
                                                       {
                                                        current_block =
                                                            2122094917359643297;
                                                        break ;
                                                    } else {
                                                        current_block =
                                                            4944626207523371504;
                                                        break ;
                                                    }
                                                }
                                            }
                                            _ => { }
                                        }
                                        currentCommand =
                                            (*currentCommand).previous
                                    }
                                }
                            }
                            _ => {
                                unreliableSequenceNumber =
                                    ntohs((*command).sendUnreliable.unreliableSequenceNumber)
                                        as EnetUint32;
                                if reliableSequenceNumber ==
                                       (*channel).incomingReliableSequenceNumber
                                           as libc::c_uint &&
                                       unreliableSequenceNumber <=
                                           (*channel).incomingUnreliableSequenceNumber
                                               as libc::c_uint {
                                    current_block = 4944626207523371504;
                                } else {
                                    currentCommand =
                                        (*channel).incomingUnreliableCommands.sentinel.previous;
                                    loop  {
                                        if !(currentCommand !=
                                                 &mut (*channel).incomingUnreliableCommands.sentinel
                                                     as *mut ENetListNode) {
                                            current_block =
                                                2122094917359643297;
                                            break ;
                                        }
                                        incomingCommand =
                                            currentCommand as
                                                *mut ENetIncomingCommand;
                                        if !((*command).header.command as
                                                 libc::c_int &
                                                 ENET_PROTOCOL_COMMAND_MASK as
                                                     libc::c_int ==
                                                 ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED
                                                     as libc::c_int) {
                                            if reliableSequenceNumber >=
                                                   (*channel).incomingReliableSequenceNumber
                                                       as libc::c_uint {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                        as libc::c_int) <
                                                       (*channel).incomingReliableSequenceNumber
                                                           as libc::c_int {
                                                    current_block =
                                                        5689316957504528238;
                                                } else {
                                                    current_block =
                                                        11048769245176032998;
                                                }
                                            } else {
                                                if (*incomingCommand).reliableSequenceNumber
                                                       as libc::c_int >=
                                                       (*channel).incomingReliableSequenceNumber
                                                           as libc::c_int {
                                                    current_block =
                                                        2122094917359643297;
                                                    break ;
                                                }
                                                current_block =
                                                    11048769245176032998;
                                            }
                                            match current_block {
                                                5689316957504528238 => { }
                                                _ => {
                                                    if ((*incomingCommand).reliableSequenceNumber
                                                            as libc::c_uint) <
                                                           reliableSequenceNumber
                                                       {
                                                        current_block =
                                                            2122094917359643297;
                                                        break ;
                                                    }
                                                    if !((*incomingCommand).reliableSequenceNumber
                                                             as libc::c_uint >
                                                             reliableSequenceNumber)
                                                       {
                                                        if (*incomingCommand).unreliableSequenceNumber
                                                               as libc::c_uint
                                                               <=
                                                               unreliableSequenceNumber
                                                           {
                                                            if ((*incomingCommand).unreliableSequenceNumber
                                                                    as
                                                                    libc::c_uint)
                                                                   <
                                                                   unreliableSequenceNumber
                                                               {
                                                                current_block
                                                                    =
                                                                    2122094917359643297;
                                                                break ;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    4944626207523371504;
                                                                break ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        currentCommand =
                                            (*currentCommand).previous
                                    }
                                }
                            }
                        }
                        match current_block {
                            4944626207523371504 => { }
                            _ => {
                                if (*peer).totalWaitingData >=
                                       (*(*peer).host).maximumWaitingData {
                                    current_block = 3740873486607446794;
                                } else {
                                    packet =
                                        enet_packet_create(data, data_length,
                                                           flags);
                                    if packet.is_null() {
                                        current_block = 3740873486607446794;
                                    } else {
                                        incomingCommand =
                                            enet_malloc(::std::mem::size_of::<ENetIncomingCommand>()
                                                            as libc::c_ulong)
                                                as *mut ENetIncomingCommand;
                                        if incomingCommand.is_null() {
                                            current_block =
                                                3740873486607446794;
                                        } else {
                                            (*incomingCommand).reliableSequenceNumber
                                                =
                                                (*command).header.reliableSequenceNumber;
                                            (*incomingCommand).unreliableSequenceNumber
                                                =
                                                (unreliableSequenceNumber &
                                                     0xffffi32 as
                                                         libc::c_uint) as
                                                    EnetUint16;
                                            (*incomingCommand).command =
                                                *command;
                                            (*incomingCommand).fragmentCount =
                                                fragmentCount;
                                            (*incomingCommand).fragmentsRemaining
                                                = fragmentCount;
                                            (*incomingCommand).packet =
                                                packet;
                                            (*incomingCommand).fragments =
                                                0 as *mut EnetUint32;
                                            if fragmentCount >
                                                   0i32 as libc::c_uint {
                                                if fragmentCount <=
                                                       ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT
                                                           as libc::c_int as
                                                           libc::c_uint {
                                                    (*incomingCommand).fragments
                                                        =
                                                        enet_malloc((fragmentCount.wrapping_add(31i32
                                                                                                    as
                                                                                                    libc::c_uint).wrapping_div(32i32
                                                                                                                                   as
                                                                                                                                   libc::c_uint)
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<EnetUint32>()
                                                                                                         as
                                                                                                         libc::c_ulong))
                                                            as
                                                            *mut EnetUint32
                                                }
                                                if (*incomingCommand).fragments.is_null()
                                                   {
                                                    enet_free(incomingCommand
                                                                  as
                                                                  *mut libc::c_void);
                                                    current_block =
                                                        3740873486607446794;
                                                } else {
                                                    memset((*incomingCommand).fragments
                                                               as
                                                               *mut libc::c_void,
                                                           0i32,
                                                           (fragmentCount.wrapping_add(31i32
                                                                                           as
                                                                                           libc::c_uint).wrapping_div(32i32
                                                                                                                          as
                                                                                                                          libc::c_uint)
                                                                as
                                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<EnetUint32>()
                                                                                                as
                                                                                                libc::c_ulong));
                                                    current_block =
                                                        5597585068398118923;
                                                }
                                            } else {
                                                current_block =
                                                    5597585068398118923;
                                            }
                                            match current_block {
                                                3740873486607446794 => { }
                                                _ => {
                                                    if !packet.is_null() {
                                                        (*packet).referenceCount
                                                            =
                                                            (*packet).referenceCount.wrapping_add(1);
                                                        (*peer).totalWaitingData
                                                            =
                                                            ((*peer).totalWaitingData
                                                                 as
                                                                 libc::c_ulong).wrapping_add((*packet).data_length)
                                                                as SizeT as
                                                                SizeT
                                                    }
                                                    enet_list_insert((*currentCommand).next,
                                                                     incomingCommand
                                                                         as
                                                                         *mut libc::c_void);
                                                    match (*command).header.command
                                                              as libc::c_int &
                                                              ENET_PROTOCOL_COMMAND_MASK
                                                                  as
                                                                  libc::c_int
                                                        {
                                                        8 | 6 => {
                                                            enet_peer_dispatch_incoming_reliable_commands(peer,
                                                                                                          channel);
                                                        }
                                                        _ => {
                                                            enet_peer_dispatch_incoming_unreliable_commands(peer,
                                                                                                            channel);
                                                        }
                                                    }
                                                    return incomingCommand
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => { current_block = 4944626207523371504; }
                }
            }
        }
    }
    match current_block {
        4944626207523371504 => {
            if !(fragmentCount > 0i32 as libc::c_uint) {
                if !packet.is_null() &&
                       (*packet).referenceCount == 0i32 as libc::c_ulong {
                    enet_packet_destroy(packet);
                }
                return &mut dummyCommand
            }
        }
        _ => { }
    }
    if !packet.is_null() && (*packet).referenceCount == 0i32 as libc::c_ulong
       {
        enet_packet_destroy(packet);
    }
    return 0 as *mut ENetIncomingCommand;
}