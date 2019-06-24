use libc;
extern "C" {
    #[no_mangle]
    fn enet_host_compress(_: *mut ENetHost, _: *const ENetCompressor);
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
}
pub type SizeT = libc::c_ulong;
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
pub type ENetHost = _ENetHost;
pub type ENetRangeCoder = _ENetRangeCoder;
/* context exclusion roughly halves compression speed, so disable for now */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetRangeCoder {
    pub symbols: [ENetSymbol; 4096],
}
pub type ENetSymbol = _ENetSymbol;
/* * 
 @file compress.c
 @brief An adaptive order-2 PPM range coder
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetSymbol {
    pub value: enet_uint8,
    pub count: enet_uint8,
    pub under: EnetUint16,
    pub left: EnetUint16,
    pub right: EnetUint16,
    pub symbols: EnetUint16,
    pub escapes: EnetUint16,
    pub total: EnetUint16,
    pub parent: EnetUint16,
}
pub const ENET_CONTEXT_SYMBOL_MINIMUM: Unnamed = 1;
pub const ENET_CONTEXT_ESCAPE_MINIMUM: Unnamed = 1;
pub const ENET_SUBCONTEXT_ORDER: Unnamed = 2;
pub const ENET_RANGE_CODER_BOTTOM: Unnamed = 65536;
pub const ENET_SUBCONTEXT_SYMBOL_DELTA: Unnamed = 2;
pub const ENET_SUBCONTEXT_ESCAPE_DELTA: Unnamed = 5;
pub const ENET_CONTEXT_SYMBOL_DELTA: Unnamed = 3;
pub const ENET_RANGE_CODER_TOP: Unnamed = 16777216;
/* adaptation constants tuned aggressively for small packet sizes rather than large file compression */
pub type Unnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn enet_range_coder_create() -> *mut libc::c_void {
    let mut rangeCoder: *mut ENetRangeCoder =
        enet_malloc(::std::mem::size_of::<ENetRangeCoder>() as libc::c_ulong)
            as *mut ENetRangeCoder;
    if rangeCoder.is_null() { return 0 as *mut libc::c_void }
    return rangeCoder as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn enet_range_coder_destroy(mut context:
                                                      *mut libc::c_void) {
    let mut rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    if rangeCoder.is_null() { return }
    enet_free(rangeCoder as *mut libc::c_void);
}
unsafe extern "C" fn enet_symbol_rescale(mut symbol: *mut ENetSymbol)
 -> EnetUint16 {
    let mut total: EnetUint16 = 0i32 as EnetUint16;
    loop  {
        (*symbol).count =
            ((*symbol).count as libc::c_int -
                 ((*symbol).count as libc::c_int >> 1i32)) as enet_uint8;
        (*symbol).under = (*symbol).count as EnetUint16;
        if 0 != (*symbol).left {
            (*symbol).under =
                ((*symbol).under as libc::c_int +
                     enet_symbol_rescale(symbol.offset((*symbol).left as
                                                           libc::c_int as
                                                           isize)) as
                         libc::c_int) as EnetUint16
        }
        total =
            (total as libc::c_int + (*symbol).under as libc::c_int) as
                EnetUint16;
        if 0 == (*symbol).right { break ; }
        symbol = symbol.offset((*symbol).right as libc::c_int as isize)
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn enet_range_coder_compress(mut context:
                                                       *mut libc::c_void,
                                                   mut inBuffers:
                                                       *const ENetBuffer,
                                                   mut inBufferCount: SizeT,
                                                   mut inLimit: SizeT,
                                                   mut outData:
                                                       *mut enet_uint8,
                                                   mut outLimit: SizeT)
 -> SizeT {
    let mut rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    let mut outStart: *mut enet_uint8 = outData;
    let mut outEnd: *mut enet_uint8 =
        &mut *outData.offset(outLimit as isize) as *mut enet_uint8;
    let mut inData: *const enet_uint8 = 0 as *const enet_uint8;
    let mut inEnd: *const enet_uint8 = 0 as *const enet_uint8;
    let mut encodeLow: EnetUint32 = 0i32 as EnetUint32;
    let mut encodeRange: EnetUint32 = !0i32 as EnetUint32;
    let mut root: *mut ENetSymbol = 0 as *mut ENetSymbol;
    let mut predicted: EnetUint16 = 0i32 as EnetUint16;
    let mut order: SizeT = 0i32 as SizeT;
    let mut nextSymbol: SizeT = 0i32 as SizeT;
    if rangeCoder.is_null() || inBufferCount <= 0i32 as libc::c_ulong ||
           inLimit <= 0i32 as libc::c_ulong {
        return 0i32 as SizeT
    }
    inData = (*inBuffers).data as *const enet_uint8;
    inEnd =
        &*inData.offset((*inBuffers).data_length as isize) as
            *const enet_uint8;
    inBuffers = inBuffers.offset(1isize);
    inBufferCount = inBufferCount.wrapping_sub(1);
    let fresh0 = nextSymbol;
    nextSymbol = nextSymbol.wrapping_add(1);
    root =
        &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh0 as isize) as
            *mut ENetSymbol;
    (*root).value = 0i32 as enet_uint8;
    (*root).count = 0i32 as enet_uint8;
    (*root).under = 0i32 as EnetUint16;
    (*root).left = 0i32 as EnetUint16;
    (*root).right = 0i32 as EnetUint16;
    (*root).symbols = 0i32 as EnetUint16;
    (*root).escapes = 0i32 as EnetUint16;
    (*root).total = 0i32 as EnetUint16;
    (*root).parent = 0i32 as EnetUint16;
    (*root).escapes =
        ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int as EnetUint16;
    (*root).total =
        (ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int +
             256i32 * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int) as
            EnetUint16;
    (*root).symbols = 0i32 as EnetUint16;
    let mut current_block_237: u64;
    loop  {
        let mut subcontext: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut symbol: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut value: enet_uint8 = 0;
        let mut count: EnetUint16 = 0;
        let mut under: EnetUint16 = 0;
        let mut parent: *mut EnetUint16 = &mut predicted;
        let mut total: EnetUint16 = 0;
        if inData >= inEnd {
            if inBufferCount <= 0i32 as libc::c_ulong { break ; }
            inData = (*inBuffers).data as *const enet_uint8;
            inEnd =
                &*inData.offset((*inBuffers).data_length as isize) as
                    *const enet_uint8;
            inBuffers = inBuffers.offset(1isize);
            inBufferCount = inBufferCount.wrapping_sub(1)
        }
        let fresh1 = inData;
        inData = inData.offset(1);
        value = *fresh1;
        subcontext =
            &mut *(*rangeCoder).symbols.as_mut_ptr().offset(predicted as
                                                                isize) as
                *mut ENetSymbol;
        loop  {
            if !(subcontext != root) {
                current_block_237 = 9343041660989783267;
                break ;
            }
            under = (value as libc::c_int * 0i32) as EnetUint16;
            count = 0i32 as EnetUint16;
            if 0 == (*subcontext).symbols {
                let fresh2 = nextSymbol;
                nextSymbol = nextSymbol.wrapping_add(1);
                symbol =
                    &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh2 as
                                                                        isize)
                        as *mut ENetSymbol;
                (*symbol).value = value;
                (*symbol).count =
                    ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                (*symbol).under =
                    ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as
                        EnetUint16;
                (*symbol).left = 0i32 as EnetUint16;
                (*symbol).right = 0i32 as EnetUint16;
                (*symbol).symbols = 0i32 as EnetUint16;
                (*symbol).escapes = 0i32 as EnetUint16;
                (*symbol).total = 0i32 as EnetUint16;
                (*symbol).parent = 0i32 as EnetUint16;
                (*subcontext).symbols =
                    symbol.wrapping_offset_from(subcontext) as libc::c_long as
                        EnetUint16
            } else {
                let mut node: *mut ENetSymbol =
                    subcontext.offset((*subcontext).symbols as libc::c_int as
                                          isize);
                loop  {
                    if (value as libc::c_int) < (*node).value as libc::c_int {
                        (*node).under =
                            ((*node).under as libc::c_int +
                                 ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                                as EnetUint16;
                        if 0 != (*node).left {
                            node =
                                node.offset((*node).left as libc::c_int as
                                                isize)
                        } else {
                            let fresh3 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol =
                                &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh3
                                                                                    as
                                                                                    isize)
                                    as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as
                                    enet_uint8;
                            (*symbol).under =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as
                                    EnetUint16;
                            (*symbol).left = 0i32 as EnetUint16;
                            (*symbol).right = 0i32 as EnetUint16;
                            (*symbol).symbols = 0i32 as EnetUint16;
                            (*symbol).escapes = 0i32 as EnetUint16;
                            (*symbol).total = 0i32 as EnetUint16;
                            (*symbol).parent = 0i32 as EnetUint16;
                            (*node).left =
                                symbol.wrapping_offset_from(node) as
                                    libc::c_long as EnetUint16;
                            break ;
                        }
                    } else if value as libc::c_int >
                                  (*node).value as libc::c_int {
                        under =
                            (under as libc::c_int +
                                 (*node).under as libc::c_int) as EnetUint16;
                        if 0 != (*node).right {
                            node =
                                node.offset((*node).right as libc::c_int as
                                                isize)
                        } else {
                            let fresh4 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol =
                                &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh4
                                                                                    as
                                                                                    isize)
                                    as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as
                                    enet_uint8;
                            (*symbol).under =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as
                                    EnetUint16;
                            (*symbol).left = 0i32 as EnetUint16;
                            (*symbol).right = 0i32 as EnetUint16;
                            (*symbol).symbols = 0i32 as EnetUint16;
                            (*symbol).escapes = 0i32 as EnetUint16;
                            (*symbol).total = 0i32 as EnetUint16;
                            (*symbol).parent = 0i32 as EnetUint16;
                            (*node).right =
                                symbol.wrapping_offset_from(node) as
                                    libc::c_long as EnetUint16;
                            break ;
                        }
                    } else {
                        count =
                            (count as libc::c_int +
                                 (*node).count as libc::c_int) as EnetUint16;
                        under =
                            (under as libc::c_int +
                                 ((*node).under as libc::c_int -
                                      (*node).count as libc::c_int)) as
                                EnetUint16;
                        (*node).under =
                            ((*node).under as libc::c_int +
                                 ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                                as EnetUint16;
                        (*node).count =
                            ((*node).count as libc::c_int +
                                 ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                                as enet_uint8;
                        symbol = node;
                        break ;
                    }
                }
            }
            *parent =
                symbol.wrapping_offset_from((*rangeCoder).symbols.as_mut_ptr())
                    as libc::c_long as EnetUint16;
            parent = &mut (*symbol).parent;
            total = (*subcontext).total;
            if count as libc::c_int > 0i32 {
                encodeRange =
                    (encodeRange as
                         libc::c_uint).wrapping_div(total as libc::c_uint) as
                        EnetUint32 as EnetUint32;
                encodeLow =
                    (encodeLow as
                         libc::c_uint).wrapping_add((((*subcontext).escapes as
                                                          libc::c_int +
                                                          under as
                                                              libc::c_int) as
                                                         libc::c_uint).wrapping_mul(encodeRange))
                        as EnetUint32 as EnetUint32;
                encodeRange =
                    (encodeRange as
                         libc::c_uint).wrapping_mul(count as libc::c_uint) as
                        EnetUint32 as EnetUint32;
                loop  {
                    if encodeLow ^ encodeLow.wrapping_add(encodeRange) >=
                           ENET_RANGE_CODER_TOP as libc::c_int as libc::c_uint
                       {
                        if encodeRange >=
                               ENET_RANGE_CODER_BOTTOM as libc::c_int as
                                   libc::c_uint {
                            break ;
                        }
                        encodeRange =
                            encodeLow.wrapping_neg() &
                                (ENET_RANGE_CODER_BOTTOM as libc::c_int -
                                     1i32) as libc::c_uint
                    }
                    if outData >= outEnd { return 0i32 as SizeT }
                    let fresh5 = outData;
                    outData = outData.offset(1);
                    *fresh5 = (encodeLow >> 24i32) as enet_uint8;
                    encodeRange <<= 8i32;
                    encodeLow <<= 8i32
                }
            } else {
                if (*subcontext).escapes as libc::c_int > 0i32 &&
                       ((*subcontext).escapes as libc::c_int) <
                           total as libc::c_int {
                    encodeRange =
                        (encodeRange as
                             libc::c_uint).wrapping_div(total as libc::c_uint)
                            as EnetUint32 as EnetUint32;
                    encodeLow =
                        (encodeLow as
                             libc::c_uint).wrapping_add((0i32 as
                                                             libc::c_uint).wrapping_mul(encodeRange))
                            as EnetUint32 as EnetUint32;
                    encodeRange =
                        (encodeRange as
                             libc::c_uint).wrapping_mul((*subcontext).escapes
                                                            as libc::c_uint)
                            as EnetUint32 as EnetUint32;
                    loop  {
                        if encodeLow ^ encodeLow.wrapping_add(encodeRange) >=
                               ENET_RANGE_CODER_TOP as libc::c_int as
                                   libc::c_uint {
                            if encodeRange >=
                                   ENET_RANGE_CODER_BOTTOM as libc::c_int as
                                       libc::c_uint {
                                break ;
                            }
                            encodeRange =
                                encodeLow.wrapping_neg() &
                                    (ENET_RANGE_CODER_BOTTOM as libc::c_int -
                                         1i32) as libc::c_uint
                        }
                        if outData >= outEnd { return 0i32 as SizeT }
                        let fresh6 = outData;
                        outData = outData.offset(1);
                        *fresh6 = (encodeLow >> 24i32) as enet_uint8;
                        encodeRange <<= 8i32;
                        encodeLow <<= 8i32
                    }
                }
                (*subcontext).escapes =
                    ((*subcontext).escapes as libc::c_int +
                         ENET_SUBCONTEXT_ESCAPE_DELTA as libc::c_int) as
                        EnetUint16;
                (*subcontext).total =
                    ((*subcontext).total as libc::c_int +
                         ENET_SUBCONTEXT_ESCAPE_DELTA as libc::c_int) as
                        EnetUint16
            }
            (*subcontext).total =
                ((*subcontext).total as libc::c_int +
                     ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int) as
                    EnetUint16;
            if count as libc::c_int >
                   0xffi32 -
                       2i32 * ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int ||
                   (*subcontext).total as libc::c_int >
                       ENET_RANGE_CODER_BOTTOM as libc::c_int - 0x100i32 {
                (*subcontext).total =
                    (if 0 != (*subcontext).symbols as libc::c_int {
                         enet_symbol_rescale(subcontext.offset((*subcontext).symbols
                                                                   as
                                                                   libc::c_int
                                                                   as isize))
                             as libc::c_int
                     } else { 0i32 }) as EnetUint16;
                (*subcontext).escapes =
                    ((*subcontext).escapes as libc::c_int -
                         ((*subcontext).escapes as libc::c_int >> 1i32)) as
                        EnetUint16;
                (*subcontext).total =
                    ((*subcontext).total as libc::c_int +
                         ((*subcontext).escapes as libc::c_int +
                              256i32 * 0i32)) as EnetUint16
            }
            if count as libc::c_int > 0i32 {
                current_block_237 = 7983517938030562037;
                break ;
            }
            subcontext =
                &mut *(*rangeCoder).symbols.as_mut_ptr().offset((*subcontext).parent
                                                                    as isize)
                    as *mut ENetSymbol
        }
        match current_block_237 {
            9343041660989783267 => {
                under =
                    (value as libc::c_int *
                         ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int) as
                        EnetUint16;
                count =
                    ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int as EnetUint16;
                if 0 == (*root).symbols {
                    let fresh7 = nextSymbol;
                    nextSymbol = nextSymbol.wrapping_add(1);
                    symbol =
                        &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh7
                                                                            as
                                                                            isize)
                            as *mut ENetSymbol;
                    (*symbol).value = value;
                    (*symbol).count =
                        ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as
                            enet_uint8;
                    (*symbol).under =
                        ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as
                            EnetUint16;
                    (*symbol).left = 0i32 as EnetUint16;
                    (*symbol).right = 0i32 as EnetUint16;
                    (*symbol).symbols = 0i32 as EnetUint16;
                    (*symbol).escapes = 0i32 as EnetUint16;
                    (*symbol).total = 0i32 as EnetUint16;
                    (*symbol).parent = 0i32 as EnetUint16;
                    (*root).symbols =
                        symbol.wrapping_offset_from(root) as libc::c_long as
                            EnetUint16
                } else {
                    let mut node_0: *mut ENetSymbol =
                        root.offset((*root).symbols as libc::c_int as isize);
                    loop  {
                        if (value as libc::c_int) <
                               (*node_0).value as libc::c_int {
                            (*node_0).under =
                                ((*node_0).under as libc::c_int +
                                     ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                                    as EnetUint16;
                            if 0 != (*node_0).left {
                                node_0 =
                                    node_0.offset((*node_0).left as
                                                      libc::c_int as isize)
                            } else {
                                let fresh8 = nextSymbol;
                                nextSymbol = nextSymbol.wrapping_add(1);
                                symbol =
                                    &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh8
                                                                                        as
                                                                                        isize)
                                        as *mut ENetSymbol;
                                (*symbol).value = value;
                                (*symbol).count =
                                    ENET_CONTEXT_SYMBOL_DELTA as libc::c_int
                                        as enet_uint8;
                                (*symbol).under =
                                    ENET_CONTEXT_SYMBOL_DELTA as libc::c_int
                                        as EnetUint16;
                                (*symbol).left = 0i32 as EnetUint16;
                                (*symbol).right = 0i32 as EnetUint16;
                                (*symbol).symbols = 0i32 as EnetUint16;
                                (*symbol).escapes = 0i32 as EnetUint16;
                                (*symbol).total = 0i32 as EnetUint16;
                                (*symbol).parent = 0i32 as EnetUint16;
                                (*node_0).left =
                                    symbol.wrapping_offset_from(node_0) as
                                        libc::c_long as EnetUint16;
                                break ;
                            }
                        } else if value as libc::c_int >
                                      (*node_0).value as libc::c_int {
                            under =
                                (under as libc::c_int +
                                     (*node_0).under as libc::c_int) as
                                    EnetUint16;
                            if 0 != (*node_0).right {
                                node_0 =
                                    node_0.offset((*node_0).right as
                                                      libc::c_int as isize)
                            } else {
                                let fresh9 = nextSymbol;
                                nextSymbol = nextSymbol.wrapping_add(1);
                                symbol =
                                    &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh9
                                                                                        as
                                                                                        isize)
                                        as *mut ENetSymbol;
                                (*symbol).value = value;
                                (*symbol).count =
                                    ENET_CONTEXT_SYMBOL_DELTA as libc::c_int
                                        as enet_uint8;
                                (*symbol).under =
                                    ENET_CONTEXT_SYMBOL_DELTA as libc::c_int
                                        as EnetUint16;
                                (*symbol).left = 0i32 as EnetUint16;
                                (*symbol).right = 0i32 as EnetUint16;
                                (*symbol).symbols = 0i32 as EnetUint16;
                                (*symbol).escapes = 0i32 as EnetUint16;
                                (*symbol).total = 0i32 as EnetUint16;
                                (*symbol).parent = 0i32 as EnetUint16;
                                (*node_0).right =
                                    symbol.wrapping_offset_from(node_0) as
                                        libc::c_long as EnetUint16;
                                break ;
                            }
                        } else {
                            count =
                                (count as libc::c_int +
                                     (*node_0).count as libc::c_int) as
                                    EnetUint16;
                            under =
                                (under as libc::c_int +
                                     ((*node_0).under as libc::c_int -
                                          (*node_0).count as libc::c_int)) as
                                    EnetUint16;
                            (*node_0).under =
                                ((*node_0).under as libc::c_int +
                                     ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                                    as EnetUint16;
                            (*node_0).count =
                                ((*node_0).count as libc::c_int +
                                     ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                                    as enet_uint8;
                            symbol = node_0;
                            break ;
                        }
                    }
                }
                *parent =
                    symbol.wrapping_offset_from((*rangeCoder).symbols.as_mut_ptr())
                        as libc::c_long as EnetUint16;
                parent = &mut (*symbol).parent;
                total = (*root).total;
                encodeRange =
                    (encodeRange as
                         libc::c_uint).wrapping_div(total as libc::c_uint) as
                        EnetUint32 as EnetUint32;
                encodeLow =
                    (encodeLow as
                         libc::c_uint).wrapping_add((((*root).escapes as
                                                          libc::c_int +
                                                          under as
                                                              libc::c_int) as
                                                         libc::c_uint).wrapping_mul(encodeRange))
                        as EnetUint32 as EnetUint32;
                encodeRange =
                    (encodeRange as
                         libc::c_uint).wrapping_mul(count as libc::c_uint) as
                        EnetUint32 as EnetUint32;
                loop  {
                    if encodeLow ^ encodeLow.wrapping_add(encodeRange) >=
                           ENET_RANGE_CODER_TOP as libc::c_int as libc::c_uint
                       {
                        if encodeRange >=
                               ENET_RANGE_CODER_BOTTOM as libc::c_int as
                                   libc::c_uint {
                            break ;
                        }
                        encodeRange =
                            encodeLow.wrapping_neg() &
                                (ENET_RANGE_CODER_BOTTOM as libc::c_int -
                                     1i32) as libc::c_uint
                    }
                    if outData >= outEnd { return 0i32 as SizeT }
                    let fresh10 = outData;
                    outData = outData.offset(1);
                    *fresh10 = (encodeLow >> 24i32) as enet_uint8;
                    encodeRange <<= 8i32;
                    encodeLow <<= 8i32
                }
                (*root).total =
                    ((*root).total as libc::c_int +
                         ENET_CONTEXT_SYMBOL_DELTA as libc::c_int) as
                        EnetUint16;
                if count as libc::c_int >
                       0xffi32 -
                           2i32 * ENET_CONTEXT_SYMBOL_DELTA as libc::c_int +
                           ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int ||
                       (*root).total as libc::c_int >
                           ENET_RANGE_CODER_BOTTOM as libc::c_int - 0x100i32 {
                    (*root).total =
                        (if 0 != (*root).symbols as libc::c_int {
                             enet_symbol_rescale(root.offset((*root).symbols
                                                                 as
                                                                 libc::c_int
                                                                 as isize)) as
                                 libc::c_int
                         } else { 0i32 }) as EnetUint16;
                    (*root).escapes =
                        ((*root).escapes as libc::c_int -
                             ((*root).escapes as libc::c_int >> 1i32)) as
                            EnetUint16;
                    (*root).total =
                        ((*root).total as libc::c_int +
                             ((*root).escapes as libc::c_int +
                                  256i32 *
                                      ENET_CONTEXT_SYMBOL_MINIMUM as
                                          libc::c_int)) as EnetUint16
                }
            }
            _ => { }
        }
        if order >= ENET_SUBCONTEXT_ORDER as libc::c_int as libc::c_ulong {
            predicted = (*rangeCoder).symbols[predicted as usize].parent
        } else { order = order.wrapping_add(1) }
        if nextSymbol >=
               (::std::mem::size_of::<[ENetSymbol; 4096]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<ENetSymbol>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(ENET_SUBCONTEXT_ORDER
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
           {
            nextSymbol = 0i32 as SizeT;
            let fresh11 = nextSymbol;
            nextSymbol = nextSymbol.wrapping_add(1);
            root =
                &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh11 as
                                                                    isize) as
                    *mut ENetSymbol;
            (*root).value = 0i32 as enet_uint8;
            (*root).count = 0i32 as enet_uint8;
            (*root).under = 0i32 as EnetUint16;
            (*root).left = 0i32 as EnetUint16;
            (*root).right = 0i32 as EnetUint16;
            (*root).symbols = 0i32 as EnetUint16;
            (*root).escapes = 0i32 as EnetUint16;
            (*root).total = 0i32 as EnetUint16;
            (*root).parent = 0i32 as EnetUint16;
            (*root).escapes =
                ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int as EnetUint16;
            (*root).total =
                (ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int +
                     256i32 * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int) as
                    EnetUint16;
            (*root).symbols = 0i32 as EnetUint16;
            predicted = 0i32 as EnetUint16;
            order = 0i32 as SizeT
        }
    }
    while 0 != encodeLow {
        if outData >= outEnd { return 0i32 as SizeT }
        let fresh12 = outData;
        outData = outData.offset(1);
        *fresh12 = (encodeLow >> 24i32) as enet_uint8;
        encodeLow <<= 8i32
    }
    return outData.wrapping_offset_from(outStart) as libc::c_long as SizeT;
}
#[no_mangle]
pub unsafe extern "C" fn enet_range_coder_decompress(mut context:
                                                         *mut libc::c_void,
                                                     mut inData:
                                                         *const enet_uint8,
                                                     mut inLimit: SizeT,
                                                     mut outData:
                                                         *mut enet_uint8,
                                                     mut outLimit: SizeT)
 -> SizeT {
    let mut rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    let mut outStart: *mut enet_uint8 = outData;
    let mut outEnd: *mut enet_uint8 =
        &mut *outData.offset(outLimit as isize) as *mut enet_uint8;
    let mut inEnd: *const enet_uint8 =
        &*inData.offset(inLimit as isize) as *const enet_uint8;
    let mut decodeLow: EnetUint32 = 0i32 as EnetUint32;
    let mut decodeCode: EnetUint32 = 0i32 as EnetUint32;
    let mut decodeRange: EnetUint32 = !0i32 as EnetUint32;
    let mut root: *mut ENetSymbol = 0 as *mut ENetSymbol;
    let mut predicted: EnetUint16 = 0i32 as EnetUint16;
    let mut order: SizeT = 0i32 as SizeT;
    let mut nextSymbol: SizeT = 0i32 as SizeT;
    if rangeCoder.is_null() || inLimit <= 0i32 as libc::c_ulong {
        return 0i32 as SizeT
    }
    let fresh13 = nextSymbol;
    nextSymbol = nextSymbol.wrapping_add(1);
    root =
        &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh13 as isize) as
            *mut ENetSymbol;
    (*root).value = 0i32 as enet_uint8;
    (*root).count = 0i32 as enet_uint8;
    (*root).under = 0i32 as EnetUint16;
    (*root).left = 0i32 as EnetUint16;
    (*root).right = 0i32 as EnetUint16;
    (*root).symbols = 0i32 as EnetUint16;
    (*root).escapes = 0i32 as EnetUint16;
    (*root).total = 0i32 as EnetUint16;
    (*root).parent = 0i32 as EnetUint16;
    (*root).escapes =
        ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int as EnetUint16;
    (*root).total =
        (ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int +
             256i32 * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int) as
            EnetUint16;
    (*root).symbols = 0i32 as EnetUint16;
    if inData < inEnd {
        let fresh14 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh14 as libc::c_int) << 24i32) as libc::c_uint
    }
    if inData < inEnd {
        let fresh15 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh15 as libc::c_int) << 16i32) as libc::c_uint
    }
    if inData < inEnd {
        let fresh16 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh16 as libc::c_int) << 8i32) as libc::c_uint
    }
    if inData < inEnd {
        let fresh17 = inData;
        inData = inData.offset(1);
        decodeCode |= *fresh17 as libc::c_uint
    }
    let mut current_block_297: u64;
    loop  {
        let mut subcontext: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut symbol: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut patch: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut value: enet_uint8 = 0i32 as enet_uint8;
        let mut code: EnetUint16 = 0;
        let mut under: EnetUint16 = 0;
        let mut count: EnetUint16 = 0;
        let mut bottom: EnetUint16 = 0;
        let mut parent: *mut EnetUint16 = &mut predicted;
        let mut total: EnetUint16 = 0;
        subcontext =
            &mut *(*rangeCoder).symbols.as_mut_ptr().offset(predicted as
                                                                isize) as
                *mut ENetSymbol;
        loop  {
            if !(subcontext != root) {
                current_block_297 = 6950488995570599823;
                break ;
            }
            if !((*subcontext).escapes as libc::c_int <= 0i32) {
                total = (*subcontext).total;
                if !((*subcontext).escapes as libc::c_int >=
                         total as libc::c_int) {
                    decodeRange =
                        (decodeRange as
                             libc::c_uint).wrapping_div(total as libc::c_uint)
                            as EnetUint32 as EnetUint32;
                    code =
                        decodeCode.wrapping_sub(decodeLow).wrapping_div(decodeRange)
                            as EnetUint16;
                    if (code as libc::c_int) <
                           (*subcontext).escapes as libc::c_int {
                        decodeLow =
                            (decodeLow as
                                 libc::c_uint).wrapping_add((0i32 as
                                                                 libc::c_uint).wrapping_mul(decodeRange))
                                as EnetUint32 as EnetUint32;
                        decodeRange =
                            (decodeRange as
                                 libc::c_uint).wrapping_mul((*subcontext).escapes
                                                                as
                                                                libc::c_uint)
                                as EnetUint32 as EnetUint32;
                        loop  {
                            if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                                   >=
                                   ENET_RANGE_CODER_TOP as libc::c_int as
                                       libc::c_uint {
                                if decodeRange >=
                                       ENET_RANGE_CODER_BOTTOM as libc::c_int
                                           as libc::c_uint {
                                    break ;
                                }
                                decodeRange =
                                    decodeLow.wrapping_neg() &
                                        (ENET_RANGE_CODER_BOTTOM as
                                             libc::c_int - 1i32) as
                                            libc::c_uint
                            }
                            decodeCode <<= 8i32;
                            if inData < inEnd {
                                let fresh18 = inData;
                                inData = inData.offset(1);
                                decodeCode |= *fresh18 as libc::c_uint
                            }
                            decodeRange <<= 8i32;
                            decodeLow <<= 8i32
                        }
                    } else {
                        code =
                            (code as libc::c_int -
                                 (*subcontext).escapes as libc::c_int) as
                                EnetUint16;
                        under = 0i32 as EnetUint16;
                        count = 0i32 as EnetUint16;
                        if 0 == (*subcontext).symbols {
                            return 0i32 as SizeT
                        } else {
                            let mut node: *mut ENetSymbol =
                                subcontext.offset((*subcontext).symbols as
                                                      libc::c_int as isize);
                            loop  {
                                let mut after: EnetUint16 =
                                    (under as libc::c_int +
                                         (*node).under as libc::c_int +
                                         ((*node).value as libc::c_int + 1i32)
                                             * 0i32) as EnetUint16;
                                let mut before: EnetUint16 =
                                    ((*node).count as libc::c_int + 0i32) as
                                        EnetUint16;
                                if code as libc::c_int >= after as libc::c_int
                                   {
                                    under =
                                        (under as libc::c_int +
                                             (*node).under as libc::c_int) as
                                            EnetUint16;
                                    if 0 != (*node).right {
                                        node =
                                            node.offset((*node).right as
                                                            libc::c_int as
                                                            isize)
                                    } else { return 0i32 as SizeT }
                                } else if (code as libc::c_int) <
                                              after as libc::c_int -
                                                  before as libc::c_int {
                                    (*node).under =
                                        ((*node).under as libc::c_int +
                                             ENET_SUBCONTEXT_SYMBOL_DELTA as
                                                 libc::c_int) as EnetUint16;
                                    if 0 != (*node).left {
                                        node =
                                            node.offset((*node).left as
                                                            libc::c_int as
                                                            isize)
                                    } else { return 0i32 as SizeT }
                                } else {
                                    value = (*node).value;
                                    count =
                                        (count as libc::c_int +
                                             (*node).count as libc::c_int) as
                                            EnetUint16;
                                    under =
                                        (after as libc::c_int -
                                             before as libc::c_int) as
                                            EnetUint16;
                                    (*node).under =
                                        ((*node).under as libc::c_int +
                                             ENET_SUBCONTEXT_SYMBOL_DELTA as
                                                 libc::c_int) as EnetUint16;
                                    (*node).count =
                                        ((*node).count as libc::c_int +
                                             ENET_SUBCONTEXT_SYMBOL_DELTA as
                                                 libc::c_int) as enet_uint8;
                                    symbol = node;
                                    break ;
                                }
                            }
                        }
                        bottom =
                            symbol.wrapping_offset_from((*rangeCoder).symbols.as_mut_ptr())
                                as libc::c_long as EnetUint16;
                        decodeLow =
                            (decodeLow as
                                 libc::c_uint).wrapping_add((((*subcontext).escapes
                                                                  as
                                                                  libc::c_int
                                                                  +
                                                                  under as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_uint).wrapping_mul(decodeRange))
                                as EnetUint32 as EnetUint32;
                        decodeRange =
                            (decodeRange as
                                 libc::c_uint).wrapping_mul(count as
                                                                libc::c_uint)
                                as EnetUint32 as EnetUint32;
                        loop  {
                            if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                                   >=
                                   ENET_RANGE_CODER_TOP as libc::c_int as
                                       libc::c_uint {
                                if decodeRange >=
                                       ENET_RANGE_CODER_BOTTOM as libc::c_int
                                           as libc::c_uint {
                                    break ;
                                }
                                decodeRange =
                                    decodeLow.wrapping_neg() &
                                        (ENET_RANGE_CODER_BOTTOM as
                                             libc::c_int - 1i32) as
                                            libc::c_uint
                            }
                            decodeCode <<= 8i32;
                            if inData < inEnd {
                                let fresh19 = inData;
                                inData = inData.offset(1);
                                decodeCode |= *fresh19 as libc::c_uint
                            }
                            decodeRange <<= 8i32;
                            decodeLow <<= 8i32
                        }
                        (*subcontext).total =
                            ((*subcontext).total as libc::c_int +
                                 ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                                as EnetUint16;
                        if count as libc::c_int >
                               0xffi32 -
                                   2i32 *
                                       ENET_SUBCONTEXT_SYMBOL_DELTA as
                                           libc::c_int ||
                               (*subcontext).total as libc::c_int >
                                   ENET_RANGE_CODER_BOTTOM as libc::c_int -
                                       0x100i32 {
                            (*subcontext).total =
                                (if 0 != (*subcontext).symbols as libc::c_int
                                    {
                                     enet_symbol_rescale(subcontext.offset((*subcontext).symbols
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize))
                                         as libc::c_int
                                 } else { 0i32 }) as EnetUint16;
                            (*subcontext).escapes =
                                ((*subcontext).escapes as libc::c_int -
                                     ((*subcontext).escapes as libc::c_int >>
                                          1i32)) as EnetUint16;
                            (*subcontext).total =
                                ((*subcontext).total as libc::c_int +
                                     ((*subcontext).escapes as libc::c_int +
                                          256i32 * 0i32)) as EnetUint16
                        }
                        current_block_297 = 603970300416817108;
                        break ;
                    }
                }
            }
            subcontext =
                &mut *(*rangeCoder).symbols.as_mut_ptr().offset((*subcontext).parent
                                                                    as isize)
                    as *mut ENetSymbol
        }
        match current_block_297 {
            6950488995570599823 => {
                total = (*root).total;
                decodeRange =
                    (decodeRange as
                         libc::c_uint).wrapping_div(total as libc::c_uint) as
                        EnetUint32 as EnetUint32;
                code =
                    decodeCode.wrapping_sub(decodeLow).wrapping_div(decodeRange)
                        as EnetUint16;
                if (code as libc::c_int) < (*root).escapes as libc::c_int {
                    decodeLow =
                        (decodeLow as
                             libc::c_uint).wrapping_add((0i32 as
                                                             libc::c_uint).wrapping_mul(decodeRange))
                            as EnetUint32 as EnetUint32;
                    decodeRange =
                        (decodeRange as
                             libc::c_uint).wrapping_mul((*root).escapes as
                                                            libc::c_uint) as
                            EnetUint32 as EnetUint32;
                    loop  {
                        if decodeLow ^ decodeLow.wrapping_add(decodeRange) >=
                               ENET_RANGE_CODER_TOP as libc::c_int as
                                   libc::c_uint {
                            if decodeRange >=
                                   ENET_RANGE_CODER_BOTTOM as libc::c_int as
                                       libc::c_uint {
                                break ;
                            }
                            decodeRange =
                                decodeLow.wrapping_neg() &
                                    (ENET_RANGE_CODER_BOTTOM as libc::c_int -
                                         1i32) as libc::c_uint
                        }
                        decodeCode <<= 8i32;
                        if inData < inEnd {
                            let fresh20 = inData;
                            inData = inData.offset(1);
                            decodeCode |= *fresh20 as libc::c_uint
                        }
                        decodeRange <<= 8i32;
                        decodeLow <<= 8i32
                    }
                    break ;
                } else {
                    code =
                        (code as libc::c_int - (*root).escapes as libc::c_int)
                            as EnetUint16;
                    under = 0i32 as EnetUint16;
                    count =
                        ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int as
                            EnetUint16;
                    if 0 == (*root).symbols {
                        value =
                            (code as libc::c_int /
                                 ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                                as enet_uint8;
                        under =
                            (code as libc::c_int -
                                 code as libc::c_int %
                                     ENET_CONTEXT_SYMBOL_MINIMUM as
                                         libc::c_int) as EnetUint16;
                        let fresh21 = nextSymbol;
                        nextSymbol = nextSymbol.wrapping_add(1);
                        symbol =
                            &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh21
                                                                                as
                                                                                isize)
                                as *mut ENetSymbol;
                        (*symbol).value = value;
                        (*symbol).count =
                            ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as
                                enet_uint8;
                        (*symbol).under =
                            ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as
                                EnetUint16;
                        (*symbol).left = 0i32 as EnetUint16;
                        (*symbol).right = 0i32 as EnetUint16;
                        (*symbol).symbols = 0i32 as EnetUint16;
                        (*symbol).escapes = 0i32 as EnetUint16;
                        (*symbol).total = 0i32 as EnetUint16;
                        (*symbol).parent = 0i32 as EnetUint16;
                        (*root).symbols =
                            symbol.wrapping_offset_from(root) as libc::c_long
                                as EnetUint16
                    } else {
                        let mut node_0: *mut ENetSymbol =
                            root.offset((*root).symbols as libc::c_int as
                                            isize);
                        loop  {
                            let mut after_0: EnetUint16 =
                                (under as libc::c_int +
                                     (*node_0).under as libc::c_int +
                                     ((*node_0).value as libc::c_int + 1i32) *
                                         ENET_CONTEXT_SYMBOL_MINIMUM as
                                             libc::c_int) as EnetUint16;
                            let mut before_0: EnetUint16 =
                                ((*node_0).count as libc::c_int +
                                     ENET_CONTEXT_SYMBOL_MINIMUM as
                                         libc::c_int) as EnetUint16;
                            if code as libc::c_int >= after_0 as libc::c_int {
                                under =
                                    (under as libc::c_int +
                                         (*node_0).under as libc::c_int) as
                                        EnetUint16;
                                if 0 != (*node_0).right {
                                    node_0 =
                                        node_0.offset((*node_0).right as
                                                          libc::c_int as
                                                          isize)
                                } else {
                                    value =
                                        ((*node_0).value as libc::c_int + 1i32
                                             +
                                             (code as libc::c_int -
                                                  after_0 as libc::c_int) /
                                                 ENET_CONTEXT_SYMBOL_MINIMUM
                                                     as libc::c_int) as
                                            enet_uint8;
                                    under =
                                        (code as libc::c_int -
                                             (code as libc::c_int -
                                                  after_0 as libc::c_int) %
                                                 ENET_CONTEXT_SYMBOL_MINIMUM
                                                     as libc::c_int) as
                                            EnetUint16;
                                    let fresh22 = nextSymbol;
                                    nextSymbol = nextSymbol.wrapping_add(1);
                                    symbol =
                                        &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh22
                                                                                            as
                                                                                            isize)
                                            as *mut ENetSymbol;
                                    (*symbol).value = value;
                                    (*symbol).count =
                                        ENET_CONTEXT_SYMBOL_DELTA as
                                            libc::c_int as enet_uint8;
                                    (*symbol).under =
                                        ENET_CONTEXT_SYMBOL_DELTA as
                                            libc::c_int as EnetUint16;
                                    (*symbol).left = 0i32 as EnetUint16;
                                    (*symbol).right = 0i32 as EnetUint16;
                                    (*symbol).symbols = 0i32 as EnetUint16;
                                    (*symbol).escapes = 0i32 as EnetUint16;
                                    (*symbol).total = 0i32 as EnetUint16;
                                    (*symbol).parent = 0i32 as EnetUint16;
                                    (*node_0).right =
                                        symbol.wrapping_offset_from(node_0) as
                                            libc::c_long as EnetUint16;
                                    break ;
                                }
                            } else if (code as libc::c_int) <
                                          after_0 as libc::c_int -
                                              before_0 as libc::c_int {
                                (*node_0).under =
                                    ((*node_0).under as libc::c_int +
                                         ENET_CONTEXT_SYMBOL_DELTA as
                                             libc::c_int) as EnetUint16;
                                if 0 != (*node_0).left {
                                    node_0 =
                                        node_0.offset((*node_0).left as
                                                          libc::c_int as
                                                          isize)
                                } else {
                                    value =
                                        ((*node_0).value as libc::c_int - 1i32
                                             -
                                             (after_0 as libc::c_int -
                                                  before_0 as libc::c_int -
                                                  code as libc::c_int - 1i32)
                                                 /
                                                 ENET_CONTEXT_SYMBOL_MINIMUM
                                                     as libc::c_int) as
                                            enet_uint8;
                                    under =
                                        (code as libc::c_int -
                                             (after_0 as libc::c_int -
                                                  before_0 as libc::c_int -
                                                  code as libc::c_int - 1i32)
                                                 %
                                                 ENET_CONTEXT_SYMBOL_MINIMUM
                                                     as libc::c_int) as
                                            EnetUint16;
                                    let fresh23 = nextSymbol;
                                    nextSymbol = nextSymbol.wrapping_add(1);
                                    symbol =
                                        &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh23
                                                                                            as
                                                                                            isize)
                                            as *mut ENetSymbol;
                                    (*symbol).value = value;
                                    (*symbol).count =
                                        ENET_CONTEXT_SYMBOL_DELTA as
                                            libc::c_int as enet_uint8;
                                    (*symbol).under =
                                        ENET_CONTEXT_SYMBOL_DELTA as
                                            libc::c_int as EnetUint16;
                                    (*symbol).left = 0i32 as EnetUint16;
                                    (*symbol).right = 0i32 as EnetUint16;
                                    (*symbol).symbols = 0i32 as EnetUint16;
                                    (*symbol).escapes = 0i32 as EnetUint16;
                                    (*symbol).total = 0i32 as EnetUint16;
                                    (*symbol).parent = 0i32 as EnetUint16;
                                    (*node_0).left =
                                        symbol.wrapping_offset_from(node_0) as
                                            libc::c_long as EnetUint16;
                                    break ;
                                }
                            } else {
                                value = (*node_0).value;
                                count =
                                    (count as libc::c_int +
                                         (*node_0).count as libc::c_int) as
                                        EnetUint16;
                                under =
                                    (after_0 as libc::c_int -
                                         before_0 as libc::c_int) as
                                        EnetUint16;
                                (*node_0).under =
                                    ((*node_0).under as libc::c_int +
                                         ENET_CONTEXT_SYMBOL_DELTA as
                                             libc::c_int) as EnetUint16;
                                (*node_0).count =
                                    ((*node_0).count as libc::c_int +
                                         ENET_CONTEXT_SYMBOL_DELTA as
                                             libc::c_int) as enet_uint8;
                                symbol = node_0;
                                break ;
                            }
                        }
                    }
                    bottom =
                        symbol.wrapping_offset_from((*rangeCoder).symbols.as_mut_ptr())
                            as libc::c_long as EnetUint16;
                    decodeLow =
                        (decodeLow as
                             libc::c_uint).wrapping_add((((*root).escapes as
                                                              libc::c_int +
                                                              under as
                                                                  libc::c_int)
                                                             as
                                                             libc::c_uint).wrapping_mul(decodeRange))
                            as EnetUint32 as EnetUint32;
                    decodeRange =
                        (decodeRange as
                             libc::c_uint).wrapping_mul(count as libc::c_uint)
                            as EnetUint32 as EnetUint32;
                    loop  {
                        if decodeLow ^ decodeLow.wrapping_add(decodeRange) >=
                               ENET_RANGE_CODER_TOP as libc::c_int as
                                   libc::c_uint {
                            if decodeRange >=
                                   ENET_RANGE_CODER_BOTTOM as libc::c_int as
                                       libc::c_uint {
                                break ;
                            }
                            decodeRange =
                                decodeLow.wrapping_neg() &
                                    (ENET_RANGE_CODER_BOTTOM as libc::c_int -
                                         1i32) as libc::c_uint
                        }
                        decodeCode <<= 8i32;
                        if inData < inEnd {
                            let fresh24 = inData;
                            inData = inData.offset(1);
                            decodeCode |= *fresh24 as libc::c_uint
                        }
                        decodeRange <<= 8i32;
                        decodeLow <<= 8i32
                    }
                    (*root).total =
                        ((*root).total as libc::c_int +
                             ENET_CONTEXT_SYMBOL_DELTA as libc::c_int) as
                            EnetUint16;
                    if count as libc::c_int >
                           0xffi32 -
                               2i32 * ENET_CONTEXT_SYMBOL_DELTA as libc::c_int
                               + ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int ||
                           (*root).total as libc::c_int >
                               ENET_RANGE_CODER_BOTTOM as libc::c_int -
                                   0x100i32 {
                        (*root).total =
                            (if 0 != (*root).symbols as libc::c_int {
                                 enet_symbol_rescale(root.offset((*root).symbols
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     isize))
                                     as libc::c_int
                             } else { 0i32 }) as EnetUint16;
                        (*root).escapes =
                            ((*root).escapes as libc::c_int -
                                 ((*root).escapes as libc::c_int >> 1i32)) as
                                EnetUint16;
                        (*root).total =
                            ((*root).total as libc::c_int +
                                 ((*root).escapes as libc::c_int +
                                      256i32 *
                                          ENET_CONTEXT_SYMBOL_MINIMUM as
                                              libc::c_int)) as EnetUint16
                    }
                }
            }
            _ => { }
        }
        patch =
            &mut *(*rangeCoder).symbols.as_mut_ptr().offset(predicted as
                                                                isize) as
                *mut ENetSymbol;
        while patch != subcontext {
            under = (value as libc::c_int * 0i32) as EnetUint16;
            count = 0i32 as EnetUint16;
            if 0 == (*patch).symbols {
                let fresh25 = nextSymbol;
                nextSymbol = nextSymbol.wrapping_add(1);
                symbol =
                    &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh25 as
                                                                        isize)
                        as *mut ENetSymbol;
                (*symbol).value = value;
                (*symbol).count =
                    ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                (*symbol).under =
                    ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as
                        EnetUint16;
                (*symbol).left = 0i32 as EnetUint16;
                (*symbol).right = 0i32 as EnetUint16;
                (*symbol).symbols = 0i32 as EnetUint16;
                (*symbol).escapes = 0i32 as EnetUint16;
                (*symbol).total = 0i32 as EnetUint16;
                (*symbol).parent = 0i32 as EnetUint16;
                (*patch).symbols =
                    symbol.wrapping_offset_from(patch) as libc::c_long as
                        EnetUint16
            } else {
                let mut node_1: *mut ENetSymbol =
                    patch.offset((*patch).symbols as libc::c_int as isize);
                loop  {
                    if (value as libc::c_int) < (*node_1).value as libc::c_int
                       {
                        (*node_1).under =
                            ((*node_1).under as libc::c_int +
                                 ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                                as EnetUint16;
                        if 0 != (*node_1).left {
                            node_1 =
                                node_1.offset((*node_1).left as libc::c_int as
                                                  isize)
                        } else {
                            let fresh26 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol =
                                &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh26
                                                                                    as
                                                                                    isize)
                                    as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as
                                    enet_uint8;
                            (*symbol).under =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as
                                    EnetUint16;
                            (*symbol).left = 0i32 as EnetUint16;
                            (*symbol).right = 0i32 as EnetUint16;
                            (*symbol).symbols = 0i32 as EnetUint16;
                            (*symbol).escapes = 0i32 as EnetUint16;
                            (*symbol).total = 0i32 as EnetUint16;
                            (*symbol).parent = 0i32 as EnetUint16;
                            (*node_1).left =
                                symbol.wrapping_offset_from(node_1) as
                                    libc::c_long as EnetUint16;
                            break ;
                        }
                    } else if value as libc::c_int >
                                  (*node_1).value as libc::c_int {
                        under =
                            (under as libc::c_int +
                                 (*node_1).under as libc::c_int) as
                                EnetUint16;
                        if 0 != (*node_1).right {
                            node_1 =
                                node_1.offset((*node_1).right as libc::c_int
                                                  as isize)
                        } else {
                            let fresh27 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol =
                                &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh27
                                                                                    as
                                                                                    isize)
                                    as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as
                                    enet_uint8;
                            (*symbol).under =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as
                                    EnetUint16;
                            (*symbol).left = 0i32 as EnetUint16;
                            (*symbol).right = 0i32 as EnetUint16;
                            (*symbol).symbols = 0i32 as EnetUint16;
                            (*symbol).escapes = 0i32 as EnetUint16;
                            (*symbol).total = 0i32 as EnetUint16;
                            (*symbol).parent = 0i32 as EnetUint16;
                            (*node_1).right =
                                symbol.wrapping_offset_from(node_1) as
                                    libc::c_long as EnetUint16;
                            break ;
                        }
                    } else {
                        count =
                            (count as libc::c_int +
                                 (*node_1).count as libc::c_int) as
                                EnetUint16;
                        under =
                            (under as libc::c_int +
                                 ((*node_1).under as libc::c_int -
                                      (*node_1).count as libc::c_int)) as
                                EnetUint16;
                        (*node_1).under =
                            ((*node_1).under as libc::c_int +
                                 ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                                as EnetUint16;
                        (*node_1).count =
                            ((*node_1).count as libc::c_int +
                                 ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                                as enet_uint8;
                        symbol = node_1;
                        break ;
                    }
                }
            }
            *parent =
                symbol.wrapping_offset_from((*rangeCoder).symbols.as_mut_ptr())
                    as libc::c_long as EnetUint16;
            parent = &mut (*symbol).parent;
            if count as libc::c_int <= 0i32 {
                (*patch).escapes =
                    ((*patch).escapes as libc::c_int +
                         ENET_SUBCONTEXT_ESCAPE_DELTA as libc::c_int) as
                        EnetUint16;
                (*patch).total =
                    ((*patch).total as libc::c_int +
                         ENET_SUBCONTEXT_ESCAPE_DELTA as libc::c_int) as
                        EnetUint16
            }
            (*patch).total =
                ((*patch).total as libc::c_int +
                     ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int) as
                    EnetUint16;
            if count as libc::c_int >
                   0xffi32 -
                       2i32 * ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int ||
                   (*patch).total as libc::c_int >
                       ENET_RANGE_CODER_BOTTOM as libc::c_int - 0x100i32 {
                (*patch).total =
                    (if 0 != (*patch).symbols as libc::c_int {
                         enet_symbol_rescale(patch.offset((*patch).symbols as
                                                              libc::c_int as
                                                              isize)) as
                             libc::c_int
                     } else { 0i32 }) as EnetUint16;
                (*patch).escapes =
                    ((*patch).escapes as libc::c_int -
                         ((*patch).escapes as libc::c_int >> 1i32)) as
                        EnetUint16;
                (*patch).total =
                    ((*patch).total as libc::c_int +
                         ((*patch).escapes as libc::c_int + 256i32 * 0i32)) as
                        EnetUint16
            }
            patch =
                &mut *(*rangeCoder).symbols.as_mut_ptr().offset((*patch).parent
                                                                    as isize)
                    as *mut ENetSymbol
        }
        *parent = bottom;
        if outData >= outEnd { return 0i32 as SizeT }
        let fresh28 = outData;
        outData = outData.offset(1);
        *fresh28 = value;
        if order >= ENET_SUBCONTEXT_ORDER as libc::c_int as libc::c_ulong {
            predicted = (*rangeCoder).symbols[predicted as usize].parent
        } else { order = order.wrapping_add(1) }
        if nextSymbol >=
               (::std::mem::size_of::<[ENetSymbol; 4096]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<ENetSymbol>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(ENET_SUBCONTEXT_ORDER
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
           {
            nextSymbol = 0i32 as SizeT;
            let fresh29 = nextSymbol;
            nextSymbol = nextSymbol.wrapping_add(1);
            root =
                &mut *(*rangeCoder).symbols.as_mut_ptr().offset(fresh29 as
                                                                    isize) as
                    *mut ENetSymbol;
            (*root).value = 0i32 as enet_uint8;
            (*root).count = 0i32 as enet_uint8;
            (*root).under = 0i32 as EnetUint16;
            (*root).left = 0i32 as EnetUint16;
            (*root).right = 0i32 as EnetUint16;
            (*root).symbols = 0i32 as EnetUint16;
            (*root).escapes = 0i32 as EnetUint16;
            (*root).total = 0i32 as EnetUint16;
            (*root).parent = 0i32 as EnetUint16;
            (*root).escapes =
                ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int as EnetUint16;
            (*root).total =
                (ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int +
                     256i32 * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int) as
                    EnetUint16;
            (*root).symbols = 0i32 as EnetUint16;
            predicted = 0i32 as EnetUint16;
            order = 0i32 as SizeT
        }
    }
    return outData.wrapping_offset_from(outStart) as libc::c_long as SizeT;
}
/* * @defgroup host ENet host functions
    @{
*/
/* * Sets the packet compressor the host should use to the default range coder.
    @param host host to enable the range coder for
    @returns 0 on success, < 0 on failure
*/
#[no_mangle]
pub unsafe extern "C" fn enet_host_compress_with_range_coder(mut host:
                                                                 *mut ENetHost)
 -> libc::c_int {
    let mut compressor: ENetCompressor =
        ENetCompressor{context: 0 as *mut libc::c_void,
                       compress: None,
                       decompress: None,
                       destroy: None,};
    memset(&mut compressor as *mut ENetCompressor as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ENetCompressor>() as libc::c_ulong);
    compressor.context = enet_range_coder_create();
    if compressor.context.is_null() { return -1i32 }
    compressor.compress =
        Some(enet_range_coder_compress as
                 unsafe extern "C" fn(_: *mut libc::c_void,
                                      _: *const ENetBuffer, _: SizeT,
                                      _: SizeT, _: *mut enet_uint8,
                                      _: SizeT) -> SizeT);
    compressor.decompress =
        Some(enet_range_coder_decompress as
                 unsafe extern "C" fn(_: *mut libc::c_void,
                                      _: *const enet_uint8, _: SizeT,
                                      _: *mut enet_uint8, _: SizeT)
                     -> SizeT);
    compressor.destroy =
        Some(enet_range_coder_destroy as
                 unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    enet_host_compress(host, &mut compressor);
    return 0i32;
}