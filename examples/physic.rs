use stray::prelude::*;
use legion::*;
use rapier2d::prelude::*;

#[system(for_each)]
fn movement(
    #[resource] rbs: &mut PhysicData<RigidBodySet>, 
    rbh: &mut RigidBodyHandle
) {
    let rigid_body = rbs.0.get_mut(*rbh).unwrap();
    // Code
}
fn main(){
    let mut resources = Resources::default();

    Stray::new()
        .with_size(1000, 500)
        .add_plugin(PhysicPlugin, &mut resources)
        .push((
            RigidBody::init(
                &mut resources,
                Body{
                        body_type: RigidBodyType::Dynamic,
                        collider: ColliderBuilder::ball(0.5).build()
                    }
            ),
            Transform2D::new(0, 0, 0, 1.0),
            Sprite::new(include_bytes!("sprite.png"), 0)
        ))
        .set_resources(resources)
        .add_system(movement_system())
        .build()
        .run();
}