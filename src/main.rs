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
        .insert_resource(Gravity::from(Vec3::new(0., -200.0, 0.)))
        .run();
}

fn handle_collision(
    mut events: EventReader<CollisionEvent>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    for event in events.iter() {
        let (_, player_layer) = event.collision_layers();
        let mut player = players.single_mut();
        if event.is_started() && is_player(player_layer) {
            //player collides with floor
            let y = -player.linear.y / 1.1;
            player.linear.y = y;

            //TODO: find direction character is pointing
        }
    }
}

fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Player) && !layers.contains_group(Layer::World)
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
            half_extends: Vec2::new(500.0, 5.0).extend(0.0),
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
