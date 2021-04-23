#![allow(non_camel_case_types)]
/* arch-part */
type b8 = bool;

/**
 * kernel linked list element
 */
struct klle {
	data : u32,       /* data pointer */ //FIXME
	next : *mut klle,  /* next member pointer */
	prev : *mut klle,  /* next member pointer */
}
type klle_t = klle;

/* functions */
#[inline(always)]
fn kllist_empty(nurse : *mut klle_t)->b8
{
    let result : b8 = unsafe {((*nurse).next == nurse)};
    return result;
}


fn main() {
    println!("Hello, world!");
}
