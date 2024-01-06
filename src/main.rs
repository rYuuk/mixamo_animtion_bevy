use bevy::{gltf::Gltf, prelude::*};

#[derive(Resource)]
struct Animations(Handle<AnimationClip>);

#[derive(Resource)]
struct AnimationsCopy {
    anim: Handle<Gltf>,
    model: Handle<Gltf>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_systems(Startup, load_model_and_animation)
        .add_systems(OnEnter(AppState::Setup), check)
        .add_systems(OnEnter(AppState::Finished), finish)
        .add_systems(Update, run_animation)
        .run();
}

fn check(
    mut commands: Commands,
    animation_copy: Res<AnimationsCopy>,
    assets: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut events: EventReader<AssetEvent<Gltf>>,
) {
    let mut model_loaded = false;
    let mut anim_loaded = false;
    for event in events.read() {
        if event.is_loaded_with_dependencies(&animation_copy.model) {
            model_loaded = true;
        }

        if event.is_loaded_with_dependencies(&animation_copy.anim) {
            anim_loaded = true;

            commands.insert_resource(Animations(
                assets.get(&animation_copy.anim).unwrap().animations[0].clone(),
            ));
        }
    }

    if model_loaded && anim_loaded {
        next_state.set(AppState::Finished);
    }
}

fn finish(
    animation_copy: Res<AnimationsCopy>,
    mut assets: ResMut<Assets<Gltf>>,
    animation: Res<Animations>,
) {
    let model = assets.get_mut(&animation_copy.model).unwrap();
    let cloned_animation = animation.0.clone();
    model.animations = vec![cloned_animation];

    // Show the model with animation somehow ??
}

fn run_animation(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0.clone_weak()).repeat();
    }
}

fn load_model_and_animation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

      // light
      commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-1.0, 1.8, 3.0)
        .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    });

    // commands.insert_resource(Animations(
    //     asset_server.load("DanceAnimation.glb#Animation0"),
    // ));

    // commands.spawn((
    //     SceneBundle {
    //         scene: asset_server.load("Avatar.glb#Scene0"),
    //         ..default()
    //     },
    // ));

    let model: Handle<Gltf> = asset_server.load("Avatar.glb");
    let anim: Handle<Gltf> = asset_server.load("DanceAnimation.glb");

    commands.insert_resource(AnimationsCopy { anim, model });
}
