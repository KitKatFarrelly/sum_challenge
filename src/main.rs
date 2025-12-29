use std::ops::Add;
use std::iter::Sum;

trait generic_sum<A>
{
    fn gen_sum(self) -> A;
}

impl<T: IntoIterator<Item=A>, A: Sum> generic_sum<A> for T
{
    fn gen_sum(self) -> A
    {
        let mut iterator = self.into_iter();

        /*
        while let Some(current) = iterator.next()
        {
            ret_val = ret_val + current;
        }
        */
        iterator.sum::<A>()
    }
}

fn main() {
    println!("summed vector: {}",[1,2,3].gen_sum());
}
