use legion::systems::Resources;

use crate::StrayBuilder;

pub trait Plugin{
    fn build(&self, stray: StrayBuilder, resources: &mut Resources) -> StrayBuilder;
}

