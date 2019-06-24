use libc;
pub type SizeT = libc::c_ulong;
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
/* * 
 @file list.c
 @brief ENet linked list functions
*/
/* * 
    @defgroup list ENet linked list utility functions
    @ingroup private
    @{
*/
#[no_mangle]
pub unsafe extern "C" fn enet_list_clear(mut list: *mut ENetList) {
    (*list).sentinel.next = &mut (*list).sentinel;
    (*list).sentinel.previous = &mut (*list).sentinel;
}
#[no_mangle]
pub unsafe extern "C" fn enet_list_insert(mut position: ENetListIterator,
                                          mut data: *mut libc::c_void)
 -> ENetListIterator {
    let mut result: ENetListIterator = data as ENetListIterator;
    (*result).previous = (*position).previous;
    (*result).next = position;
    (*(*result).previous).next = result;
    (*position).previous = result;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn enet_list_remove(mut position: ENetListIterator)
 -> *mut libc::c_void {
    (*(*position).previous).next = (*position).next;
    (*(*position).next).previous = (*position).previous;
    return position as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn enet_list_move(mut position: ENetListIterator,
                                        mut dataFirst: *mut libc::c_void,
                                        mut dataLast: *mut libc::c_void)
 -> ENetListIterator {
    let mut first: ENetListIterator = dataFirst as ENetListIterator;
    let mut last: ENetListIterator = dataLast as ENetListIterator;
    (*(*first).previous).next = (*last).next;
    (*(*last).next).previous = (*first).previous;
    (*first).previous = (*position).previous;
    (*last).next = position;
    (*(*first).previous).next = first;
    (*position).previous = last;
    return first;
}
#[no_mangle]
pub unsafe extern "C" fn enet_list_size(mut list: *mut ENetList) -> SizeT {
    let mut size: SizeT = 0i32 as SizeT;
    let mut position: ENetListIterator = 0 as *mut ENetListNode;
    position = (*list).sentinel.next;
    while position != &mut (*list).sentinel as *mut ENetListNode {
        size = size.wrapping_add(1);
        position = (*position).next
    }
    return size;
}