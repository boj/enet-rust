use libc;
extern "C" {
    /* * @defgroup callbacks ENet internal callbacks
    @{
    @ingroup private
*/
    #[no_mangle]
    fn enet_malloc(_: SizeT) -> *mut libc::c_void;
    #[no_mangle]
    fn enet_free(_: *mut libc::c_void);
    #[no_mangle]
    fn enet_list_clear(_: *mut ENetList);
    #[no_mangle]
    fn enet_list_insert(_: ENetListIterator, _: *mut libc::c_void)
     -> ENetListIterator;
    #[no_mangle]
    fn enet_list_remove(_: ENetListIterator) -> *mut libc::c_void;
    /* * @} */
    /* * @defgroup private ENet private implementation functions */
    /* *
  Returns the wall-time in milliseconds.  Its initial value is unspecified
  unless otherwise set.
  */
    #[no_mangle]
    fn enet_time_get() -> EnetUint32;
    #[no_mangle]
    fn enet_socket_send(_: ENetSocket, _: *const ENetAddress,
                        _: *const ENetBuffer, _: SizeT) -> libc::c_int;
    #[no_mangle]
    fn enet_socket_receive(_: ENetSocket, _: *mut ENetAddress,
                           _: *mut ENetBuffer, _: SizeT) -> libc::c_int;
    #[no_mangle]
    fn enet_socket_wait(_: ENetSocket, _: *mut EnetUint32, _: EnetUint32)
     -> libc::c_int;
    #[no_mangle]
    fn enet_packet_destroy(_: *mut ENetPacket);
    #[no_mangle]
    fn enet_host_bandwidth_throttle(_: *mut ENetHost);
    #[no_mangle]
    fn enet_peer_receive(_: *mut ENetPeer, channelID: *mut enet_uint8)
     -> *mut ENetPacket;
    #[no_mangle]
    fn enet_peer_ping(_: *mut ENetPeer);
    #[no_mangle]
    fn enet_peer_reset(_: *mut ENetPeer);
    #[no_mangle]
    fn enet_peer_disconnect(_: *mut ENetPeer, _: EnetUint32);
    #[no_mangle]
    fn enet_peer_throttle(_: *mut ENetPeer, _: EnetUint32) -> libc::c_int;
    #[no_mangle]
    fn enet_peer_reset_queues(_: *mut ENetPeer);
    #[no_mangle]
    fn enet_peer_queue_outgoing_command(_: *mut ENetPeer,
                                        _: *const ENetProtocol,
                                        _: *mut ENetPacket, _: EnetUint32,
                                        _: EnetUint16)
     -> *mut ENetOutgoingCommand;
    #[no_mangle]
    fn enet_peer_queue_incoming_command(_: *mut ENetPeer,
                                        _: *const ENetProtocol,
                                        _: *const libc::c_void, _: SizeT,
                                        _: EnetUint32, _: EnetUint32)
     -> *mut ENetIncomingCommand;
    #[no_mangle]
    fn enet_peer_queue_acknowledgement(_: *mut ENetPeer,
                                       _: *const ENetProtocol, _: EnetUint16)
     -> *mut ENetAcknowledgement;
    #[no_mangle]
    fn enet_peer_dispatch_incoming_unreliable_commands(_: *mut ENetPeer,
                                                       _: *mut ENetChannel);
    #[no_mangle]
    fn enet_peer_dispatch_incoming_reliable_commands(_: *mut ENetPeer,
                                                     _: *mut ENetChannel);
    #[no_mangle]
    fn enet_peer_on_connect(_: *mut ENetPeer);
    #[no_mangle]
    fn enet_peer_on_disconnect(_: *mut ENetPeer);
    #[no_mangle]
    fn ntohl(__netlong: Uint32T) -> Uint32T;
    #[no_mangle]
    fn ntohs(__netshort: Uint16T) -> Uint16T;
    #[no_mangle]
    fn htonl(__hostlong: Uint32T) -> Uint32T;
    #[no_mangle]
    fn htons(__hostshort: Uint16T) -> Uint16T;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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
pub type ENetProtocolCommand = _ENetProtocolCommand;
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
pub type _ENetSocketWait = libc::c_uint;
pub const ENET_SOCKET_WAIT_INTERRUPT: _ENetSocketWait = 4;
pub const ENET_SOCKET_WAIT_RECEIVE: _ENetSocketWait = 2;
pub const ENET_SOCKET_WAIT_SEND: _ENetSocketWait = 1;
pub const ENET_SOCKET_WAIT_NONE: _ENetSocketWait = 0;
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
pub type ENetEvent = _ENetEvent;
/* * 
 @file  protocol.c
 @brief ENet protocol functions
*/
static mut commandSizes: [SizeT; 13] =
    [0i32 as SizeT,
     ::std::mem::size_of::<ENetProtocolAcknowledge>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolConnect>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolVerifyConnect>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolDisconnect>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolPing>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolSendReliable>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolSendUnreliable>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolSendFragment>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolSendUnsequenced>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolBandwidthLimit>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolThrottleConfigure>() as libc::c_ulong,
     ::std::mem::size_of::<ENetProtocolSendFragment>() as libc::c_ulong];
#[no_mangle]
pub unsafe extern "C" fn enet_protocol_command_size(mut commandNumber:
                                                        enet_uint8)
 -> SizeT {
    return commandSizes[(commandNumber as libc::c_int &
                             ENET_PROTOCOL_COMMAND_MASK as libc::c_int) as
                            usize];
}
unsafe extern "C" fn enet_protocol_change_state(mut host: *mut ENetHost,
                                                mut peer: *mut ENetPeer,
                                                mut state: ENetPeerState) {
    if state as libc::c_uint ==
           ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint ||
           state as libc::c_uint ==
               ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
       {
        enet_peer_on_connect(peer);
    } else { enet_peer_on_disconnect(peer); }
    (*peer).state = state;
}
unsafe extern "C" fn enet_protocol_dispatch_state(mut host: *mut ENetHost,
                                                  mut peer: *mut ENetPeer,
                                                  mut state: ENetPeerState) {
    enet_protocol_change_state(host, peer, state);
    if 0 == (*peer).needsDispatch {
        enet_list_insert(&mut (*host).dispatchQueue.sentinel,
                         &mut (*peer).dispatchList as *mut ENetListNode as
                             *mut libc::c_void);
        (*peer).needsDispatch = 1i32
    };
}
unsafe extern "C" fn enet_protocol_dispatch_incoming_commands(mut host:
                                                                  *mut ENetHost,
                                                              mut event:
                                                                  *mut ENetEvent)
 -> libc::c_int {
    while !((*host).dispatchQueue.sentinel.next ==
                &mut (*host).dispatchQueue.sentinel as *mut ENetListNode) {
        let mut peer: *mut ENetPeer =
            enet_list_remove((*host).dispatchQueue.sentinel.next) as
                *mut ENetPeer;
        (*peer).needsDispatch = 0i32;
        match (*peer).state as libc::c_uint {
            3 | 4 => {
                enet_protocol_change_state(host, peer,
                                           ENET_PEER_STATE_CONNECTED);
                (*event).type_0 = ENET_EVENT_TYPE_CONNECT;
                (*event).peer = peer;
                (*event).data = (*peer).eventData;
                return 1i32
            }
            9 => {
                (*host).recalculateBandwidthLimits = 1i32;
                (*event).type_0 = ENET_EVENT_TYPE_DISCONNECT;
                (*event).peer = peer;
                (*event).data = (*peer).eventData;
                enet_peer_reset(peer);
                return 1i32
            }
            5 => {
                if (*peer).dispatchedCommands.sentinel.next ==
                       &mut (*peer).dispatchedCommands.sentinel as
                           *mut ENetListNode {
                    continue ;
                }
                (*event).packet =
                    enet_peer_receive(peer, &mut (*event).channelID);
                if (*event).packet.is_null() { continue ; }
                (*event).type_0 = ENET_EVENT_TYPE_RECEIVE;
                (*event).peer = peer;
                if !((*peer).dispatchedCommands.sentinel.next ==
                         &mut (*peer).dispatchedCommands.sentinel as
                             *mut ENetListNode) {
                    (*peer).needsDispatch = 1i32;
                    enet_list_insert(&mut (*host).dispatchQueue.sentinel,
                                     &mut (*peer).dispatchList as
                                         *mut ENetListNode as
                                         *mut libc::c_void);
                }
                return 1i32
            }
            _ => { }
        }
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_notify_connect(mut host: *mut ENetHost,
                                                  mut peer: *mut ENetPeer,
                                                  mut event: *mut ENetEvent) {
    (*host).recalculateBandwidthLimits = 1i32;
    if !event.is_null() {
        enet_protocol_change_state(host, peer, ENET_PEER_STATE_CONNECTED);
        (*event).type_0 = ENET_EVENT_TYPE_CONNECT;
        (*event).peer = peer;
        (*event).data = (*peer).eventData
    } else {
        enet_protocol_dispatch_state(host, peer,
                                     (if (*peer).state as libc::c_uint ==
                                             ENET_PEER_STATE_CONNECTING as
                                                 libc::c_int as libc::c_uint {
                                          ENET_PEER_STATE_CONNECTION_SUCCEEDED
                                              as libc::c_int
                                      } else {
                                          ENET_PEER_STATE_CONNECTION_PENDING
                                              as libc::c_int
                                      }) as ENetPeerState);
    };
}
unsafe extern "C" fn enet_protocol_notify_disconnect(mut host: *mut ENetHost,
                                                     mut peer: *mut ENetPeer,
                                                     mut event:
                                                         *mut ENetEvent) {
    if (*peer).state as libc::c_uint >=
           ENET_PEER_STATE_CONNECTION_PENDING as libc::c_int as libc::c_uint {
        (*host).recalculateBandwidthLimits = 1i32
    }
    if (*peer).state as libc::c_uint !=
           ENET_PEER_STATE_CONNECTING as libc::c_int as libc::c_uint &&
           ((*peer).state as libc::c_uint) <
               ENET_PEER_STATE_CONNECTION_SUCCEEDED as libc::c_int as
                   libc::c_uint {
        enet_peer_reset(peer);
    } else if !event.is_null() {
        (*event).type_0 = ENET_EVENT_TYPE_DISCONNECT;
        (*event).peer = peer;
        (*event).data = 0i32 as EnetUint32;
        enet_peer_reset(peer);
    } else {
        (*peer).eventData = 0i32 as EnetUint32;
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    };
}
unsafe extern "C" fn enet_protocol_remove_sent_unreliable_commands(mut peer:
                                                                       *mut ENetPeer) {
    let mut outgoingCommand: *mut ENetOutgoingCommand =
        0 as *mut ENetOutgoingCommand;
    if (*peer).sentUnreliableCommands.sentinel.next ==
           &mut (*peer).sentUnreliableCommands.sentinel as *mut ENetListNode {
        return
    }
    loop  {
        outgoingCommand =
            (*peer).sentUnreliableCommands.sentinel.next as *mut libc::c_void
                as *mut ENetOutgoingCommand;
        enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
        if !(*outgoingCommand).packet.is_null() {
            (*(*outgoingCommand).packet).referenceCount =
                (*(*outgoingCommand).packet).referenceCount.wrapping_sub(1);
            if (*(*outgoingCommand).packet).referenceCount ==
                   0i32 as libc::c_ulong {
                (*(*outgoingCommand).packet).flags |=
                    ENET_PACKET_FLAG_SENT as libc::c_int as libc::c_uint;
                enet_packet_destroy((*outgoingCommand).packet);
            }
        }
        enet_free(outgoingCommand as *mut libc::c_void);
        if (*peer).sentUnreliableCommands.sentinel.next ==
               &mut (*peer).sentUnreliableCommands.sentinel as
                   *mut ENetListNode {
            break ;
        }
    }
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint &&
           (*peer).outgoingReliableCommands.sentinel.next ==
               &mut (*peer).outgoingReliableCommands.sentinel as
                   *mut ENetListNode &&
           (*peer).outgoingUnreliableCommands.sentinel.next ==
               &mut (*peer).outgoingUnreliableCommands.sentinel as
                   *mut ENetListNode &&
           (*peer).sentReliableCommands.sentinel.next ==
               &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode
       {
        enet_peer_disconnect(peer, (*peer).eventData);
    };
}
unsafe extern "C" fn enet_protocol_remove_sent_reliable_command(mut peer:
                                                                    *mut ENetPeer,
                                                                mut reliableSequenceNumber:
                                                                    EnetUint16,
                                                                mut channelID:
                                                                    enet_uint8)
 -> ENetProtocolCommand {
    let mut outgoingCommand: *mut ENetOutgoingCommand =
        0 as *mut ENetOutgoingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut commandNumber: ENetProtocolCommand = ENET_PROTOCOL_COMMAND_NONE;
    let mut wasSent: libc::c_int = 1i32;
    currentCommand = (*peer).sentReliableCommands.sentinel.next;
    while currentCommand !=
              &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode
          {
        outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
        if (*outgoingCommand).reliableSequenceNumber as libc::c_int ==
               reliableSequenceNumber as libc::c_int &&
               (*outgoingCommand).command.header.channelID as libc::c_int ==
                   channelID as libc::c_int {
            break ;
        }
        currentCommand = (*currentCommand).next
    }
    if currentCommand ==
           &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode {
        currentCommand = (*peer).outgoingReliableCommands.sentinel.next;
        while currentCommand !=
                  &mut (*peer).outgoingReliableCommands.sentinel as
                      *mut ENetListNode {
            outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
            if ((*outgoingCommand).sendAttempts as libc::c_int) < 1i32 {
                return ENET_PROTOCOL_COMMAND_NONE
            }
            if (*outgoingCommand).reliableSequenceNumber as libc::c_int ==
                   reliableSequenceNumber as libc::c_int &&
                   (*outgoingCommand).command.header.channelID as libc::c_int
                       == channelID as libc::c_int {
                break ;
            }
            currentCommand = (*currentCommand).next
        }
        if currentCommand ==
               &mut (*peer).outgoingReliableCommands.sentinel as
                   *mut ENetListNode {
            return ENET_PROTOCOL_COMMAND_NONE
        }
        wasSent = 0i32
    }
    if outgoingCommand.is_null() { return ENET_PROTOCOL_COMMAND_NONE }
    if (channelID as libc::c_ulong) < (*peer).channelCount {
        let mut channel: *mut ENetChannel =
            &mut *(*peer).channels.offset(channelID as isize) as
                *mut ENetChannel;
        let mut reliableWindow: EnetUint16 =
            (reliableSequenceNumber as libc::c_int /
                 ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as
                EnetUint16;
        if (*channel).reliableWindows[reliableWindow as usize] as libc::c_int
               > 0i32 {
            (*channel).reliableWindows[reliableWindow as usize] =
                (*channel).reliableWindows[reliableWindow as
                                               usize].wrapping_sub(1);
            if 0 == (*channel).reliableWindows[reliableWindow as usize] {
                (*channel).usedReliableWindows =
                    ((*channel).usedReliableWindows as libc::c_int &
                         !(1i32 << reliableWindow as libc::c_int)) as
                        EnetUint16
            }
        }
    }
    commandNumber =
        ((*outgoingCommand).command.header.command as libc::c_int &
             ENET_PROTOCOL_COMMAND_MASK as libc::c_int) as
            ENetProtocolCommand;
    enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
    if !(*outgoingCommand).packet.is_null() {
        if 0 != wasSent {
            (*peer).reliableDataInTransit =
                ((*peer).reliableDataInTransit as
                     libc::c_uint).wrapping_sub((*outgoingCommand).fragmentLength
                                                    as libc::c_uint) as
                    EnetUint32 as EnetUint32
        }
        (*(*outgoingCommand).packet).referenceCount =
            (*(*outgoingCommand).packet).referenceCount.wrapping_sub(1);
        if (*(*outgoingCommand).packet).referenceCount ==
               0i32 as libc::c_ulong {
            (*(*outgoingCommand).packet).flags |=
                ENET_PACKET_FLAG_SENT as libc::c_int as libc::c_uint;
            enet_packet_destroy((*outgoingCommand).packet);
        }
    }
    enet_free(outgoingCommand as *mut libc::c_void);
    if (*peer).sentReliableCommands.sentinel.next ==
           &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode {
        return commandNumber
    }
    outgoingCommand =
        (*peer).sentReliableCommands.sentinel.next as *mut libc::c_void as
            *mut ENetOutgoingCommand;
    (*peer).nextTimeout =
        (*outgoingCommand).sentTime.wrapping_add((*outgoingCommand).roundTripTimeout);
    return commandNumber;
}
unsafe extern "C" fn enet_protocol_handle_connect(mut host: *mut ENetHost,
                                                  mut header:
                                                      *mut ENetProtocolHeader,
                                                  mut command:
                                                      *mut ENetProtocol)
 -> *mut ENetPeer {
    let mut incomingSessionID: enet_uint8 = 0;
    let mut outgoingSessionID: enet_uint8 = 0;
    let mut mtu: EnetUint32 = 0;
    let mut windowSize: EnetUint32 = 0;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut channelCount: SizeT = 0;
    let mut duplicatePeers: SizeT = 0i32 as SizeT;
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut peer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut verifyCommand: ENetProtocol =
        _ENetProtocol{header:
                          ENetProtocolCommandHeader{command: 0,
                                                    channelID: 0,
                                                    reliableSequenceNumber:
                                                        0,},};
    channelCount = ntohl((*command).connect.channelCount) as SizeT;
    if channelCount <
           ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong
           ||
           channelCount >
               ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as
                   libc::c_ulong {
        return 0 as *mut ENetPeer
    }
    currentPeer = (*host).peers;
    while currentPeer <
              &mut *(*host).peers.offset((*host).peerCount as isize) as
                  *mut ENetPeer {
        if (*currentPeer).state as libc::c_uint ==
               ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint {
            if peer.is_null() { peer = currentPeer }
        } else if (*currentPeer).state as libc::c_uint !=
                      ENET_PEER_STATE_CONNECTING as libc::c_int as
                          libc::c_uint &&
                      (*currentPeer).address.host ==
                          (*host).receivedAddress.host {
            if (*currentPeer).address.port as libc::c_int ==
                   (*host).receivedAddress.port as libc::c_int &&
                   (*currentPeer).connectID == (*command).connect.connectID {
                return 0 as *mut ENetPeer
            }
            duplicatePeers = duplicatePeers.wrapping_add(1)
        }
        currentPeer = currentPeer.offset(1isize)
    }
    if peer.is_null() || duplicatePeers >= (*host).duplicatePeers {
        return 0 as *mut ENetPeer
    }
    if channelCount > (*host).channelLimit {
        channelCount = (*host).channelLimit
    }
    (*peer).channels =
        enet_malloc(channelCount.wrapping_mul(::std::mem::size_of::<ENetChannel>()
                                                  as libc::c_ulong)) as
            *mut ENetChannel;
    if (*peer).channels.is_null() { return 0 as *mut ENetPeer }
    (*peer).channelCount = channelCount;
    (*peer).state = ENET_PEER_STATE_ACKNOWLEDGING_CONNECT;
    (*peer).connectID = (*command).connect.connectID;
    (*peer).address = (*host).receivedAddress;
    (*peer).outgoingPeerID = ntohs((*command).connect.outgoingPeerID);
    (*peer).incomingBandwidth = ntohl((*command).connect.incomingBandwidth);
    (*peer).outgoingBandwidth = ntohl((*command).connect.outgoingBandwidth);
    (*peer).packetThrottleInterval =
        ntohl((*command).connect.packetThrottleInterval);
    (*peer).packetThrottleAcceleration =
        ntohl((*command).connect.packetThrottleAcceleration);
    (*peer).packetThrottleDeceleration =
        ntohl((*command).connect.packetThrottleDeceleration);
    (*peer).eventData = ntohl((*command).connect.data);
    incomingSessionID =
        (if (*command).connect.incomingSessionID as libc::c_int == 0xffi32 {
             (*peer).outgoingSessionID as libc::c_int
         } else { (*command).connect.incomingSessionID as libc::c_int }) as
            enet_uint8;
    incomingSessionID =
        (incomingSessionID as libc::c_int + 1i32 &
             ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int >>
                 ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int) as
            enet_uint8;
    if incomingSessionID as libc::c_int ==
           (*peer).outgoingSessionID as libc::c_int {
        incomingSessionID =
            (incomingSessionID as libc::c_int + 1i32 &
                 ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int >>
                     ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int) as
                enet_uint8
    }
    (*peer).outgoingSessionID = incomingSessionID;
    outgoingSessionID =
        (if (*command).connect.outgoingSessionID as libc::c_int == 0xffi32 {
             (*peer).incomingSessionID as libc::c_int
         } else { (*command).connect.outgoingSessionID as libc::c_int }) as
            enet_uint8;
    outgoingSessionID =
        (outgoingSessionID as libc::c_int + 1i32 &
             ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int >>
                 ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int) as
            enet_uint8;
    if outgoingSessionID as libc::c_int ==
           (*peer).incomingSessionID as libc::c_int {
        outgoingSessionID =
            (outgoingSessionID as libc::c_int + 1i32 &
                 ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int >>
                     ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int) as
                enet_uint8
    }
    (*peer).incomingSessionID = outgoingSessionID;
    channel = (*peer).channels;
    while channel <
              &mut *(*peer).channels.offset(channelCount as isize) as
                  *mut ENetChannel {
        (*channel).outgoingReliableSequenceNumber = 0i32 as EnetUint16;
        (*channel).outgoingUnreliableSequenceNumber = 0i32 as EnetUint16;
        (*channel).incomingReliableSequenceNumber = 0i32 as EnetUint16;
        (*channel).incomingUnreliableSequenceNumber = 0i32 as EnetUint16;
        enet_list_clear(&mut (*channel).incomingReliableCommands);
        enet_list_clear(&mut (*channel).incomingUnreliableCommands);
        (*channel).usedReliableWindows = 0i32 as EnetUint16;
        memset((*channel).reliableWindows.as_mut_ptr() as *mut libc::c_void,
               0i32,
               ::std::mem::size_of::<[EnetUint16; 16]>() as libc::c_ulong);
        channel = channel.offset(1isize)
    }
    mtu = ntohl((*command).connect.mtu);
    if mtu < ENET_PROTOCOL_MINIMUM_MTU as libc::c_int as libc::c_uint {
        mtu = ENET_PROTOCOL_MINIMUM_MTU as libc::c_int as EnetUint32
    } else if mtu > ENET_PROTOCOL_MAXIMUM_MTU as libc::c_int as libc::c_uint {
        mtu = ENET_PROTOCOL_MAXIMUM_MTU as libc::c_int as EnetUint32
    }
    (*peer).mtu = mtu;
    if (*host).outgoingBandwidth == 0i32 as libc::c_uint &&
           (*peer).incomingBandwidth == 0i32 as libc::c_uint {
        (*peer).windowSize =
            ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    } else if (*host).outgoingBandwidth == 0i32 as libc::c_uint ||
                  (*peer).incomingBandwidth == 0i32 as libc::c_uint {
        (*peer).windowSize =
            (if (*host).outgoingBandwidth > (*peer).incomingBandwidth {
                 (*host).outgoingBandwidth
             } else {
                 (*peer).incomingBandwidth
             }).wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as libc::c_int as
                                 libc::c_uint).wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE
                                                                as libc::c_int
                                                                as
                                                                libc::c_uint)
    } else {
        (*peer).windowSize =
            (if (*host).outgoingBandwidth < (*peer).incomingBandwidth {
                 (*host).outgoingBandwidth
             } else {
                 (*peer).incomingBandwidth
             }).wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as libc::c_int as
                                 libc::c_uint).wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE
                                                                as libc::c_int
                                                                as
                                                                libc::c_uint)
    }
    if (*peer).windowSize <
           ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        (*peer).windowSize =
            ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    } else if (*peer).windowSize >
                  ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as
                      libc::c_uint {
        (*peer).windowSize =
            ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    }
    if (*host).incomingBandwidth == 0i32 as libc::c_uint {
        windowSize =
            ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    } else {
        windowSize =
            (*host).incomingBandwidth.wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE
                                                       as libc::c_int as
                                                       libc::c_uint).wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint)
    }
    if windowSize > ntohl((*command).connect.windowSize) {
        windowSize = ntohl((*command).connect.windowSize)
    }
    if windowSize <
           ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        windowSize =
            ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    } else if windowSize >
                  ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as
                      libc::c_uint {
        windowSize =
            ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    }
    verifyCommand.header.command =
        (ENET_PROTOCOL_COMMAND_VERIFY_CONNECT as libc::c_int |
             ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int) as
            enet_uint8;
    verifyCommand.header.channelID = 0xffi32 as enet_uint8;
    verifyCommand.verifyConnect.outgoingPeerID =
        htons((*peer).incomingPeerID);
    verifyCommand.verifyConnect.incomingSessionID = incomingSessionID;
    verifyCommand.verifyConnect.outgoingSessionID = outgoingSessionID;
    verifyCommand.verifyConnect.mtu = htonl((*peer).mtu);
    verifyCommand.verifyConnect.windowSize = htonl(windowSize);
    verifyCommand.verifyConnect.channelCount =
        htonl(channelCount as Uint32T);
    verifyCommand.verifyConnect.incomingBandwidth =
        htonl((*host).incomingBandwidth);
    verifyCommand.verifyConnect.outgoingBandwidth =
        htonl((*host).outgoingBandwidth);
    verifyCommand.verifyConnect.packetThrottleInterval =
        htonl((*peer).packetThrottleInterval);
    verifyCommand.verifyConnect.packetThrottleAcceleration =
        htonl((*peer).packetThrottleAcceleration);
    verifyCommand.verifyConnect.packetThrottleDeceleration =
        htonl((*peer).packetThrottleDeceleration);
    verifyCommand.verifyConnect.connectID = (*peer).connectID;
    enet_peer_queue_outgoing_command(peer, &mut verifyCommand,
                                     0 as *mut ENetPacket,
                                     0i32 as EnetUint32,
                                     0i32 as EnetUint16);
    return peer;
}
unsafe extern "C" fn enet_protocol_handle_send_reliable(mut host:
                                                            *mut ENetHost,
                                                        mut peer:
                                                            *mut ENetPeer,
                                                        mut command:
                                                            *const ENetProtocol,
                                                        mut currentData:
                                                            *mut *mut enet_uint8)
 -> libc::c_int {
    let mut data_length: SizeT = 0;
    if (*command).header.channelID as libc::c_ulong >= (*peer).channelCount ||
           (*peer).state as libc::c_uint !=
               ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint &&
               (*peer).state as libc::c_uint !=
                   ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                       libc::c_uint {
        return -1i32
    }
    data_length = ntohs((*command).sendReliable.data_length) as SizeT;
    *currentData = (*currentData).offset(data_length as isize);
    if data_length > (*host).maximumPacketSize ||
           *currentData < (*host).receivedData ||
           *currentData >
               &mut *(*host).receivedData.offset((*host).receivedDataLength as
                                                     isize) as *mut enet_uint8
       {
        return -1i32
    }
    if enet_peer_queue_incoming_command(peer, command,
                                        (command as
                                             *const enet_uint8).offset(::std::mem::size_of::<ENetProtocolSendReliable>()
                                                                           as
                                                                           libc::c_ulong
                                                                           as
                                                                           isize)
                                            as *const libc::c_void,
                                        data_length,
                                        ENET_PACKET_FLAG_RELIABLE as
                                            libc::c_int as EnetUint32,
                                        0i32 as EnetUint32).is_null() {
        return -1i32
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_send_unsequenced(mut host:
                                                               *mut ENetHost,
                                                           mut peer:
                                                               *mut ENetPeer,
                                                           mut command:
                                                               *const ENetProtocol,
                                                           mut currentData:
                                                               *mut *mut enet_uint8)
 -> libc::c_int {
    let mut unsequencedGroup: EnetUint32 = 0;
    let mut index: EnetUint32 = 0;
    let mut data_length: SizeT = 0;
    if (*command).header.channelID as libc::c_ulong >= (*peer).channelCount ||
           (*peer).state as libc::c_uint !=
               ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint &&
               (*peer).state as libc::c_uint !=
                   ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                       libc::c_uint {
        return -1i32
    }
    data_length = ntohs((*command).sendUnsequenced.data_length) as SizeT;
    *currentData = (*currentData).offset(data_length as isize);
    if data_length > (*host).maximumPacketSize ||
           *currentData < (*host).receivedData ||
           *currentData >
               &mut *(*host).receivedData.offset((*host).receivedDataLength as
                                                     isize) as *mut enet_uint8
       {
        return -1i32
    }
    unsequencedGroup =
        ntohs((*command).sendUnsequenced.unsequencedGroup) as EnetUint32;
    index =
        unsequencedGroup.wrapping_rem(ENET_PEER_UNSEQUENCED_WINDOW_SIZE as
                                          libc::c_int as libc::c_uint);
    if unsequencedGroup < (*peer).incomingUnsequencedGroup as libc::c_uint {
        unsequencedGroup =
            (unsequencedGroup as
                 libc::c_uint).wrapping_add(0x10000i32 as libc::c_uint) as
                EnetUint32 as EnetUint32
    }
    if unsequencedGroup >=
           ((*peer).incomingUnsequencedGroup as
                EnetUint32).wrapping_add((ENET_PEER_FREE_UNSEQUENCED_WINDOWS
                                               as libc::c_int *
                                               ENET_PEER_UNSEQUENCED_WINDOW_SIZE
                                                   as libc::c_int) as
                                              libc::c_uint) {
        return 0i32
    }
    unsequencedGroup &= 0xffffi32 as libc::c_uint;
    if unsequencedGroup.wrapping_sub(index) !=
           (*peer).incomingUnsequencedGroup as libc::c_uint {
        (*peer).incomingUnsequencedGroup =
            unsequencedGroup.wrapping_sub(index) as EnetUint16;
        memset((*peer).unsequencedWindow.as_mut_ptr() as *mut libc::c_void,
               0i32,
               ::std::mem::size_of::<[EnetUint32; 32]>() as libc::c_ulong);
    } else if 0 !=
                  (*peer).unsequencedWindow[index.wrapping_div(32i32 as
                                                                   libc::c_uint)
                                                as usize] &
                      (1i32 << index.wrapping_rem(32i32 as libc::c_uint)) as
                          libc::c_uint {
        return 0i32
    }
    if enet_peer_queue_incoming_command(peer, command,
                                        (command as
                                             *const enet_uint8).offset(::std::mem::size_of::<ENetProtocolSendUnsequenced>()
                                                                           as
                                                                           libc::c_ulong
                                                                           as
                                                                           isize)
                                            as *const libc::c_void,
                                        data_length,
                                        ENET_PACKET_FLAG_UNSEQUENCED as
                                            libc::c_int as EnetUint32,
                                        0i32 as EnetUint32).is_null() {
        return -1i32
    }
    (*peer).unsequencedWindow[index.wrapping_div(32i32 as libc::c_uint) as
                                  usize] |=
        (1i32 << index.wrapping_rem(32i32 as libc::c_uint)) as libc::c_uint;
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_send_unreliable(mut host:
                                                              *mut ENetHost,
                                                          mut peer:
                                                              *mut ENetPeer,
                                                          mut command:
                                                              *const ENetProtocol,
                                                          mut currentData:
                                                              *mut *mut enet_uint8)
 -> libc::c_int {
    let mut data_length: SizeT = 0;
    if (*command).header.channelID as libc::c_ulong >= (*peer).channelCount ||
           (*peer).state as libc::c_uint !=
               ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint &&
               (*peer).state as libc::c_uint !=
                   ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                       libc::c_uint {
        return -1i32
    }
    data_length = ntohs((*command).sendUnreliable.data_length) as SizeT;
    *currentData = (*currentData).offset(data_length as isize);
    if data_length > (*host).maximumPacketSize ||
           *currentData < (*host).receivedData ||
           *currentData >
               &mut *(*host).receivedData.offset((*host).receivedDataLength as
                                                     isize) as *mut enet_uint8
       {
        return -1i32
    }
    if enet_peer_queue_incoming_command(peer, command,
                                        (command as
                                             *const enet_uint8).offset(::std::mem::size_of::<ENetProtocolSendUnreliable>()
                                                                           as
                                                                           libc::c_ulong
                                                                           as
                                                                           isize)
                                            as *const libc::c_void,
                                        data_length, 0i32 as EnetUint32,
                                        0i32 as EnetUint32).is_null() {
        return -1i32
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_send_fragment(mut host:
                                                            *mut ENetHost,
                                                        mut peer:
                                                            *mut ENetPeer,
                                                        mut command:
                                                            *const ENetProtocol,
                                                        mut currentData:
                                                            *mut *mut enet_uint8)
 -> libc::c_int {
    let mut fragmentNumber: EnetUint32 = 0;
    let mut fragmentCount: EnetUint32 = 0;
    let mut fragmentOffset: EnetUint32 = 0;
    let mut fragmentLength: EnetUint32 = 0;
    let mut startSequenceNumber: EnetUint32 = 0;
    let mut totalLength: EnetUint32 = 0;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut startWindow: EnetUint16 = 0;
    let mut currentWindow: EnetUint16 = 0;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut startCommand: *mut ENetIncomingCommand =
        0 as *mut ENetIncomingCommand;
    if (*command).header.channelID as libc::c_ulong >= (*peer).channelCount ||
           (*peer).state as libc::c_uint !=
               ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint &&
               (*peer).state as libc::c_uint !=
                   ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                       libc::c_uint {
        return -1i32
    }
    fragmentLength = ntohs((*command).sendFragment.data_length) as EnetUint32;
    *currentData = (*currentData).offset(fragmentLength as isize);
    if fragmentLength as libc::c_ulong > (*host).maximumPacketSize ||
           *currentData < (*host).receivedData ||
           *currentData >
               &mut *(*host).receivedData.offset((*host).receivedDataLength as
                                                     isize) as *mut enet_uint8
       {
        return -1i32
    }
    channel =
        &mut *(*peer).channels.offset((*command).header.channelID as isize) as
            *mut ENetChannel;
    startSequenceNumber =
        ntohs((*command).sendFragment.startSequenceNumber) as EnetUint32;
    startWindow =
        startSequenceNumber.wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as
                                             libc::c_int as libc::c_uint) as
            EnetUint16;
    currentWindow =
        ((*channel).incomingReliableSequenceNumber as libc::c_int /
             ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as EnetUint16;
    if startSequenceNumber <
           (*channel).incomingReliableSequenceNumber as libc::c_uint {
        startWindow =
            (startWindow as libc::c_int +
                 ENET_PEER_RELIABLE_WINDOWS as libc::c_int) as EnetUint16
    }
    if (startWindow as libc::c_int) < currentWindow as libc::c_int ||
           startWindow as libc::c_int >=
               currentWindow as libc::c_int +
                   ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int - 1i32 {
        return 0i32
    }
    fragmentNumber = ntohl((*command).sendFragment.fragmentNumber);
    fragmentCount = ntohl((*command).sendFragment.fragmentCount);
    fragmentOffset = ntohl((*command).sendFragment.fragmentOffset);
    totalLength = ntohl((*command).sendFragment.totalLength);
    if fragmentCount >
           ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as libc::c_int as libc::c_uint
           || fragmentNumber >= fragmentCount ||
           totalLength as libc::c_ulong > (*host).maximumPacketSize ||
           fragmentOffset >= totalLength ||
           fragmentLength > totalLength.wrapping_sub(fragmentOffset) {
        return -1i32
    }
    let mut current_block_23: u64;
    currentCommand = (*channel).incomingReliableCommands.sentinel.previous;
    while currentCommand !=
              &mut (*channel).incomingReliableCommands.sentinel as
                  *mut ENetListNode {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if startSequenceNumber >=
               (*channel).incomingReliableSequenceNumber as libc::c_uint {
            if ((*incomingCommand).reliableSequenceNumber as libc::c_int) <
                   (*channel).incomingReliableSequenceNumber as libc::c_int {
                current_block_23 = 13797916685926291137;
            } else { current_block_23 = 7172762164747879670; }
        } else {
            if (*incomingCommand).reliableSequenceNumber as libc::c_int >=
                   (*channel).incomingReliableSequenceNumber as libc::c_int {
                break ;
            }
            current_block_23 = 7172762164747879670;
        }
        match current_block_23 {
            7172762164747879670 => {
                if (*incomingCommand).reliableSequenceNumber as libc::c_uint
                       <= startSequenceNumber {
                    if ((*incomingCommand).reliableSequenceNumber as
                            libc::c_uint) < startSequenceNumber {
                        break ;
                    }
                    if (*incomingCommand).command.header.command as
                           libc::c_int &
                           ENET_PROTOCOL_COMMAND_MASK as libc::c_int !=
                           ENET_PROTOCOL_COMMAND_SEND_FRAGMENT as libc::c_int
                           ||
                           totalLength as libc::c_ulong !=
                               (*(*incomingCommand).packet).data_length ||
                           fragmentCount != (*incomingCommand).fragmentCount {
                        return -1i32
                    }
                    startCommand = incomingCommand;
                    break ;
                }
            }
            _ => { }
        }
        currentCommand = (*currentCommand).previous
    }
    if startCommand.is_null() {
        let mut hostCommand: ENetProtocol = *command;
        hostCommand.header.reliableSequenceNumber =
            startSequenceNumber as EnetUint16;
        startCommand =
            enet_peer_queue_incoming_command(peer, &mut hostCommand,
                                             0 as *const libc::c_void,
                                             totalLength as SizeT,
                                             ENET_PACKET_FLAG_RELIABLE as
                                                 libc::c_int as EnetUint32,
                                             fragmentCount);
        if startCommand.is_null() { return -1i32 }
    }
    if *(*startCommand).fragments.offset(fragmentNumber.wrapping_div(32i32 as
                                                                         libc::c_uint)
                                             as isize) &
           (1i32 << fragmentNumber.wrapping_rem(32i32 as libc::c_uint)) as
               libc::c_uint == 0i32 as libc::c_uint {
        (*startCommand).fragmentsRemaining =
            (*startCommand).fragmentsRemaining.wrapping_sub(1);
        let ref mut fresh0 =
            *(*startCommand).fragments.offset(fragmentNumber.wrapping_div(32i32
                                                                              as
                                                                              libc::c_uint)
                                                  as isize);
        *fresh0 |=
            (1i32 << fragmentNumber.wrapping_rem(32i32 as libc::c_uint)) as
                libc::c_uint;
        if fragmentOffset.wrapping_add(fragmentLength) as libc::c_ulong >
               (*(*startCommand).packet).data_length {
            fragmentLength =
                (*(*startCommand).packet).data_length.wrapping_sub(fragmentOffset
                                                                      as
                                                                      libc::c_ulong)
                    as EnetUint32
        }
        memcpy((*(*startCommand).packet).data.offset(fragmentOffset as isize)
                   as *mut libc::c_void,
               (command as
                    *mut enet_uint8).offset(::std::mem::size_of::<ENetProtocolSendFragment>()
                                                as libc::c_ulong as isize) as
                   *const libc::c_void, fragmentLength as libc::c_ulong);
        if (*startCommand).fragmentsRemaining <= 0i32 as libc::c_uint {
            enet_peer_dispatch_incoming_reliable_commands(peer, channel);
        }
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_send_unreliable_fragment(mut host:
                                                                       *mut ENetHost,
                                                                   mut peer:
                                                                       *mut ENetPeer,
                                                                   mut command:
                                                                       *const ENetProtocol,
                                                                   mut currentData:
                                                                       *mut *mut enet_uint8)
 -> libc::c_int {
    let mut fragmentNumber: EnetUint32 = 0;
    let mut fragmentCount: EnetUint32 = 0;
    let mut fragmentOffset: EnetUint32 = 0;
    let mut fragmentLength: EnetUint32 = 0;
    let mut reliableSequenceNumber: EnetUint32 = 0;
    let mut startSequenceNumber: EnetUint32 = 0;
    let mut totalLength: EnetUint32 = 0;
    let mut reliableWindow: EnetUint16 = 0;
    let mut currentWindow: EnetUint16 = 0;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut startCommand: *mut ENetIncomingCommand =
        0 as *mut ENetIncomingCommand;
    if (*command).header.channelID as libc::c_ulong >= (*peer).channelCount ||
           (*peer).state as libc::c_uint !=
               ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint &&
               (*peer).state as libc::c_uint !=
                   ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                       libc::c_uint {
        return -1i32
    }
    fragmentLength = ntohs((*command).sendFragment.data_length) as EnetUint32;
    *currentData = (*currentData).offset(fragmentLength as isize);
    if fragmentLength as libc::c_ulong > (*host).maximumPacketSize ||
           *currentData < (*host).receivedData ||
           *currentData >
               &mut *(*host).receivedData.offset((*host).receivedDataLength as
                                                     isize) as *mut enet_uint8
       {
        return -1i32
    }
    channel =
        &mut *(*peer).channels.offset((*command).header.channelID as isize) as
            *mut ENetChannel;
    reliableSequenceNumber =
        (*command).header.reliableSequenceNumber as EnetUint32;
    startSequenceNumber =
        ntohs((*command).sendFragment.startSequenceNumber) as EnetUint32;
    reliableWindow =
        reliableSequenceNumber.wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as
                                                libc::c_int as libc::c_uint)
            as EnetUint16;
    currentWindow =
        ((*channel).incomingReliableSequenceNumber as libc::c_int /
             ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as EnetUint16;
    if reliableSequenceNumber <
           (*channel).incomingReliableSequenceNumber as libc::c_uint {
        reliableWindow =
            (reliableWindow as libc::c_int +
                 ENET_PEER_RELIABLE_WINDOWS as libc::c_int) as EnetUint16
    }
    if (reliableWindow as libc::c_int) < currentWindow as libc::c_int ||
           reliableWindow as libc::c_int >=
               currentWindow as libc::c_int +
                   ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int - 1i32 {
        return 0i32
    }
    if reliableSequenceNumber ==
           (*channel).incomingReliableSequenceNumber as libc::c_uint &&
           startSequenceNumber <=
               (*channel).incomingUnreliableSequenceNumber as libc::c_uint {
        return 0i32
    }
    fragmentNumber = ntohl((*command).sendFragment.fragmentNumber);
    fragmentCount = ntohl((*command).sendFragment.fragmentCount);
    fragmentOffset = ntohl((*command).sendFragment.fragmentOffset);
    totalLength = ntohl((*command).sendFragment.totalLength);
    if fragmentCount >
           ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as libc::c_int as libc::c_uint
           || fragmentNumber >= fragmentCount ||
           totalLength as libc::c_ulong > (*host).maximumPacketSize ||
           fragmentOffset >= totalLength ||
           fragmentLength > totalLength.wrapping_sub(fragmentOffset) {
        return -1i32
    }
    let mut current_block_26: u64;
    currentCommand = (*channel).incomingUnreliableCommands.sentinel.previous;
    while currentCommand !=
              &mut (*channel).incomingUnreliableCommands.sentinel as
                  *mut ENetListNode {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if reliableSequenceNumber >=
               (*channel).incomingReliableSequenceNumber as libc::c_uint {
            if ((*incomingCommand).reliableSequenceNumber as libc::c_int) <
                   (*channel).incomingReliableSequenceNumber as libc::c_int {
                current_block_26 = 18317007320854588510;
            } else { current_block_26 = 15345278821338558188; }
        } else {
            if (*incomingCommand).reliableSequenceNumber as libc::c_int >=
                   (*channel).incomingReliableSequenceNumber as libc::c_int {
                break ;
            }
            current_block_26 = 15345278821338558188;
        }
        match current_block_26 {
            15345278821338558188 => {
                if ((*incomingCommand).reliableSequenceNumber as libc::c_uint)
                       < reliableSequenceNumber {
                    break ;
                }
                if !((*incomingCommand).reliableSequenceNumber as libc::c_uint
                         > reliableSequenceNumber) {
                    if (*incomingCommand).unreliableSequenceNumber as
                           libc::c_uint <= startSequenceNumber {
                        if ((*incomingCommand).unreliableSequenceNumber as
                                libc::c_uint) < startSequenceNumber {
                            break ;
                        }
                        if (*incomingCommand).command.header.command as
                               libc::c_int &
                               ENET_PROTOCOL_COMMAND_MASK as libc::c_int !=
                               ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT
                                   as libc::c_int ||
                               totalLength as libc::c_ulong !=
                                   (*(*incomingCommand).packet).data_length ||
                               fragmentCount !=
                                   (*incomingCommand).fragmentCount {
                            return -1i32
                        }
                        startCommand = incomingCommand;
                        break ;
                    }
                }
            }
            _ => { }
        }
        currentCommand = (*currentCommand).previous
    }
    if startCommand.is_null() {
        startCommand =
            enet_peer_queue_incoming_command(peer, command,
                                             0 as *const libc::c_void,
                                             totalLength as SizeT,
                                             ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT
                                                 as libc::c_int as
                                                 EnetUint32, fragmentCount);
        if startCommand.is_null() { return -1i32 }
    }
    if *(*startCommand).fragments.offset(fragmentNumber.wrapping_div(32i32 as
                                                                         libc::c_uint)
                                             as isize) &
           (1i32 << fragmentNumber.wrapping_rem(32i32 as libc::c_uint)) as
               libc::c_uint == 0i32 as libc::c_uint {
        (*startCommand).fragmentsRemaining =
            (*startCommand).fragmentsRemaining.wrapping_sub(1);
        let ref mut fresh1 =
            *(*startCommand).fragments.offset(fragmentNumber.wrapping_div(32i32
                                                                              as
                                                                              libc::c_uint)
                                                  as isize);
        *fresh1 |=
            (1i32 << fragmentNumber.wrapping_rem(32i32 as libc::c_uint)) as
                libc::c_uint;
        if fragmentOffset.wrapping_add(fragmentLength) as libc::c_ulong >
               (*(*startCommand).packet).data_length {
            fragmentLength =
                (*(*startCommand).packet).data_length.wrapping_sub(fragmentOffset
                                                                      as
                                                                      libc::c_ulong)
                    as EnetUint32
        }
        memcpy((*(*startCommand).packet).data.offset(fragmentOffset as isize)
                   as *mut libc::c_void,
               (command as
                    *mut enet_uint8).offset(::std::mem::size_of::<ENetProtocolSendFragment>()
                                                as libc::c_ulong as isize) as
                   *const libc::c_void, fragmentLength as libc::c_ulong);
        if (*startCommand).fragmentsRemaining <= 0i32 as libc::c_uint {
            enet_peer_dispatch_incoming_unreliable_commands(peer, channel);
        }
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_ping(mut host: *mut ENetHost,
                                               mut peer: *mut ENetPeer,
                                               mut command:
                                                   *const ENetProtocol)
 -> libc::c_int {
    if (*peer).state as libc::c_uint !=
           ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint &&
           (*peer).state as libc::c_uint !=
               ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
       {
        return -1i32
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_bandwidth_limit(mut host:
                                                              *mut ENetHost,
                                                          mut peer:
                                                              *mut ENetPeer,
                                                          mut command:
                                                              *const ENetProtocol)
 -> libc::c_int {
    if (*peer).state as libc::c_uint !=
           ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint &&
           (*peer).state as libc::c_uint !=
               ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
       {
        return -1i32
    }
    if (*peer).incomingBandwidth != 0i32 as libc::c_uint {
        (*host).bandwidthLimitedPeers =
            (*host).bandwidthLimitedPeers.wrapping_sub(1)
    }
    (*peer).incomingBandwidth =
        ntohl((*command).bandwidthLimit.incomingBandwidth);
    (*peer).outgoingBandwidth =
        ntohl((*command).bandwidthLimit.outgoingBandwidth);
    if (*peer).incomingBandwidth != 0i32 as libc::c_uint {
        (*host).bandwidthLimitedPeers =
            (*host).bandwidthLimitedPeers.wrapping_add(1)
    }
    if (*peer).incomingBandwidth == 0i32 as libc::c_uint &&
           (*host).outgoingBandwidth == 0i32 as libc::c_uint {
        (*peer).windowSize =
            ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    } else if (*peer).incomingBandwidth == 0i32 as libc::c_uint ||
                  (*host).outgoingBandwidth == 0i32 as libc::c_uint {
        (*peer).windowSize =
            (if (*peer).incomingBandwidth > (*host).outgoingBandwidth {
                 (*peer).incomingBandwidth
             } else {
                 (*host).outgoingBandwidth
             }).wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as libc::c_int as
                                 libc::c_uint).wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE
                                                                as libc::c_int
                                                                as
                                                                libc::c_uint)
    } else {
        (*peer).windowSize =
            (if (*peer).incomingBandwidth < (*host).outgoingBandwidth {
                 (*peer).incomingBandwidth
             } else {
                 (*host).outgoingBandwidth
             }).wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as libc::c_int as
                                 libc::c_uint).wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE
                                                                as libc::c_int
                                                                as
                                                                libc::c_uint)
    }
    if (*peer).windowSize <
           ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        (*peer).windowSize =
            ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    } else if (*peer).windowSize >
                  ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as
                      libc::c_uint {
        (*peer).windowSize =
            ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_throttle_configure(mut host:
                                                                 *mut ENetHost,
                                                             mut peer:
                                                                 *mut ENetPeer,
                                                             mut command:
                                                                 *const ENetProtocol)
 -> libc::c_int {
    if (*peer).state as libc::c_uint !=
           ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint &&
           (*peer).state as libc::c_uint !=
               ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
       {
        return -1i32
    }
    (*peer).packetThrottleInterval =
        ntohl((*command).throttleConfigure.packetThrottleInterval);
    (*peer).packetThrottleAcceleration =
        ntohl((*command).throttleConfigure.packetThrottleAcceleration);
    (*peer).packetThrottleDeceleration =
        ntohl((*command).throttleConfigure.packetThrottleDeceleration);
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_disconnect(mut host: *mut ENetHost,
                                                     mut peer: *mut ENetPeer,
                                                     mut command:
                                                         *const ENetProtocol)
 -> libc::c_int {
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT as libc::c_int as
                   libc::c_uint {
        return 0i32
    }
    enet_peer_reset_queues(peer);
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_CONNECTION_SUCCEEDED as libc::c_int as libc::c_uint
           ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_DISCONNECTING as libc::c_int as libc::c_uint ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_CONNECTING as libc::c_int as libc::c_uint {
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    } else if (*peer).state as libc::c_uint !=
                  ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint &&
                  (*peer).state as libc::c_uint !=
                      ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                          libc::c_uint {
        if (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_CONNECTION_PENDING as libc::c_int as
                   libc::c_uint {
            (*host).recalculateBandwidthLimits = 1i32
        }
        enet_peer_reset(peer);
    } else if 0 !=
                  (*command).header.command as libc::c_int &
                      ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int {
        enet_protocol_change_state(host, peer,
                                   ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT);
    } else {
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    }
    if (*peer).state as libc::c_uint !=
           ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint {
        (*peer).eventData = ntohl((*command).disconnect.data)
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_acknowledge(mut host: *mut ENetHost,
                                                      mut event:
                                                          *mut ENetEvent,
                                                      mut peer: *mut ENetPeer,
                                                      mut command:
                                                          *const ENetProtocol)
 -> libc::c_int {
    let mut roundTripTime: EnetUint32 = 0;
    let mut receivedSentTime: EnetUint32 = 0;
    let mut receivedReliableSequenceNumber: EnetUint32 = 0;
    let mut commandNumber: ENetProtocolCommand = ENET_PROTOCOL_COMMAND_NONE;
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint ||
           (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint {
        return 0i32
    }
    receivedSentTime =
        ntohs((*command).acknowledge.receivedSentTime) as EnetUint32;
    receivedSentTime |= (*host).serviceTime & 0xffff0000u32;
    if receivedSentTime & 0x8000i32 as libc::c_uint >
           (*host).serviceTime & 0x8000i32 as libc::c_uint {
        receivedSentTime =
            (receivedSentTime as
                 libc::c_uint).wrapping_sub(0x10000i32 as libc::c_uint) as
                EnetUint32 as EnetUint32
    }
    if (*host).serviceTime.wrapping_sub(receivedSentTime) >=
           86400000i32 as libc::c_uint {
        return 0i32
    }
    (*peer).lastReceiveTime = (*host).serviceTime;
    (*peer).earliestTimeout = 0i32 as EnetUint32;
    roundTripTime =
        if (*host).serviceTime.wrapping_sub(receivedSentTime) >=
               86400000i32 as libc::c_uint {
            receivedSentTime.wrapping_sub((*host).serviceTime)
        } else { (*host).serviceTime.wrapping_sub(receivedSentTime) };
    enet_peer_throttle(peer, roundTripTime);
    (*peer).roundTripTimeVariance =
        ((*peer).roundTripTimeVariance as
             libc::c_uint).wrapping_sub((*peer).roundTripTimeVariance.wrapping_div(4i32
                                                                                       as
                                                                                       libc::c_uint))
            as EnetUint32 as EnetUint32;
    if roundTripTime >= (*peer).roundTripTime {
        (*peer).roundTripTime =
            ((*peer).roundTripTime as
                 libc::c_uint).wrapping_add(roundTripTime.wrapping_sub((*peer).roundTripTime).wrapping_div(8i32
                                                                                                               as
                                                                                                               libc::c_uint))
                as EnetUint32 as EnetUint32;
        (*peer).roundTripTimeVariance =
            ((*peer).roundTripTimeVariance as
                 libc::c_uint).wrapping_add(roundTripTime.wrapping_sub((*peer).roundTripTime).wrapping_div(4i32
                                                                                                               as
                                                                                                               libc::c_uint))
                as EnetUint32 as EnetUint32
    } else {
        (*peer).roundTripTime =
            ((*peer).roundTripTime as
                 libc::c_uint).wrapping_sub((*peer).roundTripTime.wrapping_sub(roundTripTime).wrapping_div(8i32
                                                                                                               as
                                                                                                               libc::c_uint))
                as EnetUint32 as EnetUint32;
        (*peer).roundTripTimeVariance =
            ((*peer).roundTripTimeVariance as
                 libc::c_uint).wrapping_add((*peer).roundTripTime.wrapping_sub(roundTripTime).wrapping_div(4i32
                                                                                                               as
                                                                                                               libc::c_uint))
                as EnetUint32 as EnetUint32
    }
    if (*peer).roundTripTime < (*peer).lowestRoundTripTime {
        (*peer).lowestRoundTripTime = (*peer).roundTripTime
    }
    if (*peer).roundTripTimeVariance > (*peer).highestRoundTripTimeVariance {
        (*peer).highestRoundTripTimeVariance = (*peer).roundTripTimeVariance
    }
    if (*peer).packetThrottleEpoch == 0i32 as libc::c_uint ||
           (if (*host).serviceTime.wrapping_sub((*peer).packetThrottleEpoch)
                   >= 86400000i32 as libc::c_uint {
                (*peer).packetThrottleEpoch.wrapping_sub((*host).serviceTime)
            } else {
                (*host).serviceTime.wrapping_sub((*peer).packetThrottleEpoch)
            }) >= (*peer).packetThrottleInterval {
        (*peer).lastRoundTripTime = (*peer).lowestRoundTripTime;
        (*peer).lastRoundTripTimeVariance =
            (*peer).highestRoundTripTimeVariance;
        (*peer).lowestRoundTripTime = (*peer).roundTripTime;
        (*peer).highestRoundTripTimeVariance = (*peer).roundTripTimeVariance;
        (*peer).packetThrottleEpoch = (*host).serviceTime
    }
    receivedReliableSequenceNumber =
        ntohs((*command).acknowledge.receivedReliableSequenceNumber) as
            EnetUint32;
    commandNumber =
        enet_protocol_remove_sent_reliable_command(peer,
                                                   receivedReliableSequenceNumber
                                                       as EnetUint16,
                                                   (*command).header.channelID);
    match (*peer).state as libc::c_uint {
        2 => {
            if commandNumber as libc::c_uint !=
                   ENET_PROTOCOL_COMMAND_VERIFY_CONNECT as libc::c_int as
                       libc::c_uint {
                return -1i32
            }
            enet_protocol_notify_connect(host, peer, event);
        }
        7 => {
            if commandNumber as libc::c_uint !=
                   ENET_PROTOCOL_COMMAND_DISCONNECT as libc::c_int as
                       libc::c_uint {
                return -1i32
            }
            enet_protocol_notify_disconnect(host, peer, event);
        }
        6 => {
            if (*peer).outgoingReliableCommands.sentinel.next ==
                   &mut (*peer).outgoingReliableCommands.sentinel as
                       *mut ENetListNode &&
                   (*peer).outgoingUnreliableCommands.sentinel.next ==
                       &mut (*peer).outgoingUnreliableCommands.sentinel as
                           *mut ENetListNode &&
                   (*peer).sentReliableCommands.sentinel.next ==
                       &mut (*peer).sentReliableCommands.sentinel as
                           *mut ENetListNode {
                enet_peer_disconnect(peer, (*peer).eventData);
            }
        }
        _ => { }
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_verify_connect(mut host:
                                                             *mut ENetHost,
                                                         mut event:
                                                             *mut ENetEvent,
                                                         mut peer:
                                                             *mut ENetPeer,
                                                         mut command:
                                                             *const ENetProtocol)
 -> libc::c_int {
    let mut mtu: EnetUint32 = 0;
    let mut windowSize: EnetUint32 = 0;
    let mut channelCount: SizeT = 0;
    if (*peer).state as libc::c_uint !=
           ENET_PEER_STATE_CONNECTING as libc::c_int as libc::c_uint {
        return 0i32
    }
    channelCount = ntohl((*command).verifyConnect.channelCount) as SizeT;
    if channelCount <
           ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong
           ||
           channelCount >
               ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as
                   libc::c_ulong ||
           ntohl((*command).verifyConnect.packetThrottleInterval) !=
               (*peer).packetThrottleInterval ||
           ntohl((*command).verifyConnect.packetThrottleAcceleration) !=
               (*peer).packetThrottleAcceleration ||
           ntohl((*command).verifyConnect.packetThrottleDeceleration) !=
               (*peer).packetThrottleDeceleration ||
           (*command).verifyConnect.connectID != (*peer).connectID {
        (*peer).eventData = 0i32 as EnetUint32;
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
        return -1i32
    }
    enet_protocol_remove_sent_reliable_command(peer, 1i32 as EnetUint16,
                                               0xffi32 as enet_uint8);
    if channelCount < (*peer).channelCount {
        (*peer).channelCount = channelCount
    }
    (*peer).outgoingPeerID = ntohs((*command).verifyConnect.outgoingPeerID);
    (*peer).incomingSessionID = (*command).verifyConnect.incomingSessionID;
    (*peer).outgoingSessionID = (*command).verifyConnect.outgoingSessionID;
    mtu = ntohl((*command).verifyConnect.mtu);
    if mtu < ENET_PROTOCOL_MINIMUM_MTU as libc::c_int as libc::c_uint {
        mtu = ENET_PROTOCOL_MINIMUM_MTU as libc::c_int as EnetUint32
    } else if mtu > ENET_PROTOCOL_MAXIMUM_MTU as libc::c_int as libc::c_uint {
        mtu = ENET_PROTOCOL_MAXIMUM_MTU as libc::c_int as EnetUint32
    }
    if mtu < (*peer).mtu { (*peer).mtu = mtu }
    windowSize = ntohl((*command).verifyConnect.windowSize);
    if windowSize <
           ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        windowSize =
            ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    }
    if windowSize >
           ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        windowSize =
            ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    }
    if windowSize < (*peer).windowSize { (*peer).windowSize = windowSize }
    (*peer).incomingBandwidth =
        ntohl((*command).verifyConnect.incomingBandwidth);
    (*peer).outgoingBandwidth =
        ntohl((*command).verifyConnect.outgoingBandwidth);
    enet_protocol_notify_connect(host, peer, event);
    return 0i32;
}
unsafe extern "C" fn enet_protocol_handle_incoming_commands(mut host:
                                                                *mut ENetHost,
                                                            mut event:
                                                                *mut ENetEvent)
 -> libc::c_int {
    let mut header: *mut ENetProtocolHeader = 0 as *mut ENetProtocolHeader;
    let mut command: *mut ENetProtocol = 0 as *mut ENetProtocol;
    let mut peer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut currentData: *mut enet_uint8 = 0 as *mut enet_uint8;
    let mut headerSize: SizeT = 0;
    let mut peerID: EnetUint16 = 0;
    let mut flags: EnetUint16 = 0;
    let mut sessionID: enet_uint8 = 0;
    if (*host).receivedDataLength <
           &mut (*(0 as *mut ENetProtocolHeader)).sentTime as *mut EnetUint16
               as SizeT {
        return 0i32
    }
    header = (*host).receivedData as *mut ENetProtocolHeader;
    peerID = ntohs((*header).peerID);
    sessionID =
        ((peerID as libc::c_int &
              ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int) >>
             ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int) as enet_uint8;
    flags =
        (peerID as libc::c_int &
             ENET_PROTOCOL_HEADER_FLAG_MASK as libc::c_int) as EnetUint16;
    peerID =
        (peerID as libc::c_int &
             !(ENET_PROTOCOL_HEADER_FLAG_MASK as libc::c_int |
                   ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int)) as
            EnetUint16;
    headerSize =
        if 0 !=
               flags as libc::c_int &
                   ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as libc::c_int {
            ::std::mem::size_of::<ENetProtocolHeader>() as libc::c_ulong
        } else {
            &mut (*(0 as *mut ENetProtocolHeader)).sentTime as
                *mut EnetUint16 as SizeT
        };
    if (*host).checksum.is_some() {
        headerSize =
            (headerSize as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<EnetUint32>()
                                                 as libc::c_ulong) as SizeT
                as SizeT
    }
    if peerID as libc::c_int == ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int {
        peer = 0 as *mut ENetPeer
    } else if peerID as libc::c_ulong >= (*host).peerCount {
        return 0i32
    } else {
        peer = &mut *(*host).peers.offset(peerID as isize) as *mut ENetPeer;
        if (*peer).state as libc::c_uint ==
               ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint ||
               (*peer).state as libc::c_uint ==
                   ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint ||
               ((*host).receivedAddress.host != (*peer).address.host ||
                    (*host).receivedAddress.port as libc::c_int !=
                        (*peer).address.port as libc::c_int) &&
                   (*peer).address.host != 0xffffffffu32 ||
               ((*peer).outgoingPeerID as libc::c_int) <
                   ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int &&
                   sessionID as libc::c_int !=
                       (*peer).incomingSessionID as libc::c_int {
            return 0i32
        }
    }
    if 0 !=
           flags as libc::c_int &
               ENET_PROTOCOL_HEADER_FLAG_COMPRESSED as libc::c_int {
        let mut originalSize: SizeT = 0;
        if (*host).compressor.context.is_null() ||
               (*host).compressor.decompress.is_none() {
            return 0i32
        }
        originalSize =
            (*host).compressor.decompress.expect("non-null function pointer")((*host).compressor.context,
                                                                              (*host).receivedData.offset(headerSize
                                                                                                              as
                                                                                                              isize),
                                                                              (*host).receivedDataLength.wrapping_sub(headerSize),
                                                                              (*host).packetData[1usize].as_mut_ptr().offset(headerSize
                                                                                                                                 as
                                                                                                                                 isize),
                                                                              (::std::mem::size_of::<[enet_uint8; 4096]>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_sub(headerSize));
        if originalSize <= 0i32 as libc::c_ulong ||
               originalSize >
                   (::std::mem::size_of::<[enet_uint8; 4096]>() as
                        libc::c_ulong).wrapping_sub(headerSize) {
            return 0i32
        }
        memcpy((*host).packetData[1usize].as_mut_ptr() as *mut libc::c_void,
               header as *const libc::c_void, headerSize);
        (*host).receivedData = (*host).packetData[1usize].as_mut_ptr();
        (*host).receivedDataLength = headerSize.wrapping_add(originalSize)
    }
    if (*host).checksum.is_some() {
        let mut checksum: *mut EnetUint32 =
            &mut *(*host).receivedData.offset(headerSize.wrapping_sub(::std::mem::size_of::<EnetUint32>()
                                                                          as
                                                                          libc::c_ulong)
                                                  as isize) as *mut enet_uint8
                as *mut EnetUint32;
        let mut desiredChecksum: EnetUint32 = *checksum;
        let mut buffer: ENetBuffer =
            ENetBuffer{data: 0 as *mut libc::c_void, data_length: 0,};
        *checksum =
            if !peer.is_null() {
                (*peer).connectID
            } else { 0i32 as libc::c_uint };
        buffer.data = (*host).receivedData as *mut libc::c_void;
        buffer.data_length = (*host).receivedDataLength;
        if (*host).checksum.expect("non-null function pointer")(&mut buffer,
                                                                1i32 as
                                                                    SizeT) !=
               desiredChecksum {
            return 0i32
        }
    }
    if !peer.is_null() {
        (*peer).address.host = (*host).receivedAddress.host;
        (*peer).address.port = (*host).receivedAddress.port;
        (*peer).incomingDataTotal =
            ((*peer).incomingDataTotal as
                 libc::c_ulong).wrapping_add((*host).receivedDataLength) as
                EnetUint32 as EnetUint32
    }
    currentData = (*host).receivedData.offset(headerSize as isize);
    while currentData <
              &mut *(*host).receivedData.offset((*host).receivedDataLength as
                                                    isize) as *mut enet_uint8
          {
        let mut commandNumber: enet_uint8 = 0;
        let mut commandSize: SizeT = 0;
        command = currentData as *mut ENetProtocol;
        if currentData.offset(::std::mem::size_of::<ENetProtocolCommandHeader>()
                                  as libc::c_ulong as isize) >
               &mut *(*host).receivedData.offset((*host).receivedDataLength as
                                                     isize) as *mut enet_uint8
           {
            break ;
        }
        commandNumber =
            ((*command).header.command as libc::c_int &
                 ENET_PROTOCOL_COMMAND_MASK as libc::c_int) as enet_uint8;
        if commandNumber as libc::c_int >=
               ENET_PROTOCOL_COMMAND_COUNT as libc::c_int {
            break ;
        }
        commandSize = commandSizes[commandNumber as usize];
        if commandSize == 0i32 as libc::c_ulong ||
               currentData.offset(commandSize as isize) >
                   &mut *(*host).receivedData.offset((*host).receivedDataLength
                                                         as isize) as
                       *mut enet_uint8 {
            break ;
        }
        currentData = currentData.offset(commandSize as isize);
        if peer.is_null() &&
               commandNumber as libc::c_int !=
                   ENET_PROTOCOL_COMMAND_CONNECT as libc::c_int {
            break ;
        }
        (*command).header.reliableSequenceNumber =
            ntohs((*command).header.reliableSequenceNumber);
        match commandNumber as libc::c_int {
            1 => {
                if 0 !=
                       enet_protocol_handle_acknowledge(host, event, peer,
                                                        command) {
                    break ;
                }
            }
            2 => {
                if !peer.is_null() { break ; }
                peer = enet_protocol_handle_connect(host, header, command);
                if peer.is_null() { break ; }
            }
            3 => {
                if 0 !=
                       enet_protocol_handle_verify_connect(host, event, peer,
                                                           command) {
                    break ;
                }
            }
            4 => {
                if 0 != enet_protocol_handle_disconnect(host, peer, command) {
                    break ;
                }
            }
            5 => {
                if 0 != enet_protocol_handle_ping(host, peer, command) {
                    break ;
                }
            }
            6 => {
                if 0 !=
                       enet_protocol_handle_send_reliable(host, peer, command,
                                                          &mut currentData) {
                    break ;
                }
            }
            7 => {
                if 0 !=
                       enet_protocol_handle_send_unreliable(host, peer,
                                                            command,
                                                            &mut currentData)
                   {
                    break ;
                }
            }
            9 => {
                if 0 !=
                       enet_protocol_handle_send_unsequenced(host, peer,
                                                             command,
                                                             &mut currentData)
                   {
                    break ;
                }
            }
            8 => {
                if 0 !=
                       enet_protocol_handle_send_fragment(host, peer, command,
                                                          &mut currentData) {
                    break ;
                }
            }
            10 => {
                if 0 !=
                       enet_protocol_handle_bandwidth_limit(host, peer,
                                                            command) {
                    break ;
                }
            }
            11 => {
                if 0 !=
                       enet_protocol_handle_throttle_configure(host, peer,
                                                               command) {
                    break ;
                }
            }
            12 => {
                if 0 !=
                       enet_protocol_handle_send_unreliable_fragment(host,
                                                                     peer,
                                                                     command,
                                                                     &mut currentData)
                   {
                    break ;
                }
            }
            _ => { break ; }
        }
        if !(!peer.is_null() &&
                 (*command).header.command as libc::c_int &
                     ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int !=
                     0i32) {
            continue ;
        }
        let mut sentTime: EnetUint16 = 0;
        if 0 ==
               flags as libc::c_int &
                   ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as libc::c_int {
            break ;
        }
        sentTime = ntohs((*header).sentTime);
        match (*peer).state as libc::c_uint {
            7 | 2 | 0 | 9 => { }
            8 => {
                if (*command).header.command as libc::c_int &
                       ENET_PROTOCOL_COMMAND_MASK as libc::c_int ==
                       ENET_PROTOCOL_COMMAND_DISCONNECT as libc::c_int {
                    enet_peer_queue_acknowledgement(peer, command, sentTime);
                }
            }
            _ => { enet_peer_queue_acknowledgement(peer, command, sentTime); }
        }
    }
    if !event.is_null() &&
           (*event).type_0 as libc::c_uint !=
               ENET_EVENT_TYPE_NONE as libc::c_int as libc::c_uint {
        return 1i32
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_receive_incoming_commands(mut host:
                                                                 *mut ENetHost,
                                                             mut event:
                                                                 *mut ENetEvent)
 -> libc::c_int {
    let mut packets: libc::c_int = 0;
    let mut current_block_17: u64;
    packets = 0i32;
    while packets < 256i32 {
        let mut receivedLength: libc::c_int = 0;
        let mut buffer: ENetBuffer =
            ENetBuffer{data: 0 as *mut libc::c_void, data_length: 0,};
        buffer.data =
            (*host).packetData[0usize].as_mut_ptr() as *mut libc::c_void;
        buffer.data_length =
            ::std::mem::size_of::<[enet_uint8; 4096]>() as libc::c_ulong;
        receivedLength =
            enet_socket_receive((*host).socket, &mut (*host).receivedAddress,
                                &mut buffer, 1i32 as SizeT);
        if receivedLength < 0i32 { return -1i32 }
        if receivedLength == 0i32 { return 0i32 }
        (*host).receivedData = (*host).packetData[0usize].as_mut_ptr();
        (*host).receivedDataLength = receivedLength as SizeT;
        (*host).totalReceivedData =
            ((*host).totalReceivedData as
                 libc::c_uint).wrapping_add(receivedLength as libc::c_uint) as
                EnetUint32 as EnetUint32;
        (*host).totalReceivedPackets =
            (*host).totalReceivedPackets.wrapping_add(1);
        if (*host).intercept.is_some() {
            match (*host).intercept.expect("non-null function pointer")(host,
                                                                        event)
                {
                1 => {
                    current_block_17 = 3531489836707249550;
                    match current_block_17 {
                        13667463609069239843 => { return -1i32 }
                        _ => {
                            if !event.is_null() &&
                                   (*event).type_0 as libc::c_uint !=
                                       ENET_EVENT_TYPE_NONE as libc::c_int as
                                           libc::c_uint {
                                return 1i32
                            }
                        }
                    }
                    current_block_17 = 16668937799742929182;
                }
                -1 => {
                    current_block_17 = 13667463609069239843;
                    match current_block_17 {
                        13667463609069239843 => { return -1i32 }
                        _ => {
                            if !event.is_null() &&
                                   (*event).type_0 as libc::c_uint !=
                                       ENET_EVENT_TYPE_NONE as libc::c_int as
                                           libc::c_uint {
                                return 1i32
                            }
                        }
                    }
                    current_block_17 = 16668937799742929182;
                }
                _ => { current_block_17 = 13242334135786603907; }
            }
        } else { current_block_17 = 13242334135786603907; }
        match current_block_17 {
            13242334135786603907 => {
                match enet_protocol_handle_incoming_commands(host, event) {
                    1 => { return 1i32 }
                    -1 => { return -1i32 }
                    _ => { }
                }
            }
            _ => { }
        }
        packets += 1
    }
    return -1i32;
}
unsafe extern "C" fn enet_protocol_send_acknowledgements(mut host:
                                                             *mut ENetHost,
                                                         mut peer:
                                                             *mut ENetPeer) {
    let mut command: *mut ENetProtocol =
        &mut *(*host).commands.as_mut_ptr().offset((*host).commandCount as
                                                       isize) as
            *mut ENetProtocol;
    let mut buffer: *mut ENetBuffer =
        &mut *(*host).buffers.as_mut_ptr().offset((*host).buffer_count as
                                                      isize) as
            *mut ENetBuffer;
    let mut acknowledgement: *mut ENetAcknowledgement =
        0 as *mut ENetAcknowledgement;
    let mut currentAcknowledgement: ENetListIterator = 0 as *mut ENetListNode;
    let mut reliableSequenceNumber: EnetUint16 = 0;
    currentAcknowledgement = (*peer).acknowledgements.sentinel.next;
    while currentAcknowledgement !=
              &mut (*peer).acknowledgements.sentinel as *mut ENetListNode {
        if command >=
               &mut *(*host).commands.as_mut_ptr().offset((::std::mem::size_of::<[ENetProtocol; 32]>()
                                                               as
                                                               libc::c_ulong).wrapping_div(::std::mem::size_of::<ENetProtocol>()
                                                                                               as
                                                                                               libc::c_ulong)
                                                              as isize) as
                   *mut ENetProtocol ||
               buffer >=
                   &mut *(*host).buffers.as_mut_ptr().offset((::std::mem::size_of::<[ENetBuffer; 65]>()
                                                                  as
                                                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<ENetBuffer>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                                 as isize) as
                       *mut ENetBuffer ||
               ((*peer).mtu as libc::c_ulong).wrapping_sub((*host).packetSize)
                   <
                   ::std::mem::size_of::<ENetProtocolAcknowledge>() as
                       libc::c_ulong {
            (*host).continueSending = 1i32;
            break ;
        } else {
            acknowledgement =
                currentAcknowledgement as *mut ENetAcknowledgement;
            currentAcknowledgement = (*currentAcknowledgement).next;
            (*buffer).data = command as *mut libc::c_void;
            (*buffer).data_length =
                ::std::mem::size_of::<ENetProtocolAcknowledge>() as
                    libc::c_ulong;
            (*host).packetSize =
                ((*host).packetSize as
                     libc::c_ulong).wrapping_add((*buffer).data_length) as
                    SizeT as SizeT;
            reliableSequenceNumber =
                htons((*acknowledgement).command.header.reliableSequenceNumber);
            (*command).header.command =
                ENET_PROTOCOL_COMMAND_ACKNOWLEDGE as libc::c_int as
                    enet_uint8;
            (*command).header.channelID =
                (*acknowledgement).command.header.channelID;
            (*command).header.reliableSequenceNumber = reliableSequenceNumber;
            (*command).acknowledge.receivedReliableSequenceNumber =
                reliableSequenceNumber;
            (*command).acknowledge.receivedSentTime =
                htons((*acknowledgement).sentTime as Uint16T);
            if (*acknowledgement).command.header.command as libc::c_int &
                   ENET_PROTOCOL_COMMAND_MASK as libc::c_int ==
                   ENET_PROTOCOL_COMMAND_DISCONNECT as libc::c_int {
                enet_protocol_dispatch_state(host, peer,
                                             ENET_PEER_STATE_ZOMBIE);
            }
            enet_list_remove(&mut (*acknowledgement).acknowledgementList);
            enet_free(acknowledgement as *mut libc::c_void);
            command = command.offset(1isize);
            buffer = buffer.offset(1isize)
        }
    }
    (*host).commandCount =
        command.wrapping_offset_from((*host).commands.as_mut_ptr()) as
            libc::c_long as SizeT;
    (*host).buffer_count =
        buffer.wrapping_offset_from((*host).buffers.as_mut_ptr()) as
            libc::c_long as SizeT;
}
unsafe extern "C" fn enet_protocol_send_unreliable_outgoing_commands(mut host:
                                                                         *mut ENetHost,
                                                                     mut peer:
                                                                         *mut ENetPeer) {
    let mut command: *mut ENetProtocol =
        &mut *(*host).commands.as_mut_ptr().offset((*host).commandCount as
                                                       isize) as
            *mut ENetProtocol;
    let mut buffer: *mut ENetBuffer =
        &mut *(*host).buffers.as_mut_ptr().offset((*host).buffer_count as
                                                      isize) as
            *mut ENetBuffer;
    let mut outgoingCommand: *mut ENetOutgoingCommand =
        0 as *mut ENetOutgoingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    currentCommand = (*peer).outgoingUnreliableCommands.sentinel.next;
    while currentCommand !=
              &mut (*peer).outgoingUnreliableCommands.sentinel as
                  *mut ENetListNode {
        let mut commandSize: SizeT = 0;
        outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
        commandSize =
            commandSizes[((*outgoingCommand).command.header.command as
                              libc::c_int &
                              ENET_PROTOCOL_COMMAND_MASK as libc::c_int) as
                             usize];
        if command >=
               &mut *(*host).commands.as_mut_ptr().offset((::std::mem::size_of::<[ENetProtocol; 32]>()
                                                               as
                                                               libc::c_ulong).wrapping_div(::std::mem::size_of::<ENetProtocol>()
                                                                                               as
                                                                                               libc::c_ulong)
                                                              as isize) as
                   *mut ENetProtocol ||
               buffer.offset(1isize) >=
                   &mut *(*host).buffers.as_mut_ptr().offset((::std::mem::size_of::<[ENetBuffer; 65]>()
                                                                  as
                                                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<ENetBuffer>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                                 as isize) as
                       *mut ENetBuffer ||
               ((*peer).mtu as libc::c_ulong).wrapping_sub((*host).packetSize)
                   < commandSize ||
               !(*outgoingCommand).packet.is_null() &&
                   ((*peer).mtu as
                        libc::c_ulong).wrapping_sub((*host).packetSize) <
                       commandSize.wrapping_add((*outgoingCommand).fragmentLength
                                                    as libc::c_ulong) {
            (*host).continueSending = 1i32;
            break ;
        } else {
            currentCommand = (*currentCommand).next;
            if !(*outgoingCommand).packet.is_null() &&
                   (*outgoingCommand).fragmentOffset == 0i32 as libc::c_uint {
                (*peer).packetThrottleCounter =
                    ((*peer).packetThrottleCounter as
                         libc::c_uint).wrapping_add(ENET_PEER_PACKET_THROTTLE_COUNTER
                                                        as libc::c_int as
                                                        libc::c_uint) as
                        EnetUint32 as EnetUint32;
                (*peer).packetThrottleCounter =
                    ((*peer).packetThrottleCounter as
                         libc::c_uint).wrapping_rem(ENET_PEER_PACKET_THROTTLE_SCALE
                                                        as libc::c_int as
                                                        libc::c_uint) as
                        EnetUint32 as EnetUint32;
                if (*peer).packetThrottleCounter > (*peer).packetThrottle {
                    let mut reliableSequenceNumber: EnetUint16 =
                        (*outgoingCommand).reliableSequenceNumber;
                    let mut unreliableSequenceNumber: EnetUint16 =
                        (*outgoingCommand).unreliableSequenceNumber;
                    loop  {
                        (*(*outgoingCommand).packet).referenceCount =
                            (*(*outgoingCommand).packet).referenceCount.wrapping_sub(1);
                        if (*(*outgoingCommand).packet).referenceCount ==
                               0i32 as libc::c_ulong {
                            enet_packet_destroy((*outgoingCommand).packet);
                        }
                        enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
                        enet_free(outgoingCommand as *mut libc::c_void);
                        if currentCommand ==
                               &mut (*peer).outgoingUnreliableCommands.sentinel
                                   as *mut ENetListNode {
                            break ;
                        }
                        outgoingCommand =
                            currentCommand as *mut ENetOutgoingCommand;
                        if (*outgoingCommand).reliableSequenceNumber as
                               libc::c_int !=
                               reliableSequenceNumber as libc::c_int ||
                               (*outgoingCommand).unreliableSequenceNumber as
                                   libc::c_int !=
                                   unreliableSequenceNumber as libc::c_int {
                            break ;
                        }
                        currentCommand = (*currentCommand).next
                    }
                    continue ;
                }
            }
            (*buffer).data = command as *mut libc::c_void;
            (*buffer).data_length = commandSize;
            (*host).packetSize =
                ((*host).packetSize as
                     libc::c_ulong).wrapping_add((*buffer).data_length) as
                    SizeT as SizeT;
            *command = (*outgoingCommand).command;
            enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
            if !(*outgoingCommand).packet.is_null() {
                buffer = buffer.offset(1isize);
                (*buffer).data =
                    (*(*outgoingCommand).packet).data.offset((*outgoingCommand).fragmentOffset
                                                                 as isize) as
                        *mut libc::c_void;
                (*buffer).data_length =
                    (*outgoingCommand).fragmentLength as SizeT;
                (*host).packetSize =
                    ((*host).packetSize as
                         libc::c_ulong).wrapping_add((*buffer).data_length) as
                        SizeT as SizeT;
                enet_list_insert(&mut (*peer).sentUnreliableCommands.sentinel,
                                 outgoingCommand as *mut libc::c_void);
            } else { enet_free(outgoingCommand as *mut libc::c_void); }
            command = command.offset(1isize);
            buffer = buffer.offset(1isize)
        }
    }
    (*host).commandCount =
        command.wrapping_offset_from((*host).commands.as_mut_ptr()) as
            libc::c_long as SizeT;
    (*host).buffer_count =
        buffer.wrapping_offset_from((*host).buffers.as_mut_ptr()) as
            libc::c_long as SizeT;
    if (*peer).state as libc::c_uint ==
           ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint &&
           (*peer).outgoingReliableCommands.sentinel.next ==
               &mut (*peer).outgoingReliableCommands.sentinel as
                   *mut ENetListNode &&
           (*peer).outgoingUnreliableCommands.sentinel.next ==
               &mut (*peer).outgoingUnreliableCommands.sentinel as
                   *mut ENetListNode &&
           (*peer).sentReliableCommands.sentinel.next ==
               &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode
           &&
           (*peer).sentUnreliableCommands.sentinel.next ==
               &mut (*peer).sentUnreliableCommands.sentinel as
                   *mut ENetListNode {
        enet_peer_disconnect(peer, (*peer).eventData);
    };
}
unsafe extern "C" fn enet_protocol_check_timeouts(mut host: *mut ENetHost,
                                                  mut peer: *mut ENetPeer,
                                                  mut event: *mut ENetEvent)
 -> libc::c_int {
    let mut outgoingCommand: *mut ENetOutgoingCommand =
        0 as *mut ENetOutgoingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut insertPosition: ENetListIterator = 0 as *mut ENetListNode;
    currentCommand = (*peer).sentReliableCommands.sentinel.next;
    insertPosition = (*peer).outgoingReliableCommands.sentinel.next;
    while currentCommand !=
              &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode
          {
        outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
        currentCommand = (*currentCommand).next;
        if (if (*host).serviceTime.wrapping_sub((*outgoingCommand).sentTime)
                   >= 86400000i32 as libc::c_uint {
                (*outgoingCommand).sentTime.wrapping_sub((*host).serviceTime)
            } else {
                (*host).serviceTime.wrapping_sub((*outgoingCommand).sentTime)
            }) < (*outgoingCommand).roundTripTimeout {
            continue ;
        }
        if (*peer).earliestTimeout == 0i32 as libc::c_uint ||
               (*outgoingCommand).sentTime.wrapping_sub((*peer).earliestTimeout)
                   >= 86400000i32 as libc::c_uint {
            (*peer).earliestTimeout = (*outgoingCommand).sentTime
        }
        if (*peer).earliestTimeout != 0i32 as libc::c_uint &&
               ((if (*host).serviceTime.wrapping_sub((*peer).earliestTimeout)
                        >= 86400000i32 as libc::c_uint {
                     (*peer).earliestTimeout.wrapping_sub((*host).serviceTime)
                 } else {
                     (*host).serviceTime.wrapping_sub((*peer).earliestTimeout)
                 }) >= (*peer).timeoutMaximum ||
                    (*outgoingCommand).roundTripTimeout >=
                        (*outgoingCommand).roundTripTimeoutLimit &&
                        (if (*host).serviceTime.wrapping_sub((*peer).earliestTimeout)
                                >= 86400000i32 as libc::c_uint {
                             (*peer).earliestTimeout.wrapping_sub((*host).serviceTime)
                         } else {
                             (*host).serviceTime.wrapping_sub((*peer).earliestTimeout)
                         }) >= (*peer).timeoutMinimum) {
            enet_protocol_notify_disconnect(host, peer, event);
            return 1i32
        }
        if !(*outgoingCommand).packet.is_null() {
            (*peer).reliableDataInTransit =
                ((*peer).reliableDataInTransit as
                     libc::c_uint).wrapping_sub((*outgoingCommand).fragmentLength
                                                    as libc::c_uint) as
                    EnetUint32 as EnetUint32
        }
        (*peer).packetsLost = (*peer).packetsLost.wrapping_add(1);
        (*outgoingCommand).roundTripTimeout =
            ((*outgoingCommand).roundTripTimeout as
                 libc::c_uint).wrapping_mul(2i32 as libc::c_uint) as
                EnetUint32 as EnetUint32;
        enet_list_insert(insertPosition,
                         enet_list_remove(&mut (*outgoingCommand).outgoingCommandList));
        if currentCommand == (*peer).sentReliableCommands.sentinel.next &&
               !((*peer).sentReliableCommands.sentinel.next ==
                     &mut (*peer).sentReliableCommands.sentinel as
                         *mut ENetListNode) {
            outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
            (*peer).nextTimeout =
                (*outgoingCommand).sentTime.wrapping_add((*outgoingCommand).roundTripTimeout)
        }
    }
    return 0i32;
}
unsafe extern "C" fn enet_protocol_send_reliable_outgoing_commands(mut host:
                                                                       *mut ENetHost,
                                                                   mut peer:
                                                                       *mut ENetPeer)
 -> libc::c_int {
    let mut command: *mut ENetProtocol =
        &mut *(*host).commands.as_mut_ptr().offset((*host).commandCount as
                                                       isize) as
            *mut ENetProtocol;
    let mut buffer: *mut ENetBuffer =
        &mut *(*host).buffers.as_mut_ptr().offset((*host).buffer_count as
                                                      isize) as
            *mut ENetBuffer;
    let mut outgoingCommand: *mut ENetOutgoingCommand =
        0 as *mut ENetOutgoingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut reliableWindow: EnetUint16 = 0;
    let mut commandSize: SizeT = 0;
    let mut windowExceeded: libc::c_int = 0i32;
    let mut windowWrap: libc::c_int = 0i32;
    let mut canPing: libc::c_int = 1i32;
    currentCommand = (*peer).outgoingReliableCommands.sentinel.next;
    while currentCommand !=
              &mut (*peer).outgoingReliableCommands.sentinel as
                  *mut ENetListNode {
        outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
        channel =
            if ((*outgoingCommand).command.header.channelID as libc::c_ulong)
                   < (*peer).channelCount {
                &mut *(*peer).channels.offset((*outgoingCommand).command.header.channelID
                                                  as isize) as
                    *mut ENetChannel
            } else { 0 as *mut ENetChannel };
        reliableWindow =
            ((*outgoingCommand).reliableSequenceNumber as libc::c_int /
                 ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as
                EnetUint16;
        if !channel.is_null() {
            if 0 == windowWrap &&
                   ((*outgoingCommand).sendAttempts as libc::c_int) < 1i32 &&
                   0 ==
                       (*outgoingCommand).reliableSequenceNumber as
                           libc::c_int %
                           ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int &&
                   ((*channel).reliableWindows[((reliableWindow as libc::c_int
                                                     +
                                                     ENET_PEER_RELIABLE_WINDOWS
                                                         as libc::c_int -
                                                     1i32) %
                                                    ENET_PEER_RELIABLE_WINDOWS
                                                        as libc::c_int) as
                                                   usize] as libc::c_int >=
                        ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int ||
                        0 !=
                            (*channel).usedReliableWindows as libc::c_int &
                                ((1i32 <<
                                      ENET_PEER_FREE_RELIABLE_WINDOWS as
                                          libc::c_int) - 1i32 <<
                                     reliableWindow as libc::c_int |
                                     (1i32 <<
                                          ENET_PEER_FREE_RELIABLE_WINDOWS as
                                              libc::c_int) - 1i32 >>
                                         ENET_PEER_RELIABLE_WINDOWS as
                                             libc::c_int -
                                             reliableWindow as libc::c_int)) {
                windowWrap = 1i32
            }
            if 0 != windowWrap {
                currentCommand = (*currentCommand).next;
                continue ;
            }
        }
        if !(*outgoingCommand).packet.is_null() {
            if 0 == windowExceeded {
                let mut windowSize: EnetUint32 =
                    (*peer).packetThrottle.wrapping_mul((*peer).windowSize).wrapping_div(ENET_PEER_PACKET_THROTTLE_SCALE
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint);
                if (*peer).reliableDataInTransit.wrapping_add((*outgoingCommand).fragmentLength
                                                                  as
                                                                  libc::c_uint)
                       >
                       (if windowSize > (*peer).mtu {
                            windowSize
                        } else { (*peer).mtu }) {
                    windowExceeded = 1i32
                }
            }
            if 0 != windowExceeded {
                currentCommand = (*currentCommand).next;
                continue ;
            }
        }
        canPing = 0i32;
        commandSize =
            commandSizes[((*outgoingCommand).command.header.command as
                              libc::c_int &
                              ENET_PROTOCOL_COMMAND_MASK as libc::c_int) as
                             usize];
        if command >=
               &mut *(*host).commands.as_mut_ptr().offset((::std::mem::size_of::<[ENetProtocol; 32]>()
                                                               as
                                                               libc::c_ulong).wrapping_div(::std::mem::size_of::<ENetProtocol>()
                                                                                               as
                                                                                               libc::c_ulong)
                                                              as isize) as
                   *mut ENetProtocol ||
               buffer.offset(1isize) >=
                   &mut *(*host).buffers.as_mut_ptr().offset((::std::mem::size_of::<[ENetBuffer; 65]>()
                                                                  as
                                                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<ENetBuffer>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                                 as isize) as
                       *mut ENetBuffer ||
               ((*peer).mtu as libc::c_ulong).wrapping_sub((*host).packetSize)
                   < commandSize ||
               !(*outgoingCommand).packet.is_null() &&
                   (((*peer).mtu as
                         libc::c_ulong).wrapping_sub((*host).packetSize) as
                        EnetUint16 as libc::c_int) <
                       commandSize.wrapping_add((*outgoingCommand).fragmentLength
                                                    as libc::c_ulong) as
                           EnetUint16 as libc::c_int {
            (*host).continueSending = 1i32;
            break ;
        } else {
            currentCommand = (*currentCommand).next;
            if !channel.is_null() &&
                   ((*outgoingCommand).sendAttempts as libc::c_int) < 1i32 {
                (*channel).usedReliableWindows =
                    ((*channel).usedReliableWindows as libc::c_int |
                         1i32 << reliableWindow as libc::c_int) as
                        EnetUint16;
                (*channel).reliableWindows[reliableWindow as usize] =
                    (*channel).reliableWindows[reliableWindow as
                                                   usize].wrapping_add(1)
            }
            (*outgoingCommand).sendAttempts =
                (*outgoingCommand).sendAttempts.wrapping_add(1);
            if (*outgoingCommand).roundTripTimeout == 0i32 as libc::c_uint {
                (*outgoingCommand).roundTripTimeout =
                    (*peer).roundTripTime.wrapping_add((4i32 as
                                                            libc::c_uint).wrapping_mul((*peer).roundTripTimeVariance));
                (*outgoingCommand).roundTripTimeoutLimit =
                    (*peer).timeoutLimit.wrapping_mul((*outgoingCommand).roundTripTimeout)
            }
            if (*peer).sentReliableCommands.sentinel.next ==
                   &mut (*peer).sentReliableCommands.sentinel as
                       *mut ENetListNode {
                (*peer).nextTimeout =
                    (*host).serviceTime.wrapping_add((*outgoingCommand).roundTripTimeout)
            }
            enet_list_insert(&mut (*peer).sentReliableCommands.sentinel,
                             enet_list_remove(&mut (*outgoingCommand).outgoingCommandList));
            (*outgoingCommand).sentTime = (*host).serviceTime;
            (*buffer).data = command as *mut libc::c_void;
            (*buffer).data_length = commandSize;
            (*host).packetSize =
                ((*host).packetSize as
                     libc::c_ulong).wrapping_add((*buffer).data_length) as
                    SizeT as SizeT;
            (*host).headerFlags =
                ((*host).headerFlags as libc::c_int |
                     ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as libc::c_int) as
                    EnetUint16;
            *command = (*outgoingCommand).command;
            if !(*outgoingCommand).packet.is_null() {
                buffer = buffer.offset(1isize);
                (*buffer).data =
                    (*(*outgoingCommand).packet).data.offset((*outgoingCommand).fragmentOffset
                                                                 as isize) as
                        *mut libc::c_void;
                (*buffer).data_length =
                    (*outgoingCommand).fragmentLength as SizeT;
                (*host).packetSize =
                    ((*host).packetSize as
                         libc::c_ulong).wrapping_add((*outgoingCommand).fragmentLength
                                                         as libc::c_ulong) as
                        SizeT as SizeT;
                (*peer).reliableDataInTransit =
                    ((*peer).reliableDataInTransit as
                         libc::c_uint).wrapping_add((*outgoingCommand).fragmentLength
                                                        as libc::c_uint) as
                        EnetUint32 as EnetUint32
            }
            (*peer).packetsSent = (*peer).packetsSent.wrapping_add(1);
            command = command.offset(1isize);
            buffer = buffer.offset(1isize)
        }
    }
    (*host).commandCount =
        command.wrapping_offset_from((*host).commands.as_mut_ptr()) as
            libc::c_long as SizeT;
    (*host).buffer_count =
        buffer.wrapping_offset_from((*host).buffers.as_mut_ptr()) as
            libc::c_long as SizeT;
    return canPing;
}
unsafe extern "C" fn enet_protocol_send_outgoing_commands(mut host:
                                                              *mut ENetHost,
                                                          mut event:
                                                              *mut ENetEvent,
                                                          mut checkForTimeouts:
                                                              libc::c_int)
 -> libc::c_int {
    let mut headerData: [enet_uint8; 8] = [0; 8];
    let mut header: *mut ENetProtocolHeader =
        headerData.as_mut_ptr() as *mut ENetProtocolHeader;
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut sent_length: libc::c_int = 0;
    let mut shouldCompress: SizeT = 0i32 as SizeT;
    (*host).continueSending = 1i32;
    while 0 != (*host).continueSending {
        (*host).continueSending = 0i32;
        currentPeer = (*host).peers;
        while currentPeer <
                  &mut *(*host).peers.offset((*host).peerCount as isize) as
                      *mut ENetPeer {
            if !((*currentPeer).state as libc::c_uint ==
                     ENET_PEER_STATE_DISCONNECTED as libc::c_int as
                         libc::c_uint ||
                     (*currentPeer).state as libc::c_uint ==
                         ENET_PEER_STATE_ZOMBIE as libc::c_int as
                             libc::c_uint) {
                (*host).headerFlags = 0i32 as EnetUint16;
                (*host).commandCount = 0i32 as SizeT;
                (*host).buffer_count = 1i32 as SizeT;
                (*host).packetSize =
                    ::std::mem::size_of::<ENetProtocolHeader>() as
                        libc::c_ulong;
                if !((*currentPeer).acknowledgements.sentinel.next ==
                         &mut (*currentPeer).acknowledgements.sentinel as
                             *mut ENetListNode) {
                    enet_protocol_send_acknowledgements(host, currentPeer);
                }
                if checkForTimeouts != 0i32 &&
                       !((*currentPeer).sentReliableCommands.sentinel.next ==
                             &mut (*currentPeer).sentReliableCommands.sentinel
                                 as *mut ENetListNode) &&
                       !((*host).serviceTime.wrapping_sub((*currentPeer).nextTimeout)
                             >= 86400000i32 as libc::c_uint) &&
                       enet_protocol_check_timeouts(host, currentPeer, event)
                           == 1i32 {
                    if !event.is_null() &&
                           (*event).type_0 as libc::c_uint !=
                               ENET_EVENT_TYPE_NONE as libc::c_int as
                                   libc::c_uint {
                        return 1i32
                    }
                } else {
                    if ((*currentPeer).outgoingReliableCommands.sentinel.next
                            ==
                            &mut (*currentPeer).outgoingReliableCommands.sentinel
                                as *mut ENetListNode ||
                            0 !=
                                enet_protocol_send_reliable_outgoing_commands(host,
                                                                              currentPeer))
                           &&
                           (*currentPeer).sentReliableCommands.sentinel.next
                               ==
                               &mut (*currentPeer).sentReliableCommands.sentinel
                                   as *mut ENetListNode &&
                           (if (*host).serviceTime.wrapping_sub((*currentPeer).lastReceiveTime)
                                   >= 86400000i32 as libc::c_uint {
                                (*currentPeer).lastReceiveTime.wrapping_sub((*host).serviceTime)
                            } else {
                                (*host).serviceTime.wrapping_sub((*currentPeer).lastReceiveTime)
                            }) >= (*currentPeer).pingInterval &&
                           ((*currentPeer).mtu as
                                libc::c_ulong).wrapping_sub((*host).packetSize)
                               >=
                               ::std::mem::size_of::<ENetProtocolPing>() as
                                   libc::c_ulong {
                        enet_peer_ping(currentPeer);
                        enet_protocol_send_reliable_outgoing_commands(host,
                                                                      currentPeer);
                    }
                    if !((*currentPeer).outgoingUnreliableCommands.sentinel.next
                             ==
                             &mut (*currentPeer).outgoingUnreliableCommands.sentinel
                                 as *mut ENetListNode) {
                        enet_protocol_send_unreliable_outgoing_commands(host,
                                                                        currentPeer);
                    }
                    if !((*host).commandCount == 0i32 as libc::c_ulong) {
                        if (*currentPeer).packetLossEpoch ==
                               0i32 as libc::c_uint {
                            (*currentPeer).packetLossEpoch =
                                (*host).serviceTime
                        } else if (if (*host).serviceTime.wrapping_sub((*currentPeer).packetLossEpoch)
                                          >= 86400000i32 as libc::c_uint {
                                       (*currentPeer).packetLossEpoch.wrapping_sub((*host).serviceTime)
                                   } else {
                                       (*host).serviceTime.wrapping_sub((*currentPeer).packetLossEpoch)
                                   }) >=
                                      ENET_PEER_PACKET_LOSS_INTERVAL as
                                          libc::c_int as libc::c_uint &&
                                      (*currentPeer).packetsSent >
                                          0i32 as libc::c_uint {
                            let mut packetLoss: EnetUint32 =
                                (*currentPeer).packetsLost.wrapping_mul(ENET_PEER_PACKET_LOSS_SCALE
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_div((*currentPeer).packetsSent);
                            (*currentPeer).packetLossVariance =
                                ((*currentPeer).packetLossVariance as
                                     libc::c_uint).wrapping_sub((*currentPeer).packetLossVariance.wrapping_div(4i32
                                                                                                                   as
                                                                                                                   libc::c_uint))
                                    as EnetUint32 as EnetUint32;
                            if packetLoss >= (*currentPeer).packetLoss {
                                (*currentPeer).packetLoss =
                                    ((*currentPeer).packetLoss as
                                         libc::c_uint).wrapping_add(packetLoss.wrapping_sub((*currentPeer).packetLoss).wrapping_div(8i32
                                                                                                                                        as
                                                                                                                                        libc::c_uint))
                                        as EnetUint32 as EnetUint32;
                                (*currentPeer).packetLossVariance =
                                    ((*currentPeer).packetLossVariance as
                                         libc::c_uint).wrapping_add(packetLoss.wrapping_sub((*currentPeer).packetLoss).wrapping_div(4i32
                                                                                                                                        as
                                                                                                                                        libc::c_uint))
                                        as EnetUint32 as EnetUint32
                            } else {
                                (*currentPeer).packetLoss =
                                    ((*currentPeer).packetLoss as
                                         libc::c_uint).wrapping_sub((*currentPeer).packetLoss.wrapping_sub(packetLoss).wrapping_div(8i32
                                                                                                                                        as
                                                                                                                                        libc::c_uint))
                                        as EnetUint32 as EnetUint32;
                                (*currentPeer).packetLossVariance =
                                    ((*currentPeer).packetLossVariance as
                                         libc::c_uint).wrapping_add((*currentPeer).packetLoss.wrapping_sub(packetLoss).wrapping_div(4i32
                                                                                                                                        as
                                                                                                                                        libc::c_uint))
                                        as EnetUint32 as EnetUint32
                            }
                            (*currentPeer).packetLossEpoch =
                                (*host).serviceTime;
                            (*currentPeer).packetsSent = 0i32 as EnetUint32;
                            (*currentPeer).packetsLost = 0i32 as EnetUint32
                        }
                        let ref mut fresh2 =
                            (*(*host).buffers.as_mut_ptr()).data;
                        *fresh2 =
                            headerData.as_mut_ptr() as *mut libc::c_void;
                        if 0 !=
                               (*host).headerFlags as libc::c_int &
                                   ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as
                                       libc::c_int {
                            (*header).sentTime =
                                htons(((*host).serviceTime &
                                           0xffffi32 as libc::c_uint) as
                                          Uint16T);
                            (*(*host).buffers.as_mut_ptr()).data_length =
                                ::std::mem::size_of::<ENetProtocolHeader>() as
                                    libc::c_ulong
                        } else {
                            (*(*host).buffers.as_mut_ptr()).data_length =
                                &mut (*(0 as
                                            *mut ENetProtocolHeader)).sentTime
                                    as *mut EnetUint16 as SizeT
                        }
                        shouldCompress = 0i32 as SizeT;
                        if !(*host).compressor.context.is_null() &&
                               (*host).compressor.compress.is_some() {
                            let mut originalSize: SizeT =
                                (*host).packetSize.wrapping_sub(::std::mem::size_of::<ENetProtocolHeader>()
                                                                    as
                                                                    libc::c_ulong);
                            let mut compressedSize: SizeT =
                                (*host).compressor.compress.expect("non-null function pointer")((*host).compressor.context,
                                                                                                &mut *(*host).buffers.as_mut_ptr().offset(1isize),
                                                                                                (*host).buffer_count.wrapping_sub(1i32
                                                                                                                                     as
                                                                                                                                     libc::c_ulong),
                                                                                                originalSize,
                                                                                                (*host).packetData[1usize].as_mut_ptr(),
                                                                                                originalSize);
                            if compressedSize > 0i32 as libc::c_ulong &&
                                   compressedSize < originalSize {
                                (*host).headerFlags =
                                    ((*host).headerFlags as libc::c_int |
                                         ENET_PROTOCOL_HEADER_FLAG_COMPRESSED
                                             as libc::c_int) as EnetUint16;
                                shouldCompress = compressedSize
                            }
                        }
                        if ((*currentPeer).outgoingPeerID as libc::c_int) <
                               ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int {
                            (*host).headerFlags =
                                ((*host).headerFlags as libc::c_int |
                                     ((*currentPeer).outgoingSessionID as
                                          libc::c_int) <<
                                         ENET_PROTOCOL_HEADER_SESSION_SHIFT as
                                             libc::c_int) as EnetUint16
                        }
                        (*header).peerID =
                            htons(((*currentPeer).outgoingPeerID as
                                       libc::c_int |
                                       (*host).headerFlags as libc::c_int) as
                                      Uint16T);
                        if (*host).checksum.is_some() {
                            let mut checksum: *mut EnetUint32 =
                                &mut *headerData.as_mut_ptr().offset((*(*host).buffers.as_mut_ptr()).data_length
                                                                         as
                                                                         isize)
                                    as *mut enet_uint8 as *mut EnetUint32;
                            *checksum =
                                if ((*currentPeer).outgoingPeerID as
                                        libc::c_int) <
                                       ENET_PROTOCOL_MAXIMUM_PEER_ID as
                                           libc::c_int {
                                    (*currentPeer).connectID
                                } else { 0i32 as libc::c_uint };
                            let ref mut fresh3 =
                                (*(*host).buffers.as_mut_ptr()).data_length;
                            *fresh3 =
                                (*fresh3 as
                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<EnetUint32>()
                                                                     as
                                                                     libc::c_ulong)
                                    as SizeT as SizeT;
                            *checksum =
                                (*host).checksum.expect("non-null function pointer")((*host).buffers.as_mut_ptr(),
                                                                                     (*host).buffer_count)
                        }
                        if shouldCompress > 0i32 as libc::c_ulong {
                            (*host).buffers[1usize].data =
                                (*host).packetData[1usize].as_mut_ptr() as
                                    *mut libc::c_void;
                            (*host).buffers[1usize].data_length =
                                shouldCompress;
                            (*host).buffer_count = 2i32 as SizeT
                        }
                        (*currentPeer).lastSendTime = (*host).serviceTime;
                        sent_length =
                            enet_socket_send((*host).socket,
                                             &mut (*currentPeer).address,
                                             (*host).buffers.as_mut_ptr(),
                                             (*host).buffer_count);
                        enet_protocol_remove_sent_unreliable_commands(currentPeer);
                        if sent_length < 0i32 { return -1i32 }
                        (*host).totalSentData =
                            ((*host).totalSentData as
                                 libc::c_uint).wrapping_add(sent_length as
                                                                libc::c_uint)
                                as EnetUint32 as EnetUint32;
                        (*host).totalSentPackets =
                            (*host).totalSentPackets.wrapping_add(1)
                    }
                }
            }
            currentPeer = currentPeer.offset(1isize)
        }
    }
    return 0i32;
}
/* * Sends any queued packets on the host specified to its designated peers.

    @param host   host to flush
    @remarks this function need only be used in circumstances where one wishes to send queued packets earlier than in a call to enet_host_service().
    @ingroup host
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_flush(mut host: *mut ENetHost) {
    (*host).serviceTime = enet_time_get();
    enet_protocol_send_outgoing_commands(host, 0 as *mut ENetEvent, 0i32);
}
/* * Checks for any queued events on the host and dispatches one if available.

    @param host    host to check for events
    @param event   an event structure where event details will be placed if available
    @retval > 0 if an event was dispatched
    @retval 0 if no events are available
    @retval < 0 on failure
    @ingroup host
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_check_events(mut host: *mut ENetHost,
                                                mut event: *mut ENetEvent)
 -> libc::c_int {
    if event.is_null() { return -1i32 }
    (*event).type_0 = ENET_EVENT_TYPE_NONE;
    (*event).peer = 0 as *mut ENetPeer;
    (*event).packet = 0 as *mut ENetPacket;
    return enet_protocol_dispatch_incoming_commands(host, event);
}
/* * Waits for events on the host specified and shuttles packets between
    the host and its peers.

    @param host    host to service
    @param event   an event structure where event details will be placed if one occurs
                   if event == NULL then no events will be delivered
    @param timeout number of milliseconds that ENet should wait for events
    @retval > 0 if an event occurred within the specified time limit
    @retval 0 if no event occurred
    @retval < 0 on failure
    @remarks enet_host_service should be called fairly regularly for adequate performance
    @ingroup host
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_service(mut host: *mut ENetHost,
                                           mut event: *mut ENetEvent,
                                           mut timeout: EnetUint32)
 -> libc::c_int {
    let mut waitCondition: EnetUint32 = 0;
    if !event.is_null() {
        (*event).type_0 = ENET_EVENT_TYPE_NONE;
        (*event).peer = 0 as *mut ENetPeer;
        (*event).packet = 0 as *mut ENetPacket;
        match enet_protocol_dispatch_incoming_commands(host, event) {
            1 => { return 1i32 }
            -1 => { return -1i32 }
            _ => { }
        }
    }
    (*host).serviceTime = enet_time_get();
    timeout =
        (timeout as libc::c_uint).wrapping_add((*host).serviceTime) as
            EnetUint32 as EnetUint32;
    loop  {
        if (if (*host).serviceTime.wrapping_sub((*host).bandwidthThrottleEpoch)
                   >= 86400000i32 as libc::c_uint {
                (*host).bandwidthThrottleEpoch.wrapping_sub((*host).serviceTime)
            } else {
                (*host).serviceTime.wrapping_sub((*host).bandwidthThrottleEpoch)
            }) >=
               ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL as libc::c_int as
                   libc::c_uint {
            enet_host_bandwidth_throttle(host);
        }
        match enet_protocol_send_outgoing_commands(host, event, 1i32) {
            1 => { return 1i32 }
            -1 => { return -1i32 }
            _ => { }
        }
        match enet_protocol_receive_incoming_commands(host, event) {
            1 => { return 1i32 }
            -1 => { return -1i32 }
            _ => { }
        }
        match enet_protocol_send_outgoing_commands(host, event, 1i32) {
            1 => { return 1i32 }
            -1 => { return -1i32 }
            _ => { }
        }
        if !event.is_null() {
            match enet_protocol_dispatch_incoming_commands(host, event) {
                1 => { return 1i32 }
                -1 => { return -1i32 }
                _ => { }
            }
        }
        if !((*host).serviceTime.wrapping_sub(timeout) >=
                 86400000i32 as libc::c_uint) {
            return 0i32
        }
        loop  {
            (*host).serviceTime = enet_time_get();
            if !((*host).serviceTime.wrapping_sub(timeout) >=
                     86400000i32 as libc::c_uint) {
                return 0i32
            }
            waitCondition =
                (ENET_SOCKET_WAIT_RECEIVE as libc::c_int |
                     ENET_SOCKET_WAIT_INTERRUPT as libc::c_int) as
                    EnetUint32;
            if enet_socket_wait((*host).socket, &mut waitCondition,
                                (if timeout.wrapping_sub((*host).serviceTime)
                                        >= 86400000i32 as libc::c_uint {
                                     (*host).serviceTime.wrapping_sub(timeout)
                                 } else {
                                     timeout.wrapping_sub((*host).serviceTime)
                                 })) != 0i32 {
                return -1i32
            }
            if !(0 !=
                     waitCondition &
                         ENET_SOCKET_WAIT_INTERRUPT as libc::c_int as
                             libc::c_uint) {
                break ;
            }
        }
        (*host).serviceTime = enet_time_get();
        if !(0 !=
                 waitCondition &
                     ENET_SOCKET_WAIT_RECEIVE as libc::c_int as libc::c_uint)
           {
            break ;
        }
    }
    return 0i32;
}