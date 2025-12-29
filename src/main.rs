use std::ops::Add;

trait generic_sum<A>
{
    fn gen_sum(self) -> A;
}

impl<T: IntoIterator<Item=A>, A: Add<Output = A> + Default> generic_sum<A> for T
{
    fn gen_sum(self) -> A
    {
        let mut ret_val: A = Default::default();

        let mut iterator = self.into_iter();

        while let Some(current) = iterator.next()
        {
            ret_val = ret_val + current;
        }
        ret_val
    }
}

fn main() {
    println!("summed vector: {}",[1,2,3].gen_sum());
}
