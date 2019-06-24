use libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn htonl(__hostlong: Uint32T) -> Uint32T;
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
pub type __Uint32T = libc::c_uint;
pub type Uint32T = __Uint32T;
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
/* *< unsigned 32-bit type */
pub type EnetUint32 = libc::c_uint;
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
/* * @} */
/* * 
 @file  packet.c
 @brief ENet packet management functions
*/
/* * @defgroup Packet ENet packet functions 
    @{ 
*/
/* * Creates a packet that may be sent to a peer.
    @param data         initial contents of the packet's data; the packet's data will remain uninitialized if data is NULL.
    @param data_length   size of the data allocated for this packet
    @param flags        flags for this packet as described for the ENetPacket structure.
    @returns the packet on success, NULL on failure
*/
#[no_mangle]
pub unsafe extern "C" fn enet_packet_create(mut data: *const libc::c_void,
                                            mut data_length: SizeT,
                                            mut flags: EnetUint32)
 -> *mut ENetPacket {
    let mut packet: *mut ENetPacket =
        enet_malloc(::std::mem::size_of::<ENetPacket>() as libc::c_ulong) as
            *mut ENetPacket;
    if packet.is_null() { return 0 as *mut ENetPacket }
    if 0 !=
           flags & ENET_PACKET_FLAG_NO_ALLOCATE as libc::c_int as libc::c_uint
       {
        (*packet).data = data as *mut enet_uint8
    } else if data_length <= 0i32 as libc::c_ulong {
        (*packet).data = 0 as *mut enet_uint8
    } else {
        (*packet).data = enet_malloc(data_length) as *mut enet_uint8;
        if (*packet).data.is_null() {
            enet_free(packet as *mut libc::c_void);
            return 0 as *mut ENetPacket
        }
        if data != 0 as *mut libc::c_void {
            memcpy((*packet).data as *mut libc::c_void, data, data_length);
        }
    }
    (*packet).referenceCount = 0i32 as SizeT;
    (*packet).flags = flags;
    (*packet).data_length = data_length;
    (*packet).freeCallback = None;
    (*packet).userData = 0 as *mut libc::c_void;
    return packet;
}
/* * Destroys the packet and deallocates its data.
    @param packet packet to be destroyed
*/
#[no_mangle]
pub unsafe extern "C" fn enet_packet_destroy(mut packet: *mut ENetPacket) {
    if packet.is_null() { return }
    if (*packet).freeCallback.is_some() {
        (*packet).freeCallback.expect("non-null function pointer")(packet);
    }
    if 0 ==
           (*packet).flags &
               ENET_PACKET_FLAG_NO_ALLOCATE as libc::c_int as libc::c_uint &&
           !(*packet).data.is_null() {
        enet_free((*packet).data as *mut libc::c_void);
    }
    enet_free(packet as *mut libc::c_void);
}
/* * Attempts to resize the data in the packet to length specified in the 
    data_length parameter 
    @param packet packet to resize
    @param data_length new size for the packet data
    @returns 0 on success, < 0 on failure
*/
#[no_mangle]
pub unsafe extern "C" fn enet_packet_resize(mut packet: *mut ENetPacket,
                                            mut data_length: SizeT)
 -> libc::c_int {
    let mut newData: *mut enet_uint8 = 0 as *mut enet_uint8;
    if data_length <= (*packet).data_length ||
           0 !=
               (*packet).flags &
                   ENET_PACKET_FLAG_NO_ALLOCATE as libc::c_int as libc::c_uint
       {
        (*packet).data_length = data_length;
        return 0i32
    }
    newData = enet_malloc(data_length) as *mut enet_uint8;
    if newData.is_null() { return -1i32 }
    memcpy(newData as *mut libc::c_void,
           (*packet).data as *const libc::c_void, (*packet).data_length);
    enet_free((*packet).data as *mut libc::c_void);
    (*packet).data = newData;
    (*packet).data_length = data_length;
    return 0i32;
}
static mut initializedCRC32: libc::c_int = 0i32;
static mut crcTable: [EnetUint32; 256] = [0; 256];
unsafe extern "C" fn reflect_crc(mut val: libc::c_int, mut bits: libc::c_int)
 -> EnetUint32 {
    let mut result: libc::c_int = 0i32;
    let mut bit: libc::c_int = 0;
    bit = 0i32;
    while bit < bits {
        if 0 != val & 1i32 { result |= 1i32 << bits - 1i32 - bit }
        val >>= 1i32;
        bit += 1
    }
    return result as EnetUint32;
}
unsafe extern "C" fn initialize_crc32() {
    let mut byte: libc::c_int = 0;
    byte = 0i32;
    while byte < 256i32 {
        let mut crc: EnetUint32 = reflect_crc(byte, 8i32) << 24i32;
        let mut offset: libc::c_int = 0;
        offset = 0i32;
        while offset < 8i32 {
            if 0 != crc & 0x80000000u32 {
                crc = crc << 1i32 ^ 0x4c11db7i32 as libc::c_uint
            } else { crc <<= 1i32 }
            offset += 1
        }
        crcTable[byte as usize] = reflect_crc(crc as libc::c_int, 32i32);
        byte += 1
    }
    initializedCRC32 = 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn enet_crc32(mut buffers: *const ENetBuffer,
                                    mut buffer_count: SizeT) -> EnetUint32 {
    let mut crc: EnetUint32 = 0xffffffffu32;
    if 0 == initializedCRC32 { initialize_crc32(); }
    loop  {
        let fresh0 = buffer_count;
        buffer_count = buffer_count.wrapping_sub(1);
        if !(fresh0 > 0i32 as libc::c_ulong) { break ; }
        let mut data: *const enet_uint8 =
            (*buffers).data as *const enet_uint8;
        let mut dataEnd: *const enet_uint8 =
            &*data.offset((*buffers).data_length as isize) as
                *const enet_uint8;
        while data < dataEnd {
            let fresh1 = data;
            data = data.offset(1);
            crc =
                crc >> 8i32 ^
                    crcTable[(crc & 0xffi32 as libc::c_uint ^
                                  *fresh1 as libc::c_uint) as usize]
        }
        buffers = buffers.offset(1isize)
    }
    return htonl(!crc);
}