use std::iter::Zip;

pub fn zip<A, B>(t: (A, B)) 
-> Zip<<A as IntoIterator>::IntoIter, <B as IntoIterator>::IntoIter>
where
    B: IntoIterator,
    A: IntoIterator, 
{
    t.0.into_iter().zip(t.1.into_iter())
}