use bevy::prelude::*;
use bevy_psx::{camera::PsxCamera, material::PsxMaterial, PsxPlugin};

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxPlugin)
        .insert_resource(Msaa::Off)
        .add_systems(Startup,setup)
        .add_systems(Update,rotate)
        .run();
}


/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PsxMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PsxCamera::default());
    let transform =
    Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, 0.0, -1.0));
    
    commands.spawn((
        MaterialMeshBundle {
            mesh: asset_server.load("artefacts/psxgun.glb#Mesh0/Primitive0"),
            material: materials.add(PsxMaterial {
                color_texture: Some(asset_server.load("artefacts/psxgun.glb#Texture0")),
                // snap_amount: 40000000000.0,  
                snap_amount: 40.0,  
                fog_distance: Vec2::new(250.0, 750.0), 
                
                ..Default::default()
            }),
            transform,
            ..default()
        },
        Rotates,
    ));
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.95 * time.delta_seconds());
        // transform.scale = Vec3::splat(0.25);
        transform.rotate_x(0.3 * time.delta_seconds());
        transform.rotate_z(0.3 * time.delta_seconds());
    }
}
