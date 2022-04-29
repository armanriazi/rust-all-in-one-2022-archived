
use std::ops::AddAssign;
use std::cmp::PartialOrd;


pub struct Stepper<T>{
    curr:T,
    step:T,
    stop:T,
}

impl<T> Stepper<T>{
    pub fn new(start:T,stop:T,step:T)->Self{
        Stepper{
            curr:start,
            stop:stop,
            step:step,
        }
    }
}

impl<T> Iterator for Stepper<T>
    where T:AddAssign + Copy + PartialOrd
{
    type Item=T;

    fn next(&mut self)->Option<T>{
        if self.curr >= self.stop {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}