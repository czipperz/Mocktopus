use crate::mocking::MockResult;

pub trait Mock {
    type Input;
    type Output;

    fn call_mock(&mut self, input: Self::Input) -> MockResult<Self::Input, Self::Output>;
}

impl<I, O, T: FnMut<I, Output=MockResult<I, O>>> Mock for T {
    type Input = I;
    type Output = O;

    fn call_mock(&mut self, input: Self::Input) -> MockResult<I, O> {
        self.call_mut(input)
    }
}
