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
        .insert_resource(Gravity::from(Vec3::new(0., -400.0, 0.)))
        .run();
}

//How high the player can bounce
const BOUNCE_HEIGHT: f32 = 350.;

//How fast player can turn around
const TURN_RATIO: f32 = 1.;

fn handle_collision(
    mut events: EventReader<CollisionEvent>,
    transform: Query<&mut Transform, With<Player>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    for event in events.iter() {
        let (_, player_layer) = event.collision_layers();
        let mut player = players.single_mut();
        if event.is_started() && is_player(player_layer) {
            let quat = transform.single().rotation;

            let pos = quat.z.is_sign_positive();
            let a = if pos {
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
            Transform::from_translation(Vec3::Y * -200.0),
            GlobalTransform::default(),
        ))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(10000.0, 20.0).extend(0.0),
            border_radius: None,
        })
        .insert(RigidBody::Static);

    //Player
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player_x2.png"),
            transform: Transform::from_translation(Vec3::new(-400.0, 00.0, 0.0)),
            ..Default::default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        //TODO: maybe attach the rigid body offset to the player?
        //Maybe multiple colliders
        //The bottom of the pogo stick
        //The head for wipeouts
        //The body for general collisions
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(30., 90., 0.),
            border_radius: None,
        })
        .insert(RotationConstraints::lock())
        .insert(CollisionLayers::new(Layer::Player, Layer::World))
        .insert(Velocity::default());
}

const ROTATION_SPEED: f32 = 0.09;

fn handle_input(input: Res<Input<KeyCode>>, mut players: Query<&mut Transform, With<Player>>) {
    let mut player = players.single_mut();

    if input.pressed(KeyCode::A) {
        player.rotate(Quat::from_rotation_z(ROTATION_SPEED));
    } else if input.pressed(KeyCode::D) {
        player.rotate(Quat::from_rotation_z(-ROTATION_SPEED));
    }
}
