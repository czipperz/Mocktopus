use crate::mocking::mock::*;
use crate::mocking::MockResult;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::transmute;
use std::rc::Rc;

/// Trait for setting up mocks
///
/// The trait is implemented for all functions, so its methods can be called on any function.
///
/// Note: methods have any effect only if called on functions [annotated as mockable](https://docs.rs/mocktopus_macros).
pub trait Mockable0 {
    type Output;

    unsafe fn mock_raw(&self, mock: impl Mock0<Output = Self::Output>);

    fn mock_safe(&self, mock: impl Mock0<Output = Self::Output> + 'static);

    #[doc(hidden)]
    fn call_mock(&self) -> MockResult<(), Self::Output>;

    #[doc(hidden)]
    unsafe fn get_mock_id(&self) -> TypeId;
}

pub trait Mockable1<I> {
    type Output;

    unsafe fn mock_raw(&self, mock: impl Mock1<I, Output = Self::Output>);

    fn mock_safe(&self, mock: impl Mock1<I, Output = Self::Output> + 'static);

    #[doc(hidden)]
    fn call_mock(&self, input: I) -> MockResult<(I,), Self::Output>;

    #[doc(hidden)]
    unsafe fn get_mock_id(&self) -> TypeId;
}

pub trait Mockable2<I, J> {
    type Output;

    unsafe fn mock_raw(&self, mock: impl Mock2<I, J, Output = Self::Output>);

    fn mock_safe(&self, mock: impl Mock2<I, J, Output = Self::Output> + 'static);

    #[doc(hidden)]
    fn call_mock(&self, input1: I, input2: J) -> MockResult<(I, J), Self::Output>;

    #[doc(hidden)]
    unsafe fn get_mock_id(&self) -> TypeId;
}

pub trait Mockable3<I, J, K> {
    type Output;

    unsafe fn mock_raw(&self, mock: impl Mock3<I, J, K, Output = Self::Output>);

    fn mock_safe(&self, mock: impl Mock3<I, J, K, Output = Self::Output> + 'static);

    #[doc(hidden)]
    fn call_mock(&self, input1: I, input2: J, input3: K) -> MockResult<(I, J, K), Self::Output>;

    #[doc(hidden)]
    unsafe fn get_mock_id(&self) -> TypeId;
}

thread_local!{
    static MOCK_STORE0: RefCell<HashMap<TypeId, Rc<RefCell<Box<Mock0<Output = ()>>>>>> = RefCell::new(HashMap::new());
    static MOCK_STORE1: RefCell<HashMap<TypeId, Rc<RefCell<Box<Mock1<(), Output = ()>>>>>> = RefCell::new(HashMap::new());
    static MOCK_STORE2: RefCell<HashMap<TypeId, Rc<RefCell<Box<Mock2<(), (), Output = ()>>>>>> = RefCell::new(HashMap::new());
    static MOCK_STORE3: RefCell<HashMap<TypeId, Rc<RefCell<Box<Mock3<(), (), (), Output = ()>>>>>> = RefCell::new(HashMap::new());
}

impl<O, F: FnOnce() -> O> Mockable0 for F {
    type Output = O;

    unsafe fn mock_raw(&self, mock: impl Mock0< Output = Self::Output>) {
        let id = self.get_mock_id();
        MOCK_STORE0.with(|mock_ref_cell| {
            let real = Rc::new(RefCell::new(Box::new(mock) as Box<Mock0<Output = _>>));
            let stored = transmute(real);
            mock_ref_cell.borrow_mut()
                .insert(id, stored);
        })
    }

    fn mock_safe(&self, mock: impl Mock0<Output = Self::Output> + 'static ) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    fn call_mock(&self) -> MockResult<(), Self::Output> {
        unsafe {
            let id = self.get_mock_id();
            let rc_opt = MOCK_STORE0.with(|mock_ref_cell|
                mock_ref_cell.borrow()
                    .get(&id)
                    .cloned()
            );
            let stored_opt = rc_opt.as_ref()
                .and_then(|rc| rc.try_borrow_mut().ok());
            match stored_opt {
                Some(mut stored) => {
                    let real: &mut Box<Mock0<Output = O>> = transmute(&mut*stored);
                    real.call_mock()
                }
                None => MockResult::Continue(()),
            }
        }
    }

    unsafe fn get_mock_id(&self) -> TypeId {
        (||()).type_id()
    }
}

impl<I, O, F: FnOnce(I) -> O> Mockable1<I> for F {
    type Output = O;

    unsafe fn mock_raw(&self, mock: impl Mock1<I, Output = Self::Output>) {
        let id = self.get_mock_id();
        MOCK_STORE1.with(|mock_ref_cell| {
            let real = Rc::new(RefCell::new(Box::new(mock) as Box<Mock1<_, Output = _>>));
            let stored = transmute(real);
            mock_ref_cell.borrow_mut()
                .insert(id, stored);
        })
    }

    fn mock_safe(&self, mock: impl Mock1<I, Output = Self::Output> + 'static ) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    fn call_mock(&self, input: I) -> MockResult<(I,), Self::Output> {
        unsafe {
            let id = self.get_mock_id();
            let rc_opt = MOCK_STORE1.with(|mock_ref_cell|
                mock_ref_cell.borrow()
                    .get(&id)
                    .cloned()
            );
            let stored_opt = rc_opt.as_ref()
                .and_then(|rc| rc.try_borrow_mut().ok());
            match stored_opt {
                Some(mut stored) => {
                    let real: &mut Box<Mock1<I, Output = O>> = transmute(&mut*stored);
                    real.call_mock(input)
                }
                None => MockResult::Continue((input,)),
            }
        }
    }

    unsafe fn get_mock_id(&self) -> TypeId {
        (||()).type_id()
    }
}

impl<I, J, O, F: FnOnce(I, J) -> O> Mockable2<I, J> for F {
    type Output = O;

    unsafe fn mock_raw(&self, mock: impl Mock2<I, J, Output = Self::Output>) {
        let id = self.get_mock_id();
        MOCK_STORE2.with(|mock_ref_cell| {
            let real = Rc::new(RefCell::new(Box::new(mock) as Box<Mock2<_, _, Output = _>>));
            let stored = transmute(real);
            mock_ref_cell.borrow_mut()
                .insert(id, stored);
        })
    }

    fn mock_safe(&self, mock: impl Mock2<I, J, Output = Self::Output> + 'static ) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    fn call_mock(&self, input1: I, input2: J) -> MockResult<(I, J), Self::Output> {
        unsafe {
            let id = self.get_mock_id();
            let rc_opt = MOCK_STORE2.with(|mock_ref_cell|
                mock_ref_cell.borrow()
                    .get(&id)
                    .cloned()
            );
            let stored_opt = rc_opt.as_ref()
                .and_then(|rc| rc.try_borrow_mut().ok());
            match stored_opt {
                Some(mut stored) => {
                    let real: &mut Box<Mock2<I, J, Output = O>> = transmute(&mut*stored);
                    real.call_mock(input1, input2)
                }
                None => MockResult::Continue((input1, input2)),
            }
        }
    }

    unsafe fn get_mock_id(&self) -> TypeId {
        (||()).type_id()
    }
}


impl<I, J, K, O, F: FnOnce(I, J, K) -> O> Mockable3<I, J, K> for F {
    type Output = O;

    unsafe fn mock_raw(&self, mock: impl Mock3<I, J, K, Output = Self::Output>) {
        let id = self.get_mock_id();
        MOCK_STORE3.with(|mock_ref_cell| {
            let real = Rc::new(RefCell::new(Box::new(mock) as Box<Mock3<_, _, _, Output = _>>));
            let stored = transmute(real);
            mock_ref_cell.borrow_mut()
                .insert(id, stored);
        })
    }

    fn mock_safe(&self, mock: impl Mock3<I, J, K, Output = Self::Output> + 'static ) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    fn call_mock(&self, input1: I, input2: J, input3: K) -> MockResult<(I, J, K), Self::Output> {
        unsafe {
            let id = self.get_mock_id();
            let rc_opt = MOCK_STORE3.with(|mock_ref_cell|
                mock_ref_cell.borrow()
                    .get(&id)
                    .cloned()
            );
            let stored_opt = rc_opt.as_ref()
                .and_then(|rc| rc.try_borrow_mut().ok());
            match stored_opt {
                Some(mut stored) => {
                    let real: &mut Box<Mock3<I, J, K, Output = O>> = transmute(&mut*stored);
                    real.call_mock(input1, input2, input3)
                }
                None => MockResult::Continue((input1, input2, input3)),
            }
        }
    }

    unsafe fn get_mock_id(&self) -> TypeId {
        (||()).type_id()
    }
}
