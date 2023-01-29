use rapier2d::prelude::*;
use legion::*;

use stray_scene::*;

pub struct Body{
    pub body_type: RigidBodyType,
    pub collider: Collider,
}

pub trait Initializer{
    type Data;
    type ReturnData;
    fn init(res: &mut Resources, data: Self::Data) -> Self::ReturnData;
}

impl Initializer for RigidBody{
    type Data = Body;
    type ReturnData = RigidBodyHandle;

    fn init(res: &mut Resources, data: Self::Data) -> Self::ReturnData {
        let rigid_body = RigidBodyBuilder::new(data.body_type).build();
        let handle = res.get_mut::<PhysicData<RigidBodySet>>().unwrap().0.insert(
            rigid_body
        );
        
        res.get_mut::<PhysicData<ColliderSet>>().unwrap().0.insert_with_parent(
            data.collider, 
            handle, 
            &mut res.get_mut::<PhysicData<RigidBodySet>>().unwrap().0
        );
        handle
        
    }
}

#[system(for_each)]
pub fn synchronize_physics(
    transform: &mut Transform2D, 
    body_handle: &RigidBodyHandle,
    #[resource] integration_parameters: &PhysicData<IntegrationParameters>,
    #[resource] physics_pipeline: &mut PhysicData<PhysicsPipeline>,
    #[resource] island_manager: &mut PhysicData<IslandManager>,
    #[resource] broad_phase: &mut PhysicData<BroadPhase>,
    #[resource] narrow_phase: &mut PhysicData<NarrowPhase>,
    #[resource] rigid_body_set: &mut PhysicData<RigidBodySet>,
    #[resource] collider_set: &mut PhysicData<ColliderSet>,
    #[resource] impulse_joint_set: &mut PhysicData<ImpulseJointSet>,
    #[resource] multibody_joint_set: &mut PhysicData<MultibodyJointSet>,
    #[resource] ccd_solver: &mut PhysicData<CCDSolver>,

){
    let gravity = vector![0.0, -9.81];
    physics_pipeline.0.step(
        &gravity,
        &integration_parameters.0,
        &mut island_manager.0,
        &mut broad_phase.0,
        &mut narrow_phase.0,
        &mut rigid_body_set.0,
        &mut collider_set.0,
        &mut impulse_joint_set.0,
        &mut multibody_joint_set.0,
        &mut ccd_solver.0,
        None,   
        &(),
        &(),
    );
    let ball_body = &rigid_body_set.0[body_handle.0];
    transform.position.x = ball_body.translation().x;
    transform.position.y = ball_body.translation().y;
}


