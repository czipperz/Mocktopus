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

pub trait Mockable<I> {
    type Output;

    unsafe fn mock_raw(&self, mock: impl Mock<I, Output = Self::Output>);

    fn mock_safe(&self, mock: impl Mock<I, Output = Self::Output> + 'static);

    #[doc(hidden)]
    fn call_mock(&self, input: I) -> MockResult<I, Self::Output>;
}

thread_local!{
    static MOCK_STORE: RefCell<HashMap<TypeId, Rc<RefCell<Box<Mock<(), Output = ()>>>>>> = RefCell::new(HashMap::new());
}

fn get_mock_id<T>(_: T) -> TypeId {
    (||()).type_id()
}

impl<O, F: Fn() -> O> Mockable<()> for F {
    type Output = O;

    unsafe fn mock_raw(&self, mock: impl Mock<(), Output = Self::Output>) {
        let id = get_mock_id(self);
        MOCK_STORE.with(|mock_ref_cell| {
            let real = Rc::new(RefCell::new(Box::new(mock) as Box<Mock<(), Output = _>>));
            let stored = transmute(real);
            mock_ref_cell.borrow_mut()
                .insert(id, stored);
        })
    }

    fn mock_safe(&self, mock: impl Mock<(), Output = Self::Output> + 'static ) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    fn call_mock(&self, input: ()) -> MockResult<(), Self::Output> {
        unsafe {
            let id = get_mock_id(self);
            let rc_opt = MOCK_STORE.with(|mock_ref_cell|
                mock_ref_cell.borrow()
                    .get(&id)
                    .cloned()
            );
            let stored_opt = rc_opt.as_ref()
                .and_then(|rc| rc.try_borrow_mut().ok());
            match stored_opt {
                Some(mut stored) => {
                    let real: &mut Box<Mock<(), Output = O>> = transmute(&mut*stored);
                    real.call_mock(input)
                }
                None => MockResult::Continue(input),
            }
        }
    }
}

impl<I1, O, F: Fn(I1) -> O> Mockable<(I1,)> for F {
    type Output = O;

    unsafe fn mock_raw(&self, mock: impl Mock<(I1,), Output = Self::Output>) {
        let id = get_mock_id(self);
        MOCK_STORE.with(|mock_ref_cell| {
            let real = Rc::new(RefCell::new(Box::new(mock) as Box<Mock<(I1,), Output = _>>));
            let stored = transmute(real);
            mock_ref_cell.borrow_mut()
                .insert(id, stored);
        })
    }

    fn mock_safe(&self, mock: impl Mock<(I1,), Output = Self::Output> + 'static ) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    fn call_mock(&self, input: (I1,)) -> MockResult<(I1,), Self::Output> {
        unsafe {
            let id = get_mock_id(self);
            let rc_opt = MOCK_STORE.with(|mock_ref_cell|
                mock_ref_cell.borrow()
                    .get(&id)
                    .cloned()
            );
            let stored_opt = rc_opt.as_ref()
                .and_then(|rc| rc.try_borrow_mut().ok());
            match stored_opt {
                Some(mut stored) => {
                    let real: &mut Box<Mock<(I1,), Output = O>> = transmute(&mut*stored);
                    real.call_mock(input)
                }
                None => MockResult::Continue(input),
            }
        }
    }
}

impl<I1, I2, O, F: Fn(I1, I2) -> O> Mockable<(I1, I2)> for F {
    type Output = O;

    unsafe fn mock_raw(&self, mock: impl Mock<(I1, I2), Output = Self::Output>) {
        let id = get_mock_id(self);
        MOCK_STORE.with(|mock_ref_cell| {
            let real = Rc::new(RefCell::new(Box::new(mock) as Box<Mock<(I1, I2), Output = _>>));
            let stored = transmute(real);
            mock_ref_cell.borrow_mut()
                .insert(id, stored);
        })
    }

    fn mock_safe(&self, mock: impl Mock<(I1, I2), Output = Self::Output> + 'static ) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    fn call_mock(&self, input: (I1, I2)) -> MockResult<(I1, I2), Self::Output> {
        unsafe {
            let id = get_mock_id(self);
            let rc_opt = MOCK_STORE.with(|mock_ref_cell|
                mock_ref_cell.borrow()
                    .get(&id)
                    .cloned()
            );
            let stored_opt = rc_opt.as_ref()
                .and_then(|rc| rc.try_borrow_mut().ok());
            match stored_opt {
                Some(mut stored) => {
                    let real: &mut Box<Mock<(I1, I2), Output = O>> = transmute(&mut*stored);
                    real.call_mock(input)
                }
                None => MockResult::Continue(input),
            }
        }
    }
}

impl<I1, I2, I3, O, F: Fn(I1, I2, I3) -> O> Mockable<(I1, I2, I3)> for F {
    type Output = O;

    unsafe fn mock_raw(&self, mock: impl Mock<(I1, I2, I3), Output = Self::Output>) {
        let id = get_mock_id(self);
        MOCK_STORE.with(|mock_ref_cell| {
            let real = Rc::new(RefCell::new(Box::new(mock) as Box<Mock<(I1, I2, I3), Output = _>>));
            let stored = transmute(real);
            mock_ref_cell.borrow_mut()
                .insert(id, stored);
        })
    }

    fn mock_safe(&self, mock: impl Mock<(I1, I2, I3), Output = Self::Output> + 'static ) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    fn call_mock(&self, input: (I1, I2, I3)) -> MockResult<(I1, I2, I3), Self::Output> {
        unsafe {
            let id = get_mock_id(self);
            let rc_opt = MOCK_STORE.with(|mock_ref_cell|
                mock_ref_cell.borrow()
                    .get(&id)
                    .cloned()
            );
            let stored_opt = rc_opt.as_ref()
                .and_then(|rc| rc.try_borrow_mut().ok());
            match stored_opt {
                Some(mut stored) => {
                    let real: &mut Box<Mock<(I1, I2, I3), Output = O>> = transmute(&mut*stored);
                    real.call_mock(input)
                }
                None => MockResult::Continue(input),
            }
        }
    }
}


