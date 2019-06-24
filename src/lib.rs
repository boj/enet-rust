extern crate libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
              __writefds: *mut fd_set, __exceptfds: *mut fd_set,
              __timeout: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn getaddrinfo(__name: *const libc::c_char,
                   __service: *const libc::c_char, __req: *const addrinfo,
                   __pai: *mut *mut addrinfo) -> libc::c_int;
    #[no_mangle]
    fn freeaddrinfo(__ai: *mut addrinfo);
    #[no_mangle]
    fn getnameinfo(__sa: *const sockaddr, __salen: SockLenT,
                   __host: *mut libc::c_char, __hostlen: SockLenT,
                   __serv: *mut libc::c_char, __servlen: SockLenT,
                   __flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: SockLenT)
     -> libc::c_int;
    #[no_mangle]
    fn getsockname(__fd: libc::c_int, __addr: *mut sockaddr,
                   __len: *mut SockLenT) -> libc::c_int;
    #[no_mangle]
    fn connect(__fd: libc::c_int, __addr: *const sockaddr, __len: SockLenT)
     -> libc::c_int;
    #[no_mangle]
    fn sendmsg(__fd: libc::c_int, __message: *const msghdr,
               __flags: libc::c_int) -> sSizeT;
    #[no_mangle]
    fn recvmsg(__fd: libc::c_int, __message: *mut msghdr,
               __flags: libc::c_int) -> sSizeT;
    #[no_mangle]
    fn getsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *mut libc::c_void,
                  __optlen: *mut SockLenT) -> libc::c_int;
    #[no_mangle]
    fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *const libc::c_void,
                  __optlen: SockLenT) -> libc::c_int;
    #[no_mangle]
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn accept(__fd: libc::c_int, __addr: *mut sockaddr,
              __addr_len: *mut SockLenT) -> libc::c_int;
    #[no_mangle]
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: TimezonePtr16T)
     -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn poll(__fds: *mut pollfd, __nfds: NfdsT, __timeout: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut TimeT) -> TimeT;
    #[no_mangle]
    fn inet_pton(__af: libc::c_int, __cp: *const libc::c_char,
                 __buf: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn inet_ntop(__af: libc::c_int, __cp: *const libc::c_void,
                 __buf: *mut libc::c_char, __len: SockLenT)
     -> *const libc::c_char;
    #[no_mangle]
    fn ntohs(__netshort: Uint16T) -> Uint16T;
    #[no_mangle]
    fn htons(__hostshort: Uint16T) -> Uint16T;
}
pub type __Uint16T = libc::c_ushort;
pub type __Uint32T = libc::c_uint;
pub type __TimeT = libc::c_long;
pub type SusecondsT = libc::c_long;
pub type SSizeT = libc::c_long;
pub type __SockLenT = libc::c_uint;
pub type sSizeT = SSizeT;
pub type TimeT = __TimeT;
pub type SizeT = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __TimeT,
    pub tv_usec: SusecondsT,
}
pub type FdMask = libc::c_long;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [FdMask; 16],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: SizeT,
}
pub type SockLenT = __SockLenT;
pub type SocketType = libc::c_uint;
pub const SOCK_NONBLOCK: SocketType = 2048;
pub const SOCK_CLOEXEC: SocketType = 524288;
pub const SOCK_PACKET: SocketType = 10;
pub const SOCK_DCCP: SocketType = 6;
pub const SOCK_SEQPACKET: SocketType = 5;
pub const SOCK_RDM: SocketType = 4;
pub const SOCK_RAW: SocketType = 3;
pub const SOCK_DGRAM: SocketType = 2;
pub const SOCK_STREAM: SocketType = 1;
pub type SaFamilyT = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: SaFamilyT,
    pub sa_data: [libc::c_char; 14],
}
pub type Unnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: Unnamed = 1073741824;
pub const MSG_FASTOPEN: Unnamed = 536870912;
pub const MSG_ZEROCOPY: Unnamed = 67108864;
pub const MSG_BATCH: Unnamed = 262144;
pub const MSG_WAITFORONE: Unnamed = 65536;
pub const MSG_MORE: Unnamed = 32768;
pub const MSG_NOSIGNAL: Unnamed = 16384;
pub const MSG_ERRQUEUE: Unnamed = 8192;
pub const MSG_RST: Unnamed = 4096;
pub const MSG_CONFIRM: Unnamed = 2048;
pub const MSG_SYN: Unnamed = 1024;
pub const MSG_FIN: Unnamed = 512;
pub const MSG_WAITALL: Unnamed = 256;
pub const MSG_EOR: Unnamed = 128;
pub const MSG_DONTWAIT: Unnamed = 64;
pub const MSG_TRUNC: Unnamed = 32;
pub const MSG_PROXY: Unnamed = 16;
pub const MSG_CTRUNC: Unnamed = 8;
pub const MSG_DONTROUTE: Unnamed = 4;
pub const MSG_PEEK: Unnamed = 2;
pub const MSG_OOB: Unnamed = 1;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: SockLenT,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: SizeT,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: SizeT,
    pub msg_flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type TimezonePtr16T = *mut timezone;
pub type Uint16T = __Uint16T;
pub type Uint32T = __Uint32T;
pub type InAddrT = Uint32T;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct in_addr {
    pub s_addr: InAddrT,
}
pub type Unnamed0 = libc::c_uint;
pub const IPPROTO_MAX: Unnamed0 = 256;
pub const IPPROTO_RAW: Unnamed0 = 255;
pub const IPPROTO_MPLS: Unnamed0 = 137;
pub const IPPROTO_UDPLITE: Unnamed0 = 136;
pub const IPPROTO_SCTP: Unnamed0 = 132;
pub const IPPROTO_COMP: Unnamed0 = 108;
pub const IPPROTO_PIM: Unnamed0 = 103;
pub const IPPROTO_ENCAP: Unnamed0 = 98;
pub const IPPROTO_BEETPH: Unnamed0 = 94;
pub const IPPROTO_MTP: Unnamed0 = 92;
pub const IPPROTO_AH: Unnamed0 = 51;
pub const IPPROTO_ESP: Unnamed0 = 50;
pub const IPPROTO_GRE: Unnamed0 = 47;
pub const IPPROTO_RSVP: Unnamed0 = 46;
pub const IPPROTO_IPV6: Unnamed0 = 41;
pub const IPPROTO_DCCP: Unnamed0 = 33;
pub const IPPROTO_TP: Unnamed0 = 29;
pub const IPPROTO_IDP: Unnamed0 = 22;
pub const IPPROTO_UDP: Unnamed0 = 17;
pub const IPPROTO_PUP: Unnamed0 = 12;
pub const IPPROTO_EGP: Unnamed0 = 8;
pub const IPPROTO_TCP: Unnamed0 = 6;
pub const IPPROTO_IPIP: Unnamed0 = 4;
pub const IPPROTO_IGMP: Unnamed0 = 2;
pub const IPPROTO_ICMP: Unnamed0 = 1;
pub const IPPROTO_IP: Unnamed0 = 0;
pub type InPortT = Uint16T;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: SaFamilyT,
    pub sin_port: InPortT,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: SockLenT,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
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
pub type ENetSocketSet = fd_set;
/* *< unsigned 16-bit type */
pub type EnetUint16 = libc::c_ushort;
/* *< unsigned 32-bit type */
pub type EnetUint32 = libc::c_uint;
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
pub type _ENetSocketType = libc::c_uint;
pub const ENET_SOCKET_TYPE_DATAGRAM: _ENetSocketType = 2;
pub const ENET_SOCKET_TYPE_STREAM: _ENetSocketType = 1;
pub type ENetSocketType = _ENetSocketType;
pub type _ENetSocketWait = libc::c_uint;
pub const ENET_SOCKET_WAIT_INTERRUPT: _ENetSocketWait = 4;
pub const ENET_SOCKET_WAIT_RECEIVE: _ENetSocketWait = 2;
pub const ENET_SOCKET_WAIT_SEND: _ENetSocketWait = 1;
pub const ENET_SOCKET_WAIT_NONE: _ENetSocketWait = 0;
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
pub type _ENetSocketShutdown = libc::c_uint;
pub const ENET_SOCKET_SHUTDOWN_READ_WRITE: _ENetSocketShutdown = 2;
pub const ENET_SOCKET_SHUTDOWN_WRITE: _ENetSocketShutdown = 1;
pub const ENET_SOCKET_SHUTDOWN_READ: _ENetSocketShutdown = 0;
pub type ENetSocketShutdown = _ENetSocketShutdown;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type NfdsT = libc::c_ulong;
/* * 
 @file  unix.c
 @brief ENet Unix system specific functions
*/
static mut TIME_BASE: EnetUint32 = 0i32 as EnetUint32;
/* * @defgroup global ENet global functions
    @{ 
*/
/* * 
  Initializes ENet globally.  Must be called prior to using any functions in
  ENet.
  @returns 0 on success, < 0 on failure
*/
#[no_mangle]
pub unsafe extern "C" fn enet_initialize() -> libc::c_int { return 0i32; }
/* * 
  Shuts down ENet globally.  Should be called when a program that has
  initialized ENet exits.
*/
#[no_mangle]
pub unsafe extern "C" fn enet_deinitialize() { }
#[no_mangle]
pub unsafe extern "C" fn enet_host_random_seed() -> EnetUint32 {
    return time(0 as *mut TimeT) as EnetUint32;
}
/* * @} */
/* * @defgroup private ENet private implementation functions */
/* *
  Returns the wall-time in milliseconds.  Its initial value is unspecified
  unless otherwise set.
  */
#[no_mangle]
pub unsafe extern "C" fn enet_time_get() -> EnetUint32 {
    let mut time_val: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    gettimeofday(&mut time_val, 0 as *mut timezone);
    return (time_val.tv_sec * 1000i32 as libc::c_long +
                time_val.tv_usec / 1000i32 as libc::c_long -
                TIME_BASE as libc::c_long) as EnetUint32;
}
/* *
  Sets the current wall-time in milliseconds.
  */
#[no_mangle]
pub unsafe extern "C" fn enet_time_set(new_time_base: EnetUint32) {
    let mut time_val: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    gettimeofday(&mut time_val, 0 as *mut timezone);
    TIME_BASE =
        (time_val.tv_sec * 1000i32 as libc::c_long +
             time_val.tv_usec / 1000i32 as libc::c_long -
             new_time_base as libc::c_long) as EnetUint32;
}
/* * @} */
/* * @defgroup Address ENet address functions
    @{
*/
/* * Attempts to parse the printable form of the IP address in the parameter hostName
    and sets the host field in the address parameter if successful.
    @param address destination to store the parsed IP address
    @param hostName IP address to parse
    @retval 0 on success
    @retval < 0 on failure
    @returns the address of the given hostName in address on success
*/
#[no_mangle]
pub unsafe extern "C" fn enet_address_set_host_ip(address:
                                                      *mut ENetAddress,
                                                  name:
                                                      *const libc::c_char)
 -> libc::c_int {
    if 0 ==
           inet_pton(2i32, name,
                     &mut (*address).host as *mut EnetUint32 as
                         *mut libc::c_void) {
        return -1i32
    }
    return 0i32;
}
/* * Attempts to resolve the host named by the parameter hostName and sets
    the host field in the address parameter if successful.
    @param address destination to store resolved address
    @param hostName host name to lookup
    @retval 0 on success
    @retval < 0 on failure
    @returns the address of the given hostName in address on success
*/
#[no_mangle]
pub unsafe extern "C" fn enet_address_set_host(address: *mut ENetAddress,
                                               name: *const libc::c_char)
 -> libc::c_int {
    let mut hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut result_list: *mut addrinfo = 0 as *mut addrinfo;
    let mut result: *mut addrinfo = 0 as *mut addrinfo;
    memset(&mut hints as *mut addrinfo as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hints.ai_family = 2i32;
    if getaddrinfo(name, 0 as *const libc::c_char, 0 as *const addrinfo,
                   &mut result_list) != 0i32 {
        return -1i32
    }
    result = result_list;
    while !result.is_null() {
        if (*result).ai_family == 2i32 && !(*result).ai_addr.is_null() &&
               (*result).ai_addrlen as libc::c_ulong >=
                   ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong {
            let sin: *mut sockaddr_in =
                (*result).ai_addr as *mut sockaddr_in;
            (*address).host = (*sin).sin_addr.s_addr;
            freeaddrinfo(result_list);
            return 0i32
        }
        result = (*result).ai_next
    }
    if !result_list.is_null() { freeaddrinfo(result_list); }
    return enet_address_set_host_ip(address, name);
}
/* * Gives the printable form of the IP address specified in the address parameter.
    @param address    address printed
    @param hostName   destination for name, must not be NULL
    @param name_length maximum length of hostName.
    @returns the null-terminated name of the host in hostName on success
    @retval 0 on success
    @retval < 0 on failure
*/
#[no_mangle]
pub unsafe extern "C" fn enet_address_get_host_ip(address:
                                                      *const ENetAddress,
                                                  name: *mut libc::c_char,
                                                  name_length: SizeT)
 -> libc::c_int {
    if inet_ntop(2i32,
                 &(*address).host as *const EnetUint32 as
                     *const libc::c_void, name,
                 name_length as SockLenT).is_null() {
        return -1i32
    }
    return 0i32;
}
/* * Attempts to do a reverse lookup of the host field in the address parameter.
    @param address    address used for reverse lookup
    @param hostName   destination for name, must not be NULL
    @param name_length maximum length of hostName.
    @returns the null-terminated name of the host in hostName on success
    @retval 0 on success
    @retval < 0 on failure
*/
#[no_mangle]
pub unsafe extern "C" fn enet_address_get_host(address:
                                                   *const ENetAddress,
                                               name: *mut libc::c_char,
                                               name_length: SizeT)
 -> libc::c_int {
    let mut sin: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut err: libc::c_int = 0;
    memset(&mut sin as *mut sockaddr_in as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
    sin.sin_family = 2i32 as SaFamilyT;
    sin.sin_port = htons((*address).port);
    sin.sin_addr.s_addr = (*address).host;
    err =
        getnameinfo(&mut sin as *mut sockaddr_in as *mut sockaddr,
                    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                        SockLenT, name, name_length as SockLenT,
                    0 as *mut libc::c_char, 0i32 as SockLenT, 8i32);
    if 0 == err {
        if !name.is_null() && name_length > 0i32 as libc::c_ulong &&
               memchr(name as *const libc::c_void, '\u{0}' as i32,
                      name_length).is_null() {
            return -1i32
        }
        return 0i32
    }
    if err != -2i32 { return -1i32 }
    return enet_address_get_host_ip(address, name, name_length);
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_bind(socket_0: ENetSocket,
                                          address: *const ENetAddress)
 -> libc::c_int {
    let mut sin: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    memset(&mut sin as *mut sockaddr_in as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
    sin.sin_family = 2i32 as SaFamilyT;
    if !address.is_null() {
        sin.sin_port = htons((*address).port);
        sin.sin_addr.s_addr = (*address).host
    } else {
        sin.sin_port = 0i32 as InPortT;
        sin.sin_addr.s_addr = 0i32 as InAddrT
    }
    return bind(socket_0, &mut sin as *mut sockaddr_in as *mut sockaddr,
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                    SockLenT);
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_get_address(socket_0: ENetSocket,
                                                 address:
                                                     *mut ENetAddress)
 -> libc::c_int {
    let mut sin: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut sin_length: SockLenT =
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as SockLenT;
    if getsockname(socket_0, &mut sin as *mut sockaddr_in as *mut sockaddr,
                   &mut sin_length) == -1i32 {
        return -1i32
    }
    (*address).host = sin.sin_addr.s_addr;
    (*address).port = ntohs(sin.sin_port);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_listen(socket_0: ENetSocket,
                                            backlog: libc::c_int)
 -> libc::c_int {
    return listen(socket_0, if backlog < 0i32 { 128i32 } else { backlog });
}
/* * @defgroup socket ENet socket functions
    @{
*/
#[no_mangle]
pub unsafe extern "C" fn enet_socket_create(type_0: ENetSocketType)
 -> ENetSocket {
    return socket(2i32,
                  if type_0 as libc::c_uint ==
                         ENET_SOCKET_TYPE_DATAGRAM as libc::c_int as
                             libc::c_uint {
                      SOCK_DGRAM as libc::c_int
                  } else { SOCK_STREAM as libc::c_int }, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_set_option(socket_0: ENetSocket,
                                                option: ENetSocketOption,
                                                mut value: libc::c_int)
 -> libc::c_int {
    let mut result: libc::c_int = -1i32;
    match option as libc::c_uint {
        1 => {
            result =
                fcntl(socket_0, 4i32,
                      (if 0 != value { 0o4000i32 } else { 0i32 }) |
                          fcntl(socket_0, 3i32, 0) & !0o4000i32)
        }
        2 => {
            result =
                setsockopt(socket_0, 1i32, 6i32,
                           &mut value as *mut libc::c_int as *mut libc::c_char
                               as *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as SockLenT)
        }
        5 => {
            result =
                setsockopt(socket_0, 1i32, 2i32,
                           &mut value as *mut libc::c_int as *mut libc::c_char
                               as *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as SockLenT)
        }
        3 => {
            result =
                setsockopt(socket_0, 1i32, 8i32,
                           &mut value as *mut libc::c_int as *mut libc::c_char
                               as *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as SockLenT)
        }
        4 => {
            result =
                setsockopt(socket_0, 1i32, 7i32,
                           &mut value as *mut libc::c_int as *mut libc::c_char
                               as *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as SockLenT)
        }
        6 => {
            let mut time_val: timeval = timeval{tv_sec: 0, tv_usec: 0,};
            time_val.tv_sec = (value / 1000i32) as __TimeT;
            time_val.tv_usec = (value % 1000i32 * 1000i32) as SusecondsT;
            result =
                setsockopt(socket_0, 1i32, 20i32,
                           &mut time_val as *mut timeval as *mut libc::c_char
                               as *const libc::c_void,
                           ::std::mem::size_of::<timeval>() as libc::c_ulong
                               as SockLenT)
        }
        7 => {
            let mut time_val_0: timeval = timeval{tv_sec: 0, tv_usec: 0,};
            time_val_0.tv_sec = (value / 1000i32) as __TimeT;
            time_val_0.tv_usec = (value % 1000i32 * 1000i32) as SusecondsT;
            result =
                setsockopt(socket_0, 1i32, 21i32,
                           &mut time_val_0 as *mut timeval as *mut libc::c_char
                               as *const libc::c_void,
                           ::std::mem::size_of::<timeval>() as libc::c_ulong
                               as SockLenT)
        }
        9 => {
            result =
                setsockopt(socket_0, IPPROTO_TCP as libc::c_int, 1i32,
                           &mut value as *mut libc::c_int as *mut libc::c_char
                               as *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as SockLenT)
        }
        _ => { }
    }
    return if result == -1i32 { -1i32 } else { 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_get_option(socket_0: ENetSocket,
                                                option: ENetSocketOption,
                                                value: *mut libc::c_int)
 -> libc::c_int {
    let mut result: libc::c_int = -1i32;
    let mut len: SockLenT = 0;
    match option as libc::c_uint {
        8 => {
            len =
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                    SockLenT;
            result =
                getsockopt(socket_0, 1i32, 4i32, value as *mut libc::c_void,
                           &mut len)
        }
        _ => { }
    }
    return if result == -1i32 { -1i32 } else { 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_connect(socket_0: ENetSocket,
                                             address: *const ENetAddress)
 -> libc::c_int {
    let mut sin: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut result: libc::c_int = 0;
    memset(&mut sin as *mut sockaddr_in as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
    sin.sin_family = 2i32 as SaFamilyT;
    sin.sin_port = htons((*address).port);
    sin.sin_addr.s_addr = (*address).host;
    result =
        connect(socket_0, &mut sin as *mut sockaddr_in as *mut sockaddr,
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                    SockLenT);
    if result == -1i32 && *__errno_location() == 115i32 { return 0i32 }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_accept(socket_0: ENetSocket,
                                            address: *mut ENetAddress)
 -> ENetSocket {
    let mut result: libc::c_int = 0;
    let mut sin: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut sin_length: SockLenT =
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as SockLenT;
    result =
        accept(socket_0,
               if !address.is_null() {
                   &mut sin as *mut sockaddr_in as *mut sockaddr
               } else { 0 as *mut sockaddr },
               if !address.is_null() {
                   &mut sin_length
               } else { 0 as *mut SockLenT });
    if result == -1i32 { return -1i32 }
    if !address.is_null() {
        (*address).host = sin.sin_addr.s_addr;
        (*address).port = ntohs(sin.sin_port)
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_shutdown(socket_0: ENetSocket,
                                              how: ENetSocketShutdown)
 -> libc::c_int {
    return shutdown(socket_0, how as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_destroy(socket_0: ENetSocket) {
    if socket_0 != -1i32 { close(socket_0); };
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_send(socket_0: ENetSocket,
                                          address: *const ENetAddress,
                                          buffers: *const ENetBuffer,
                                          buffer_count: SizeT)
 -> libc::c_int {
    let mut msg_hdr: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut sin: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut sent_length: libc::c_int = 0;
    memset(&mut msg_hdr as *mut msghdr as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<msghdr>() as libc::c_ulong);
    if !address.is_null() {
        memset(&mut sin as *mut sockaddr_in as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
        sin.sin_family = 2i32 as SaFamilyT;
        sin.sin_port = htons((*address).port);
        sin.sin_addr.s_addr = (*address).host;
        msg_hdr.msg_name = &mut sin as *mut sockaddr_in as *mut libc::c_void;
        msg_hdr.msg_namelen =
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as SockLenT
    }
    msg_hdr.msg_iov = buffers as *mut iovec;
    msg_hdr.msg_iovlen = buffer_count;
    sent_length =
        sendmsg(socket_0, &mut msg_hdr, MSG_NOSIGNAL as libc::c_int) as
            libc::c_int;
    if sent_length == -1i32 {
        if *__errno_location() == 11i32 { return 0i32 }
        return -1i32
    }
    return sent_length;
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_receive(socket_0: ENetSocket,
                                             address: *mut ENetAddress,
                                             buffers: *mut ENetBuffer,
                                             buffer_count: SizeT)
 -> libc::c_int {
    let mut msg_hdr: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut sin: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut recv_length: libc::c_int = 0;
    memset(&mut msg_hdr as *mut msghdr as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<msghdr>() as libc::c_ulong);
    if !address.is_null() {
        msg_hdr.msg_name = &mut sin as *mut sockaddr_in as *mut libc::c_void;
        msg_hdr.msg_namelen =
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as SockLenT
    }
    msg_hdr.msg_iov = buffers as *mut iovec;
    msg_hdr.msg_iovlen = buffer_count;
    recv_length =
        recvmsg(socket_0, &mut msg_hdr, MSG_NOSIGNAL as libc::c_int) as
            libc::c_int;
    if recv_length == -1i32 {
        if *__errno_location() == 11i32 { return 0i32 }
        return -1i32
    }
    if 0 != msg_hdr.msg_flags & MSG_TRUNC as libc::c_int { return -1i32 }
    if !address.is_null() {
        (*address).host = sin.sin_addr.s_addr;
        (*address).port = ntohs(sin.sin_port)
    }
    return recv_length;
}
#[no_mangle]
pub unsafe extern "C" fn enet_socketset_select(max_socket: ENetSocket,
                                               read_set:
                                                   *mut ENetSocketSet,
                                               write_set:
                                                   *mut ENetSocketSet,
                                               timeout: EnetUint32)
 -> libc::c_int {
    let mut time_val: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    time_val.tv_sec =
        timeout.wrapping_div(1000i32 as libc::c_uint) as __TimeT;
    time_val.tv_usec =
        timeout.wrapping_rem(1000i32 as
                                 libc::c_uint).wrapping_mul(1000i32 as
                                                                libc::c_uint)
            as SusecondsT;
    return select(max_socket + 1i32, read_set, write_set, 0 as *mut fd_set,
                  &mut time_val);
}
#[no_mangle]
pub unsafe extern "C" fn enet_socket_wait(socket_0: ENetSocket,
                                          condition: *mut EnetUint32,
                                          timeout: EnetUint32)
 -> libc::c_int {
    let mut poll_socket: pollfd = pollfd{fd: 0, events: 0, revents: 0,};
    let mut poll_count: libc::c_int = 0;
    poll_socket.fd = socket_0;
    poll_socket.events = 0i32 as libc::c_short;
    if 0 != *condition & ENET_SOCKET_WAIT_SEND as libc::c_int as libc::c_uint
       {
        poll_socket.events =
            (poll_socket.events as libc::c_int | 0x4i32) as libc::c_short
    }
    if 0 !=
           *condition &
               ENET_SOCKET_WAIT_RECEIVE as libc::c_int as libc::c_uint {
        poll_socket.events =
            (poll_socket.events as libc::c_int | 0x1i32) as libc::c_short
    }
    poll_count = poll(&mut poll_socket, 1i32 as NfdsT, timeout as libc::c_int);
    if poll_count < 0i32 {
        if *__errno_location() == 4i32 &&
               0 !=
                   *condition &
                       ENET_SOCKET_WAIT_INTERRUPT as libc::c_int as
                           libc::c_uint {
            *condition =
                ENET_SOCKET_WAIT_INTERRUPT as libc::c_int as EnetUint32;
            return 0i32
        }
        return -1i32
    }
    *condition = ENET_SOCKET_WAIT_NONE as libc::c_int as EnetUint32;
    if poll_count == 0i32 { return 0i32 }
    if 0 != poll_socket.revents as libc::c_int & 0x4i32 {
        *condition |= ENET_SOCKET_WAIT_SEND as libc::c_int as libc::c_uint
    }
    if 0 != poll_socket.revents as libc::c_int & 0x1i32 {
        *condition |= ENET_SOCKET_WAIT_RECEIVE as libc::c_int as libc::c_uint
    }
    return 0i32;
}
