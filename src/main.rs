#![allow(non_camel_case_types)]
/**
 * Dead code means that is prototype of a function
 * 
 * /
/* arch-part */
#[allow(dead_code)]
type b8 = bool;

/**
 * kernel linked list element
 */
#[allow(dead_code)]
struct klle {
	data : *mut Void,       /* data pointer */ //FIXME
	next : *mut klle,  /* next member pointer */
	prev : *mut klle,  /* next member pointer */
}
#[allow(dead_code)]
type klle_t = klle;



/* functions */
#[allow(dead_code)]
#[inline(always)]
fn kllist_empty(nurse : *mut klle_t)->b8
{
    let result : b8 = unsafe {(*nurse).next == nurse};
    return result;
}


fn main() {
    println!("Hello, world!");
    
}
