use libc;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* * @defgroup callbacks ENet internal callbacks
    @{
    @ingroup private
*/
    #[no_mangle]
    fn enet_malloc(_: SizeT) -> *mut libc::c_void;
    #[no_mangle]
    fn enet_free(_: *mut libc::c_void);
    /* * @} */
    /* * @defgroup private ENet private implementation functions */
    /* *
  Returns the wall-time in milliseconds.  Its initial value is unspecified
  unless otherwise set.
  */
    #[no_mangle]
    fn enet_time_get() -> EnetUint32;
    /* * @defgroup socket ENet socket functions
    @{
*/
    #[no_mangle]
    fn enet_socket_create(_: ENetSocketType) -> ENetSocket;
    #[no_mangle]
    fn enet_socket_bind(_: ENetSocket, _: *const ENetAddress) -> libc::c_int;
    #[no_mangle]
    fn enet_socket_get_address(_: ENetSocket, _: *mut ENetAddress)
     -> libc::c_int;
    #[no_mangle]
    fn enet_socket_set_option(_: ENetSocket, _: ENetSocketOption,
                              _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn enet_socket_destroy(_: ENetSocket);
    #[no_mangle]
    fn enet_packet_destroy(_: *mut ENetPacket);
    #[no_mangle]
    fn enet_host_random_seed() -> EnetUint32;
    #[no_mangle]
    fn enet_peer_send(_: *mut ENetPeer, _: enet_uint8, _: *mut ENetPacket)
     -> libc::c_int;
    #[no_mangle]
    fn enet_peer_reset(_: *mut ENetPeer);
    #[no_mangle]
    fn enet_peer_queue_outgoing_command(_: *mut ENetPeer,
                                        _: *const ENetProtocol,
                                        _: *mut ENetPacket, _: EnetUint32,
                                        _: EnetUint16)
     -> *mut ENetOutgoingCommand;
    #[no_mangle]
    fn enet_list_clear(_: *mut ENetList);
    #[no_mangle]
    fn htonl(__hostlong: Uint32T) -> Uint32T;
    #[no_mangle]
    fn htons(__hostshort: Uint16T) -> Uint16T;
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
pub type _ENetSocketType = libc::c_uint;
pub const ENET_SOCKET_TYPE_DATAGRAM: _ENetSocketType = 2;
pub const ENET_SOCKET_TYPE_STREAM: _ENetSocketType = 1;
pub type ENetSocketType = _ENetSocketType;
pub type _ENetSocketOption = libc::c_uint;
pub const ENET_SOCKOPT_NODELAY: _ENetSocketOption = 9;
pub const ENET_SOCKOPT_ERROR: _ENetSocketOption = 8;
pub const ENET_SOCKOPT_SNDTIMEO: _ENetSocketOption = 7;
pub const ENET_SOCKOPT_RCVTIMEO: _ENetSocketOption = 6;
pub const ENET_SOCKOPT_REUSEADDR: _ENetSocketOption = 5;
pub const ENET_SOCKOPT_SNDBUF: _ENetSocketOption = 4;
pub const ENET_SOCKOPT_RCVBUF: _ENetSocketOption = 3;
pub const ENET_SOCKOPT_BROADCAST: _ENetSocketOption = 2;
pub const ENET_SOCKOPT_NONBLOCK: _ENetSocketOption = 1;
pub type ENetSocketOption = _ENetSocketOption;
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
 @file host.c
 @brief ENet host management functions
*/
/* * @defgroup host ENet host functions
    @{
*/
/* * Creates a host for communicating to peers.  

    @param address   the address at which other peers may connect to this host.  If NULL, then no peers may connect to the host.
    @param peerCount the maximum number of peers that should be allocated for the host.
    @param channelLimit the maximum number of channels allowed; if 0, then this is equivalent to ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT
    @param incomingBandwidth downstream bandwidth of the host in bytes/second; if 0, ENet will assume unlimited bandwidth.
    @param outgoingBandwidth upstream bandwidth of the host in bytes/second; if 0, ENet will assume unlimited bandwidth.

    @returns the host on success and NULL on failure

    @remarks ENet will strategically drop packets on specific sides of a connection between hosts
    to ensure the host's bandwidth is not overwhelmed.  The bandwidth parameters also determine
    the window size of a connection which limits the amount of reliable packets that may be in transit
    at any given time.
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_create(mut address: *const ENetAddress,
                                          mut peerCount: SizeT,
                                          mut channelLimit: SizeT,
                                          mut incomingBandwidth: EnetUint32,
                                          mut outgoingBandwidth: EnetUint32)
 -> *mut ENetHost {
    let mut host: *mut ENetHost = 0 as *mut ENetHost;
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    if peerCount >
           ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int as libc::c_ulong {
        return 0 as *mut ENetHost
    }
    host =
        enet_malloc(::std::mem::size_of::<ENetHost>() as libc::c_ulong) as
            *mut ENetHost;
    if host.is_null() { return 0 as *mut ENetHost }
    memset(host as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ENetHost>() as libc::c_ulong);
    (*host).peers =
        enet_malloc(peerCount.wrapping_mul(::std::mem::size_of::<ENetPeer>()
                                               as libc::c_ulong)) as
            *mut ENetPeer;
    if (*host).peers.is_null() {
        enet_free(host as *mut libc::c_void);
        return 0 as *mut ENetHost
    }
    memset((*host).peers as *mut libc::c_void, 0i32,
           peerCount.wrapping_mul(::std::mem::size_of::<ENetPeer>() as
                                      libc::c_ulong));
    (*host).socket = enet_socket_create(ENET_SOCKET_TYPE_DATAGRAM);
    if (*host).socket == -1i32 ||
           !address.is_null() &&
               enet_socket_bind((*host).socket, address) < 0i32 {
        if (*host).socket != -1i32 { enet_socket_destroy((*host).socket); }
        enet_free((*host).peers as *mut libc::c_void);
        enet_free(host as *mut libc::c_void);
        return 0 as *mut ENetHost
    }
    enet_socket_set_option((*host).socket, ENET_SOCKOPT_NONBLOCK, 1i32);
    enet_socket_set_option((*host).socket, ENET_SOCKOPT_BROADCAST, 1i32);
    enet_socket_set_option((*host).socket, ENET_SOCKOPT_RCVBUF,
                           ENET_HOST_RECEIVE_BUFFER_SIZE as libc::c_int);
    enet_socket_set_option((*host).socket, ENET_SOCKOPT_SNDBUF,
                           ENET_HOST_SEND_BUFFER_SIZE as libc::c_int);
    if !address.is_null() &&
           enet_socket_get_address((*host).socket, &mut (*host).address) <
               0i32 {
        (*host).address = *address
    }
    if 0 == channelLimit ||
           channelLimit >
               ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as
                   libc::c_ulong {
        channelLimit =
            ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as SizeT
    } else if channelLimit <
                  ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as
                      libc::c_ulong {
        channelLimit =
            ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as SizeT
    }
    (*host).randomSeed = host as SizeT as EnetUint32;
    (*host).randomSeed =
        ((*host).randomSeed as
             libc::c_uint).wrapping_add(enet_host_random_seed()) as
            EnetUint32 as EnetUint32;
    (*host).randomSeed =
        (*host).randomSeed << 16i32 | (*host).randomSeed >> 16i32;
    (*host).channelLimit = channelLimit;
    (*host).incomingBandwidth = incomingBandwidth;
    (*host).outgoingBandwidth = outgoingBandwidth;
    (*host).bandwidthThrottleEpoch = 0i32 as EnetUint32;
    (*host).recalculateBandwidthLimits = 0i32;
    (*host).mtu = ENET_HOST_DEFAULT_MTU as libc::c_int as EnetUint32;
    (*host).peerCount = peerCount;
    (*host).commandCount = 0i32 as SizeT;
    (*host).buffer_count = 0i32 as SizeT;
    (*host).checksum = None;
    (*host).receivedAddress.host = 0i32 as EnetUint32;
    (*host).receivedAddress.port = 0i32 as EnetUint16;
    (*host).receivedData = 0 as *mut enet_uint8;
    (*host).receivedDataLength = 0i32 as SizeT;
    (*host).totalSentData = 0i32 as EnetUint32;
    (*host).totalSentPackets = 0i32 as EnetUint32;
    (*host).totalReceivedData = 0i32 as EnetUint32;
    (*host).totalReceivedPackets = 0i32 as EnetUint32;
    (*host).connectedPeers = 0i32 as SizeT;
    (*host).bandwidthLimitedPeers = 0i32 as SizeT;
    (*host).duplicatePeers =
        ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int as SizeT;
    (*host).maximumPacketSize =
        ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE as libc::c_int as SizeT;
    (*host).maximumWaitingData =
        ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA as libc::c_int as SizeT;
    (*host).compressor.context = 0 as *mut libc::c_void;
    (*host).compressor.compress = None;
    (*host).compressor.decompress = None;
    (*host).compressor.destroy = None;
    (*host).intercept = None;
    enet_list_clear(&mut (*host).dispatchQueue);
    currentPeer = (*host).peers;
    while currentPeer <
              &mut *(*host).peers.offset((*host).peerCount as isize) as
                  *mut ENetPeer {
        (*currentPeer).host = host;
        (*currentPeer).incomingPeerID =
            currentPeer.wrapping_offset_from((*host).peers) as libc::c_long as
                EnetUint16;
        (*currentPeer).incomingSessionID = 0xffi32 as enet_uint8;
        (*currentPeer).outgoingSessionID = (*currentPeer).incomingSessionID;
        (*currentPeer).data = 0 as *mut libc::c_void;
        enet_list_clear(&mut (*currentPeer).acknowledgements);
        enet_list_clear(&mut (*currentPeer).sentReliableCommands);
        enet_list_clear(&mut (*currentPeer).sentUnreliableCommands);
        enet_list_clear(&mut (*currentPeer).outgoingReliableCommands);
        enet_list_clear(&mut (*currentPeer).outgoingUnreliableCommands);
        enet_list_clear(&mut (*currentPeer).dispatchedCommands);
        enet_peer_reset(currentPeer);
        currentPeer = currentPeer.offset(1isize)
    }
    return host;
}
/* * Destroys the host and all resources associated with it.
    @param host pointer to the host to destroy
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_destroy(mut host: *mut ENetHost) {
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    if host.is_null() { return }
    enet_socket_destroy((*host).socket);
    currentPeer = (*host).peers;
    while currentPeer <
              &mut *(*host).peers.offset((*host).peerCount as isize) as
                  *mut ENetPeer {
        enet_peer_reset(currentPeer);
        currentPeer = currentPeer.offset(1isize)
    }
    if !(*host).compressor.context.is_null() &&
           (*host).compressor.destroy.is_some() {
        (*host).compressor.destroy.expect("non-null function pointer")((*host).compressor.context);
    }
    enet_free((*host).peers as *mut libc::c_void);
    enet_free(host as *mut libc::c_void);
}
/* * Initiates a connection to a foreign host.
    @param host host seeking the connection
    @param address destination for the connection
    @param channelCount number of channels to allocate
    @param data user data supplied to the receiving host 
    @returns a peer representing the foreign host on success, NULL on failure
    @remarks The peer returned will have not completed the connection until enet_host_service()
    notifies of an ENET_EVENT_TYPE_CONNECT event for the peer.
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_connect(mut host: *mut ENetHost,
                                           mut address: *const ENetAddress,
                                           mut channelCount: SizeT,
                                           mut data: EnetUint32)
 -> *mut ENetPeer {
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut command: ENetProtocol =
        _ENetProtocol{header:
                          ENetProtocolCommandHeader{command: 0,
                                                    channelID: 0,
                                                    reliableSequenceNumber:
                                                        0,},};
    if channelCount <
           ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong
       {
        channelCount =
            ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as SizeT
    } else if channelCount >
                  ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as
                      libc::c_ulong {
        channelCount =
            ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as SizeT
    }
    currentPeer = (*host).peers;
    while currentPeer <
              &mut *(*host).peers.offset((*host).peerCount as isize) as
                  *mut ENetPeer {
        if (*currentPeer).state as libc::c_uint ==
               ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint {
            break ;
        }
        currentPeer = currentPeer.offset(1isize)
    }
    if currentPeer >=
           &mut *(*host).peers.offset((*host).peerCount as isize) as
               *mut ENetPeer {
        return 0 as *mut ENetPeer
    }
    (*currentPeer).channels =
        enet_malloc(channelCount.wrapping_mul(::std::mem::size_of::<ENetChannel>()
                                                  as libc::c_ulong)) as
            *mut ENetChannel;
    if (*currentPeer).channels.is_null() { return 0 as *mut ENetPeer }
    (*currentPeer).channelCount = channelCount;
    (*currentPeer).state = ENET_PEER_STATE_CONNECTING;
    (*currentPeer).address = *address;
    (*host).randomSeed = (*host).randomSeed.wrapping_add(1);
    (*currentPeer).connectID = (*host).randomSeed;
    if (*host).outgoingBandwidth == 0i32 as libc::c_uint {
        (*currentPeer).windowSize =
            ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    } else {
        (*currentPeer).windowSize =
            (*host).outgoingBandwidth.wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE
                                                       as libc::c_int as
                                                       libc::c_uint).wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint)
    }
    if (*currentPeer).windowSize <
           ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        (*currentPeer).windowSize =
            ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    } else if (*currentPeer).windowSize >
                  ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as
                      libc::c_uint {
        (*currentPeer).windowSize =
            ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as EnetUint32
    }
    channel = (*currentPeer).channels;
    while channel <
              &mut *(*currentPeer).channels.offset(channelCount as isize) as
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
    command.header.command =
        (ENET_PROTOCOL_COMMAND_CONNECT as libc::c_int |
             ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int) as
            enet_uint8;
    command.header.channelID = 0xffi32 as enet_uint8;
    command.connect.outgoingPeerID = htons((*currentPeer).incomingPeerID);
    command.connect.incomingSessionID = (*currentPeer).incomingSessionID;
    command.connect.outgoingSessionID = (*currentPeer).outgoingSessionID;
    command.connect.mtu = htonl((*currentPeer).mtu);
    command.connect.windowSize = htonl((*currentPeer).windowSize);
    command.connect.channelCount = htonl(channelCount as Uint32T);
    command.connect.incomingBandwidth = htonl((*host).incomingBandwidth);
    command.connect.outgoingBandwidth = htonl((*host).outgoingBandwidth);
    command.connect.packetThrottleInterval =
        htonl((*currentPeer).packetThrottleInterval);
    command.connect.packetThrottleAcceleration =
        htonl((*currentPeer).packetThrottleAcceleration);
    command.connect.packetThrottleDeceleration =
        htonl((*currentPeer).packetThrottleDeceleration);
    command.connect.connectID = (*currentPeer).connectID;
    command.connect.data = htonl(data);
    enet_peer_queue_outgoing_command(currentPeer, &mut command,
                                     0 as *mut ENetPacket,
                                     0i32 as EnetUint32,
                                     0i32 as EnetUint16);
    return currentPeer;
}
/* * Queues a packet to be sent to all peers associated with the host.
    @param host host on which to broadcast the packet
    @param channelID channel on which to broadcast
    @param packet packet to broadcast
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_broadcast(mut host: *mut ENetHost,
                                             mut channelID: enet_uint8,
                                             mut packet: *mut ENetPacket) {
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    currentPeer = (*host).peers;
    while currentPeer <
              &mut *(*host).peers.offset((*host).peerCount as isize) as
                  *mut ENetPeer {
        if !((*currentPeer).state as libc::c_uint !=
                 ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint) {
            enet_peer_send(currentPeer, channelID, packet);
        }
        currentPeer = currentPeer.offset(1isize)
    }
    if (*packet).referenceCount == 0i32 as libc::c_ulong {
        enet_packet_destroy(packet);
    };
}
/* * Sets the packet compressor the host should use to compress and decompress packets.
    @param host host to enable or disable compression for
    @param compressor callbacks for for the packet compressor; if NULL, then compression is disabled
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_compress(mut host: *mut ENetHost,
                                            mut compressor:
                                                *const ENetCompressor) {
    if !(*host).compressor.context.is_null() &&
           (*host).compressor.destroy.is_some() {
        (*host).compressor.destroy.expect("non-null function pointer")((*host).compressor.context);
    }
    if !compressor.is_null() {
        (*host).compressor = *compressor
    } else { (*host).compressor.context = 0 as *mut libc::c_void };
}
/* * Limits the maximum allowed channels of future incoming connections.
    @param host host to limit
    @param channelLimit the maximum number of channels allowed; if 0, then this is equivalent to ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_channel_limit(mut host: *mut ENetHost,
                                                 mut channelLimit: SizeT) {
    if 0 == channelLimit ||
           channelLimit >
               ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as
                   libc::c_ulong {
        channelLimit =
            ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as SizeT
    } else if channelLimit <
                  ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as
                      libc::c_ulong {
        channelLimit =
            ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as SizeT
    }
    (*host).channelLimit = channelLimit;
}
/* * Adjusts the bandwidth limits of a host.
    @param host host to adjust
    @param incomingBandwidth new incoming bandwidth
    @param outgoingBandwidth new outgoing bandwidth
    @remarks the incoming and outgoing bandwidth parameters are identical in function to those
    specified in enet_host_create().
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_bandwidth_limit(mut host: *mut ENetHost,
                                                   mut incomingBandwidth:
                                                       EnetUint32,
                                                   mut outgoingBandwidth:
                                                       EnetUint32) {
    (*host).incomingBandwidth = incomingBandwidth;
    (*host).outgoingBandwidth = outgoingBandwidth;
    (*host).recalculateBandwidthLimits = 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn enet_host_bandwidth_throttle(mut host:
                                                          *mut ENetHost) {
    let mut timeCurrent: EnetUint32 = enet_time_get();
    let mut elapsedTime: EnetUint32 =
        timeCurrent.wrapping_sub((*host).bandwidthThrottleEpoch);
    let mut peersRemaining: EnetUint32 =
        (*host).connectedPeers as EnetUint32;
    let mut dataTotal: EnetUint32 = !0i32 as EnetUint32;
    let mut bandwidth: EnetUint32 = !0i32 as EnetUint32;
    let mut throttle: EnetUint32 = 0i32 as EnetUint32;
    let mut bandwidthLimit: EnetUint32 = 0i32 as EnetUint32;
    let mut needsAdjustment: libc::c_int =
        if (*host).bandwidthLimitedPeers > 0i32 as libc::c_ulong {
            1i32
        } else { 0i32 };
    let mut peer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut command: ENetProtocol =
        _ENetProtocol{header:
                          ENetProtocolCommandHeader{command: 0,
                                                    channelID: 0,
                                                    reliableSequenceNumber:
                                                        0,},};
    if elapsedTime <
           ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL as libc::c_int as
               libc::c_uint {
        return
    }
    (*host).bandwidthThrottleEpoch = timeCurrent;
    if peersRemaining == 0i32 as libc::c_uint { return }
    if (*host).outgoingBandwidth != 0i32 as libc::c_uint {
        dataTotal = 0i32 as EnetUint32;
        bandwidth =
            (*host).outgoingBandwidth.wrapping_mul(elapsedTime).wrapping_div(1000i32
                                                                                 as
                                                                                 libc::c_uint);
        peer = (*host).peers;
        while peer <
                  &mut *(*host).peers.offset((*host).peerCount as isize) as
                      *mut ENetPeer {
            if !((*peer).state as libc::c_uint !=
                     ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
                     &&
                     (*peer).state as libc::c_uint !=
                         ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                             libc::c_uint) {
                dataTotal =
                    (dataTotal as
                         libc::c_uint).wrapping_add((*peer).outgoingDataTotal)
                        as EnetUint32 as EnetUint32
            }
            peer = peer.offset(1isize)
        }
    }
    while peersRemaining > 0i32 as libc::c_uint && needsAdjustment != 0i32 {
        needsAdjustment = 0i32;
        if dataTotal <= bandwidth {
            throttle =
                ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as EnetUint32
        } else {
            throttle =
                bandwidth.wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE as
                                           libc::c_int as
                                           libc::c_uint).wrapping_div(dataTotal)
        }
        peer = (*host).peers;
        while peer <
                  &mut *(*host).peers.offset((*host).peerCount as isize) as
                      *mut ENetPeer {
            let mut peerBandwidth: EnetUint32 = 0;
            if !((*peer).state as libc::c_uint !=
                     ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
                     &&
                     (*peer).state as libc::c_uint !=
                         ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                             libc::c_uint ||
                     (*peer).incomingBandwidth == 0i32 as libc::c_uint ||
                     (*peer).outgoingBandwidthThrottleEpoch == timeCurrent) {
                peerBandwidth =
                    (*peer).incomingBandwidth.wrapping_mul(elapsedTime).wrapping_div(1000i32
                                                                                         as
                                                                                         libc::c_uint);
                if !(throttle.wrapping_mul((*peer).outgoingDataTotal).wrapping_div(ENET_PEER_PACKET_THROTTLE_SCALE
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint)
                         <= peerBandwidth) {
                    (*peer).packetThrottleLimit =
                        peerBandwidth.wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE
                                                       as libc::c_int as
                                                       libc::c_uint).wrapping_div((*peer).outgoingDataTotal);
                    if (*peer).packetThrottleLimit == 0i32 as libc::c_uint {
                        (*peer).packetThrottleLimit = 1i32 as EnetUint32
                    }
                    if (*peer).packetThrottle > (*peer).packetThrottleLimit {
                        (*peer).packetThrottle = (*peer).packetThrottleLimit
                    }
                    (*peer).outgoingBandwidthThrottleEpoch = timeCurrent;
                    (*peer).incomingDataTotal = 0i32 as EnetUint32;
                    (*peer).outgoingDataTotal = 0i32 as EnetUint32;
                    needsAdjustment = 1i32;
                    peersRemaining = peersRemaining.wrapping_sub(1);
                    bandwidth =
                        (bandwidth as
                             libc::c_uint).wrapping_sub(peerBandwidth) as
                            EnetUint32 as EnetUint32;
                    dataTotal =
                        (dataTotal as
                             libc::c_uint).wrapping_sub(peerBandwidth) as
                            EnetUint32 as EnetUint32
                }
            }
            peer = peer.offset(1isize)
        }
    }
    if peersRemaining > 0i32 as libc::c_uint {
        if dataTotal <= bandwidth {
            throttle =
                ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as EnetUint32
        } else {
            throttle =
                bandwidth.wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE as
                                           libc::c_int as
                                           libc::c_uint).wrapping_div(dataTotal)
        }
        peer = (*host).peers;
        while peer <
                  &mut *(*host).peers.offset((*host).peerCount as isize) as
                      *mut ENetPeer {
            if !((*peer).state as libc::c_uint !=
                     ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
                     &&
                     (*peer).state as libc::c_uint !=
                         ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                             libc::c_uint ||
                     (*peer).outgoingBandwidthThrottleEpoch == timeCurrent) {
                (*peer).packetThrottleLimit = throttle;
                if (*peer).packetThrottle > (*peer).packetThrottleLimit {
                    (*peer).packetThrottle = (*peer).packetThrottleLimit
                }
                (*peer).incomingDataTotal = 0i32 as EnetUint32;
                (*peer).outgoingDataTotal = 0i32 as EnetUint32
            }
            peer = peer.offset(1isize)
        }
    }
    if 0 != (*host).recalculateBandwidthLimits {
        (*host).recalculateBandwidthLimits = 0i32;
        peersRemaining = (*host).connectedPeers as EnetUint32;
        bandwidth = (*host).incomingBandwidth;
        needsAdjustment = 1i32;
        if bandwidth == 0i32 as libc::c_uint {
            bandwidthLimit = 0i32 as EnetUint32
        } else {
            while peersRemaining > 0i32 as libc::c_uint &&
                      needsAdjustment != 0i32 {
                needsAdjustment = 0i32;
                bandwidthLimit = bandwidth.wrapping_div(peersRemaining);
                peer = (*host).peers;
                while peer <
                          &mut *(*host).peers.offset((*host).peerCount as
                                                         isize) as
                              *mut ENetPeer {
                    if !((*peer).state as libc::c_uint !=
                             ENET_PEER_STATE_CONNECTED as libc::c_int as
                                 libc::c_uint &&
                             (*peer).state as libc::c_uint !=
                                 ENET_PEER_STATE_DISCONNECT_LATER as
                                     libc::c_int as libc::c_uint ||
                             (*peer).incomingBandwidthThrottleEpoch ==
                                 timeCurrent) {
                        if !((*peer).outgoingBandwidth > 0i32 as libc::c_uint
                                 &&
                                 (*peer).outgoingBandwidth >= bandwidthLimit)
                           {
                            (*peer).incomingBandwidthThrottleEpoch =
                                timeCurrent;
                            needsAdjustment = 1i32;
                            peersRemaining = peersRemaining.wrapping_sub(1);
                            bandwidth =
                                (bandwidth as
                                     libc::c_uint).wrapping_sub((*peer).outgoingBandwidth)
                                    as EnetUint32 as EnetUint32
                        }
                    }
                    peer = peer.offset(1isize)
                }
            }
        }
        peer = (*host).peers;
        while peer <
                  &mut *(*host).peers.offset((*host).peerCount as isize) as
                      *mut ENetPeer {
            if !((*peer).state as libc::c_uint !=
                     ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
                     &&
                     (*peer).state as libc::c_uint !=
                         ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as
                             libc::c_uint) {
                command.header.command =
                    (ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT as libc::c_int |
                         ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as
                             libc::c_int) as enet_uint8;
                command.header.channelID = 0xffi32 as enet_uint8;
                command.bandwidthLimit.outgoingBandwidth =
                    htonl((*host).outgoingBandwidth);
                if (*peer).incomingBandwidthThrottleEpoch == timeCurrent {
                    command.bandwidthLimit.incomingBandwidth =
                        htonl((*peer).outgoingBandwidth)
                } else {
                    command.bandwidthLimit.incomingBandwidth =
                        htonl(bandwidthLimit)
                }
                enet_peer_queue_outgoing_command(peer, &mut command,
                                                 0 as *mut ENetPacket,
                                                 0i32 as EnetUint32,
                                                 0i32 as EnetUint16);
            }
            peer = peer.offset(1isize)
        }
    };
}