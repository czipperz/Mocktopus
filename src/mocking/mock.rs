use crate::mocking::MockResult;

pub trait Mock<I> {
    type Output;

    fn call_mock(&mut self, input: I) -> MockResult<I, Self::Output>;
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