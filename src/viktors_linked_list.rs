pub mod viktors_linked_list{
pub struct viktors_linked_list< T>{
    next : *mut viktors_linked_list<T>,
    prev : *mut viktors_linked_list<T>,
    data : T,
}
pub type vtl = viktors_linked_list<u8>;


 impl<u8> viktors_linked_list<u8> {
    pub unsafe fn init(&mut self)  {
        //viktors_linked_list {next : this, prev : this};
        (*self).prev = self;
        (*self).next = self;
    }
}}
