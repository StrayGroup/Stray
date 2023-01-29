use legion::Resources;
use stray_api::{Stray, Plugin, StrayBuilder};
use stray_scene::PhysicData;
use rapier2d::prelude::*;


mod rigidbody;
pub use rigidbody::*;

pub struct PhysicPlugin;

impl Plugin for PhysicPlugin{
    fn build(&self, mut stray: StrayBuilder, resources: &mut Resources) -> StrayBuilder{
        begin_physic(resources);
        stray.add_system(synchronize_physics_system())
    }
}



pub fn begin_physic(res: &mut legion::Resources){
    let rigid_body_set = RigidBodySet::new();
    let collider_set = ColliderSet::new();
    let integration_parameters = IntegrationParameters::default();
    let physics_pipeline = PhysicsPipeline::new();
    let island_manager = IslandManager::new();
    let broad_phase = BroadPhase::new();
    let narrow_phase = NarrowPhase::new();
    let impulse_joint_set = ImpulseJointSet::new();
    let multibody_joint_set = MultibodyJointSet::new();
    let ccd_solver = CCDSolver::new();
    res.insert(PhysicData(physics_pipeline));
    res.insert(PhysicData(integration_parameters));
    res.insert(PhysicData(island_manager));
    res.insert(PhysicData(broad_phase));
    res.insert(PhysicData(narrow_phase));
    res.insert(PhysicData(rigid_body_set));
    res.insert(PhysicData(collider_set));
    res.insert(PhysicData(impulse_joint_set));
    res.insert(PhysicData(multibody_joint_set));
    res.insert(PhysicData(ccd_solver));

}