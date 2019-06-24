use libc;
extern "C" {
    /* * @defgroup global ENet global functions
    @{ 
*/
    /* * 
  Initializes ENet globally.  Must be called prior to using any functions in
  ENet.
  @returns 0 on success, < 0 on failure
*/
    #[no_mangle]
    fn enet_initialize() -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
}
pub type SizeT = libc::c_ulong;
/* *< unsigned 32-bit type */
pub type EnetUint32 = libc::c_uint;
/* * 
 @file  callbacks.h
 @brief ENet callbacks
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _ENetCallbacks {
    pub malloc: Option<unsafe extern "C" fn(_: SizeT) -> *mut libc::c_void>,
    pub free: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub no_memory: Option<unsafe extern "C" fn() -> ()>,
}
pub type ENetCallbacks = _ENetCallbacks;
pub type ENetVersion = EnetUint32;
/* * 
 @file callbacks.c
 @brief ENet callback functions
*/
static mut callbacks: ENetCallbacks =
    unsafe {
        _ENetCallbacks{malloc:
                           Some(malloc as
                                    unsafe extern "C" fn(_: libc::c_ulong)
                                        -> *mut libc::c_void),
                       free:
                           Some(free as
                                    unsafe extern "C" fn(_: *mut libc::c_void)
                                        -> ()),
                       no_memory:
                           ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                              -> !>,
                                                   Option<unsafe extern "C" fn()
                                                              ->
                                                                  ()>>(Some(abort
                                                                                as
                                                                                unsafe extern "C" fn()
                                                                                    ->
                                                                                        !)),}
    };
/* * 
  Initializes ENet globally and supplies user-overridden callbacks. Must be called prior to using any functions in ENet. Do not use enet_initialize() if you use this variant. Make sure the ENetCallbacks structure is zeroed out so that any additional callbacks added in future versions will be properly ignored.

  @param version the constant ENET_VERSION should be supplied so ENet knows which version of ENetCallbacks struct to use
  @param inits user-overridden callbacks where any NULL callbacks will use ENet's defaults
  @returns 0 on success, < 0 on failure
*/
#[no_mangle]
pub unsafe extern "C" fn enet_initialize_with_callbacks(mut version:
                                                            ENetVersion,
                                                        mut inits:
                                                            *const ENetCallbacks)
 -> libc::c_int {
    if version < (1i32 << 16i32 | 3i32 << 8i32 | 0i32) as libc::c_uint {
        return -1i32
    }
    if (*inits).malloc.is_some() || (*inits).free.is_some() {
        if (*inits).malloc.is_none() || (*inits).free.is_none() {
            return -1i32
        }
        callbacks.malloc = (*inits).malloc;
        callbacks.free = (*inits).free
    }
    if (*inits).no_memory.is_some() {
        callbacks.no_memory = (*inits).no_memory
    }
    return enet_initialize();
}
/* *
  Gives the linked version of the ENet library.
  @returns the version number 
*/
#[no_mangle]
pub unsafe extern "C" fn enet_linked_version() -> ENetVersion {
    return (1i32 << 16i32 | 3i32 << 8i32 | 14i32) as ENetVersion;
}
/* * @defgroup callbacks ENet internal callbacks
    @{
    @ingroup private
*/
#[no_mangle]
pub unsafe extern "C" fn enet_malloc(mut size: SizeT) -> *mut libc::c_void {
    let mut memory: *mut libc::c_void =
        callbacks.malloc.expect("non-null function pointer")(size);
    if memory.is_null() {
        callbacks.no_memory.expect("non-null function pointer")();
    }
    return memory;
}
#[no_mangle]
pub unsafe extern "C" fn enet_free(mut memory: *mut libc::c_void) {
    callbacks.free.expect("non-null function pointer")(memory);
}