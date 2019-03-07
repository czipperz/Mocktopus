use crate::mocking::MockResult;

pub trait Mock0 {
    type Output;

    fn call_mock(&mut self) -> MockResult<(), Self::Output>;
}

pub trait Mock1<I> {
    type Output;

    fn call_mock(&mut self, input: I) -> MockResult<(I,), Self::Output>;
}

pub trait Mock2<I, J> {
    type Output;

    fn call_mock(&mut self, input1: I, input2: J) -> MockResult<(I, J), Self::Output>;
}

pub trait Mock3<I, J, K> {
    type Output;

    fn call_mock(&mut self, input1: I, input2: J, input3: K) -> MockResult<(I, J, K), Self::Output>;
}

impl<O, T: FnMut() -> MockResult<(), O>> Mock0 for T {
    type Output = O;

    fn call_mock(&mut self) -> MockResult<(), Self::Output> {
        self()
    }
}

impl<I, O, T: FnMut(I) -> MockResult<(I,), O>> Mock1<I> for T {
    type Output = O;

    fn call_mock(&mut self, input: I) -> MockResult<(I,), Self::Output> {
        self(input)
    }
}

impl<I, J, O, T: FnMut(I, J) -> MockResult<(I, J), O>> Mock2<I, J> for T {
    type Output = O;

    fn call_mock(&mut self, input1: I, input2: J) -> MockResult<(I, J), Self::Output> {
        self(input1, input2)
    }
}

impl<I, J, K, O, T: FnMut(I, J, K) -> MockResult<(I, J, K), O>> Mock3<I, J, K> for T {
    type Output = O;

    fn call_mock(&mut self, input1: I, input2: J, input3: K) -> MockResult<(I, J, K), Self::Output> {
        self(input1, input2, input3)
    }
}


//fn main() {
//    let x: bool = make_my_day(&|x| x);
//}
//
//fn make_my_day<T: Fn<I, Output = U>, I, U>(t: &T) -> &Fn<I, Output = U> {
//    &t as &Fn<I, Output = U>
//}
