use crate::mocking::MockResult;

use std::boxed::Box;
use std::marker::PhantomData;

pub trait Mock<I> {
    type Output;

    fn call_mock(&mut self, input: I) -> MockResult<I, Self::Output>;

    fn fake_call(&mut self, input: I) -> Self::Output {
        match self.call_mock(input) {
            MockResult::Return(r) => r,
            _ => unimplemented!(),
        }
    }

    fn into_mock_container(self) -> MockContainer<fn(Self::Output) -> I> where Self: Sized {
        let mock_box = Box::new(self) as Box<Mock<I, Output = Self::Output>>;
        MockContainer {
            mock: unsafe { std::mem::transmute(mock_box) },
            _variance_guard: PhantomData::default(),
        }
    }

    fn into_mock_container_normal(self) -> MockContainer<fn(I) -> Self::Output> where Self: Sized {
        let mock_box = Box::new(self) as Box<Mock<I, Output = Self::Output>>;
        MockContainer {
            mock: unsafe { std::mem::transmute(mock_box) },
            _variance_guard: PhantomData::default(),
        }
    }
}

pub struct MockContainer<F> {
    pub mock: Box<Mock<(), Output = ()>>,
    _variance_guard: PhantomData<F>,
}

pub trait SafeMock<I>/* : Mock<I, Output = O> */ {
    type Output: 'static;

    // fn call_mock(&mut self, input: I) -> MockResult<I, Self::Output>;
}

impl <I, O: 'static, M: Mock<I, Output = O>> SafeMock<I> for M {
    type Output = O;

    // fn call_mock(&mut self, input: I) -> MockResult<I, Self::Output> {
    //     M::call_mock(self, input)
    // }
}






impl<O, T: FnMut() -> MockResult<(), O>> Mock<()> for T {
    type Output = O;

    fn call_mock(&mut self, _input: ()) -> MockResult<(), Self::Output> {
        self()
    }
}

impl<I1, O, T: FnMut(I1) -> MockResult<(I1,), O>> Mock<(I1,)> for T {
    type Output = O;

    fn call_mock(&mut self, input: (I1,)) -> MockResult<(I1,), Self::Output> {
        self(input.0)
    }
}

impl<I1, I2, O, T: FnMut(I1, I2) -> MockResult<(I1, I2), O>> Mock<(I1, I2)> for T {
    type Output = O;

    fn call_mock(&mut self, input: (I1, I2)) -> MockResult<(I1, I2), Self::Output> {
        self(input.0, input.1)
    }
}

impl<I1, I2, I3, O, T: FnMut(I1, I2, I3) -> MockResult<(I1, I2, I3), O>> Mock<(I1, I2, I3)> for T {
    type Output = O;

    fn call_mock(&mut self, input: (I1, I2, I3)) -> MockResult<(I1, I2, I3), Self::Output> {
        self(input.0, input.1, input.2)
    }
}