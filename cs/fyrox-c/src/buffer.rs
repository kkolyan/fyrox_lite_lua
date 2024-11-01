// use std::cell::RefCell;
// use std::cmp::Ordering;
// use std::marker::PhantomData;
// use std::rc::Rc;
// use std::slice::{from_raw_parts, from_raw_parts_mut};
// use fyrox_lite::LiteDataType;
// use fyrox_lite::spi::Buffer;
// 
// #[derive(Clone, Debug)]
// pub struct ExternalBuffer<T, S> {
//     inner: Rc<RefCell<Inner<S>>>,
//     _pd: PhantomData<T>,
// }
// 
// #[derive(Debug)]
// struct Inner<T> {
//     data: *mut T,
//     cap: u32,
//     occ: u32,
// }
// 
// impl<T: LiteDataType, S> ExternalBuffer<T, S> {
//     /// Safety: all other methods of this struct are safe only if the pointer supplied here is not used anywhere else at the same time.
//     pub unsafe fn from_raw(data: *mut T, cap: u32) -> ExternalBuffer<T> {
//         Self {
//             inner: Rc::new(RefCell::new(Inner { data, cap, occ: 0 })),
//             _pd: Default::default(),
//         }
//     }
// }
// 
// impl<T: LiteDataType, S> Buffer<T> for ExternalBuffer<T, S> {
//     fn add(&mut self, value: T) {
//         let mut ref_mut = self.inner.borrow_mut();
//         unsafe { *ref_mut.data.add(ref_mut.occ as usize) = value; }
//         ref_mut.occ += 1;
//     }
// 
//     fn sort_by(&mut self, cmp: &mut dyn FnMut(&T, &T) -> Ordering) {
//         let ref_mut = self.inner.borrow_mut();
//         let slice = unsafe { from_raw_parts_mut(ref_mut.data, ref_mut.occ as usize) };
//         slice.sort_by(cmp);
//     }
// 
//     fn clear(&mut self) {
//         let mut ref_mut = self.inner.borrow_mut();
//         ref_mut.occ = 0;
//     }
// }
// 
// impl<T: LiteDataType, S> LiteDataType for ExternalBuffer<T, S> {}