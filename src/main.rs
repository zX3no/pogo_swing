use bevy::prelude::*;
use heron::*;

#[derive(Component)]
struct Player;

#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(spawn)
        .add_system(handle_input)
        .add_system(handle_collision)
        .add_system(update_camera)
        .insert_resource(Gravity::from(Vec3::new(0., -400., 0.)))
        .run();
}

//How high the player can bounce
const BOUNCE_HEIGHT: f32 = 350.;

//How fast player can turn around
const TURN_RATIO: f32 = 1.;

fn handle_collision(
    mut events: EventReader<CollisionEvent>,
    transform: Query<&Transform, With<PogoStick>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    for event in events.iter() {
        let (_, player_layer) = event.collision_layers();
        let mut player = players.single_mut();
        if event.is_started() && is_player(player_layer) {
            let quat = transform.single().rotation;

            let a = if quat.z.is_sign_positive() {
                (2. * quat.w.acos()) * -1.
            } else {
                2. * quat.w.acos()
            };

            let velocity = player.linear;
            let mag = velocity.length();
            let new_dir = Vec3::new(a.sin() * mag, a.cos() * mag, 0.);
            player.linear.y = -player.linear.y;
            player.linear += new_dir * TURN_RATIO;
            player.linear = player.linear.normalize_or_zero() * BOUNCE_HEIGHT;
        }
    }
}

fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Player) && !layers.contains_group(Layer::World)
}

fn update_camera(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera = cameras.single_mut();
    let player = players.single();
    let mut new_pos = player.translation;
    new_pos.z = 999.99;
    camera.translation = new_pos;
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Cuboid
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::X * 10.0),
            GlobalTransform::default(),
        ))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(10.0, 50.0).extend(0.0),
            border_radius: None,
        })
        .insert(RigidBody::Dynamic);

    // Floor
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::Y * -200.),
            GlobalTransform::default(),
        ))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(10000., 100.).extend(0.),
            border_radius: None,
        })
        .insert(RigidBody::Static);

    //Player
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player_x2.png"),
            transform: Transform::from_translation(Vec3::new(-400., 0., 0.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(Velocity::default())
        .with_children(|children| {
            //Pogostick
            children
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(12., -85., 0.)),
                    ..Default::default()
                })
                .insert(CollisionShape::Sphere { radius: 10. })
                .insert(RotationConstraints::lock())
                .insert(PogoStick)
                .insert(CollisionLayers::new(Layer::Player, Layer::World));

            //Body
            children
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(-5., -0., 0.)),
                    ..Default::default()
                })
                .insert(CollisionShape::Capsule {
                    half_segment: 60.,
                    radius: 35.,
                })
                .insert(CollisionLayers::none());

            //Head
            children
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(-10., 55., 0.)),
                    ..Default::default()
                })
                .insert(CollisionShape::Sphere { radius: 35. })
                .insert(CollisionLayers::none());
        });
}

#[derive(Component)]
struct PogoStick;

const ROTATION_SPEED: f32 = 0.09;

fn handle_input(
    input: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, (With<Player>, Without<PogoStick>)>,
) {
    let mut player = player.single_mut();

    if input.pressed(KeyCode::A) {
        player.rotate(Quat::from_rotation_z(ROTATION_SPEED));
    } else if input.pressed(KeyCode::D) {
        player.rotate(Quat::from_rotation_z(-ROTATION_SPEED));
    }
}
