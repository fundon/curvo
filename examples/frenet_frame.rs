use std::f64::consts::TAU;

use bevy::{
    prelude::*,
    render::mesh::{PrimitiveTopology, VertexAttributeValues},
    window::close_on_esc,
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};

use bevy_normal_material::plugin::NormalMaterialPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_points::plugin::PointsPlugin;
use materials::*;
use nalgebra::{Matrix4, Point3};

use curvo::prelude::*;
use rand::Rng;
mod materials;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LineMaterialPlugin)
        .add_plugins(InfiniteGridPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(PointsPlugin)
        .add_plugins(NormalMaterialPlugin)
        .add_plugins(AppPlugin)
        .run();
}
struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, close_on_esc);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let n = 12;
    let min_radius = 0.25;
    let max_radius = 2.5;
    let depth = 10.;

    let points: Vec<_> = (0..n)
        .map(|i| {
            let g = rng.gen::<f64>();
            let t = i as f64 / n as f64;
            let r = t * TAU;
            let rad = min_radius + g * (max_radius - min_radius);
            let x = r.cos() * rad;
            let z = r.sin() * rad;
            let y = depth * t;
            Point3::new(x, y, z)
        })
        .collect();
    let curve = NurbsCurve3D::try_interpolate(&points, 3, None, None).unwrap();

    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, default());
    let vertices = curve
        .tessellate(Some(1e-6))
        .iter()
        .map(|p| p.cast::<f32>())
        .map(|p| [p.x, p.y, p.z])
        .collect();
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(vertices),
    );
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: line_materials.add(LineMaterial {
                color: Color::WHITE,
            }),
            ..Default::default()
        })
        .insert(Name::new("curve"));

    let (start, end) = curve.knots_domain();
    let samples = 32;
    let span = (end - start) / ((samples - 1) as f64);
    let parameters: Vec<_> = (0..samples).map(|i| start + span * (i as f64)).collect();
    let frames = curve.compute_frenet_frames(&parameters);

    let size = 0.25;
    let hs = size * 0.5;
    let vertices = vec![
        [-hs, -hs, 0.],
        [hs, -hs, 0.],
        [hs, hs, 0.],
        [-hs, hs, 0.],
        [-hs, -hs, 0.],
    ];

    let mut tangents = vec![];
    let mut normals = vec![];
    let mut binormals = vec![];

    let length = 0.15;
    frames.iter().enumerate().for_each(|(i, frame)| {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, default());
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vertices.clone()),
        );

        /*
        let o = frame.position().cast::<f32>();
        let dir = frame.tangent().cast::<f32>();
        let up = frame.normal().cast::<f32>();
        let tr = Transform::from_translation(o.into()).looking_to(dir.into(), up.into());
        */

        let matrix: Matrix4<f64> = frame.matrix().into();
        let matrix = matrix.cast::<f32>();
        let tr = Transform::from_matrix(matrix.into());

        commands
            .spawn(MaterialMeshBundle {
                mesh: meshes.add(mesh),
                material: line_materials.add(LineMaterial {
                    color: Color::WHITE,
                }),
                transform: tr,
                ..Default::default()
            })
            .insert(Name::new(format!("frame_{}", i)));

        let p: Vec3 = frame.position().cast::<f32>().into();
        let t: Vec3 = frame.tangent().cast::<f32>().into();
        let n: Vec3 = frame.normal().cast::<f32>().into();
        let b: Vec3 = frame.binormal().cast::<f32>().into();
        tangents.push(p);
        tangents.push(p + t * length);
        normals.push(p);
        normals.push(p + n * length);
        binormals.push(p);
        binormals.push(p + b * length);
    });

    let add_arrows = |commands: &mut Commands<'_, '_>,
                      meshes: &mut ResMut<'_, Assets<Mesh>>,
                      line_materials: &mut ResMut<'_, Assets<LineMaterial>>,
                      vs: &Vec<Vec3>,
                      color: Color,
                      name: String| {
        commands
            .spawn(MaterialMeshBundle {
                mesh: meshes.add(
                    Mesh::new(PrimitiveTopology::LineList, default()).with_inserted_attribute(
                        Mesh::ATTRIBUTE_POSITION,
                        VertexAttributeValues::Float32x3(vs.iter().map(|v| v.to_array()).collect()),
                    ),
                ),
                material: line_materials.add(LineMaterial { color }),
                ..Default::default()
            })
            .insert(Name::new(name));
    };
    add_arrows(
        &mut commands,
        &mut meshes,
        &mut line_materials,
        &tangents,
        Color::RED,
        "t".to_string(),
    );
    add_arrows(
        &mut commands,
        &mut meshes,
        &mut line_materials,
        &normals,
        Color::YELLOW,
        "n".to_string(),
    );
    add_arrows(
        &mut commands,
        &mut meshes,
        &mut line_materials,
        &binormals,
        Color::GREEN,
        "b".to_string(),
    );

    let camera = Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 2.5, 10.)),
        ..Default::default()
    };
    commands.spawn((camera, PanOrbitCamera::default()));
    commands.spawn(InfiniteGridBundle::default());
}
