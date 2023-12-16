use std::fmt::Debug;

pub struct InspectPrint<I: Iterator> {
    iter: I,
}

impl<I: Iterator> Iterator for InspectPrint<I>
where
    I::Item: Debug,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            println!("{:?}", item);

            return Some(item);
        }

        None
    }
}

pub trait AdventOfCodeIteratorUtils: Iterator {
    fn println(self) -> InspectPrint<Self>
    where
        Self: Sized,
        Self::Item: Debug,
    {
        InspectPrint { iter: self }
    }
}

impl<I: Iterator> AdventOfCodeIteratorUtils for I {}
