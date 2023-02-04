use crate::StrayBuilder;

pub trait Plugin{
    fn build(&self, stray: &mut StrayBuilder);
}

