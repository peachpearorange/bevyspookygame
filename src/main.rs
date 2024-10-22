#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(const_trait_impl)]
// #![feature(type_alias_impl_trait)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![feature(variant_count)]
#![feature(strict_overflow_ops)]
#![feature(iter_intersperse)]
#![feature(trivial_bounds)]
#![feature(impl_trait_in_assoc_type)]
#![feature(option_get_or_insert_default)]
#![feature(let_chains)]
#![feature(const_mut_refs)]

// #![feature(int_roundings)]
// #![recursion_limit = "1024"]
// #![feature(const_fn_floating_point_arithmetic)]

pub mod bundletree;
// pub mod ui;

pub use bevy::prelude::Name;
use {avian3d::prelude::*,
     bevy::{app::AppExit,
            asset::{AssetServer, Handle},
            core_pipeline::{bloom::{BloomCompositeMode, BloomPrefilterSettings,
                                    BloomSettings},
                            Skybox},
            ecs::{entity::EntityHashMap, world::Command},
            math::{primitives, vec3, Vec3},
            pbr::{StandardMaterial, VolumetricFogSettings},
            prelude::*,
            render::{render_resource::TextureViewDescriptor,
                     texture::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor}},
            utils::{EntityHashSet, HashMap, HashSet},
            window::WindowMode},
     bevy_embedded_assets::*,
     bevy_mod_billboard::{BillboardDepth, BillboardLockAxis, BillboardMeshHandle,
                          BillboardTextBundle, BillboardTextureBundle,
                          BillboardTextureHandle},
     bevy_mod_picking::{prelude::{Highlight, HighlightKind},
                        PickableBundle},
     bevy_panorbit_camera::PanOrbitCamera,
     bevy_quill::{prelude::*, QuillPlugin, ViewChild},
     bevy_quill_overlays::QuillOverlaysPlugin,
     dynamics::solver::SolverConfig,
     enum_assoc::Assoc,
     fancy_constructor::new,
     rand::{random, thread_rng},
     rust_utils::*,
     std::f32::consts::PI};
// ui::UIData

pub const GLOWY_COLOR: Color = Color::srgb(13.99, 11.32, 50.0);
pub const GLOWY_COLOR_2: Color = Color::srgb(30.0, 20.7, 10.5);
pub const GLOWY_COLOR_3: Color = Color::srgb(0.0, 30.0, 0.0);
pub const EXPLOSION_COLOR: Color = Color::srgb(8.0, 3.0, 3.0);
pub const LASER_COLOR: Color = Color::hsv(60.0, 1.0, 4.0);
// hsv(61, 100%, 100%)
pub const BILLBOARD_REL_SCALE: f32 = 2.0;
pub const TEXT_SCALE: f32 = 0.013;
pub const ENABLE_SHADOWS_OTHER_THAN_SUN: bool = false;

#[derive(Assoc, Copy, Clone, Hash, Eq, PartialEq)]
#[func(pub const fn path(&self) -> &'static str)]
pub enum MySprite {
  #[assoc(path = "white_corners.png")]
  WhiteCorners,
  #[assoc(path = "note.png")]
  Note,
  #[assoc(path = "treemonster.png")]
  TreeMonster,
  #[assoc(path = "tent.png")]
  Tent,
  #[assoc(path = "torch.png")]
  Torch,
  #[assoc(path = "player.png")]
  Player,
  #[assoc(path = "spaceman.png")]
  SpaceMan,
  #[assoc(path = "spacecowboy.png")]
  SpaceCowBoy,
  #[assoc(path = "spacewizard.png")]
  SpaceWizard,
  #[assoc(path = "wormhole.png")]
  WormHole,
  #[assoc(path = "gate.png")]
  Gate,
  #[assoc(path = "turret.png")]
  Turret,
  #[assoc(path = "crystal_monster.png")]
  CrystalMonster,
  #[assoc(path = "container.png")]
  Container,
  #[assoc(path = "mushroom_man.png")]
  MushroomMan,
  #[assoc(path = "asteroid.png")]
  Asteroid,
  #[assoc(path = "icesteroid.png")]
  IceAsteroid,
  #[assoc(path = "crystal_asteroid.png")]
  CrystalAsteroid,
  #[assoc(path = "coin.png")]
  Coin,
  #[assoc(path = "space_cat.png")]
  SpaceCat,
  #[assoc(path = "spherical_cow.png")]
  SphericalCow,
  #[assoc(path = "zorp.png")]
  Zorp,
  #[assoc(path = "space_station.png")]
  SpaceStation,
  #[assoc(path = "ice_planet.png")]
  IcePlanet,
  #[assoc(path = "lava_planet.png")]
  LavaPlanet,
  #[assoc(path = "pixelc/habitableplanet.png")]
  HabitablePlanet,
  #[assoc(path = "pixelc/browngasgiant.png")]
  BrownGasGiant,
  #[assoc(path = "pixelc/marslikeplanet.png")]
  MarsLikePlanet,
  #[assoc(path = "sandplanet.png")]
  SandPlanet,
  #[assoc(path = "hpbox.png")]
  HPBox,
  #[assoc(path = "sign.png")]
  Sign,
  #[assoc(path = "floating_island.png")]
  FloatingIsland,
  #[assoc(path = "spacepiratebase.png")]
  SpacePirateBase,
  #[assoc(path = "spaceshipwhite.png")]
  SpaceshipWhite,
  #[assoc(path = "spaceshipblue.png")]
  SpaceshipBlue,
  #[assoc(path = "spaceshipred.png")]
  SpaceshipRed,
  #[assoc(path = "spaceshipdarkred.png")]
  SpaceshipDarkRed,
  #[assoc(path = "spaceshippurple.png")]
  SpaceshipPurple,
  #[assoc(path = "spaceshipabandoned.png")]
  SpaceshipAbandoned,
  #[assoc(path = "wizardspaceship.png")]
  WizardSpaceShip,
  #[assoc(path = "spaceshipgreen.png")]
  SpaceshipGreen,
  #[assoc(path = "purpleenemyship.png")]
  PurpleEnemyShip,
  #[assoc(path = "spaceshipwhite2.png")]
  SpaceshipWhite2,
  #[assoc(path = "stone.png")]
  Stone,
  #[assoc(path = "pixelc/bricks.png")]
  Bricks,
  #[assoc(path = "pixelc/chest.png")]
  Chest,
  #[assoc(path = "pixelc/block_textures.png")]
  BlockTextures,
  #[assoc(path = "sun.png")]
  Sun,
  #[assoc(path = "fire.png")]
  Fire,
  #[assoc(path = "iceberg.png")]
  Iceberg,
  #[assoc(path = "coffee.png")]
  Coffee,
  #[assoc(path = "stickman.png")]
  Stickman,
  #[assoc(path = "grass.png")]
  Grass,
  #[assoc(path = "water.png")]
  Water,
  #[assoc(path = "tree.png")]
  Tree,
  #[assoc(path = "snow.png")]
  Snow,
  #[assoc(path = "penguin.png")]
  Penguin,
  #[assoc(path = "pixelc/missile.png")]
  Missile,
  // #[assoc(path = "pixelc/torch.png")]
  // Torch,
  #[assoc(path = "nasa_starmap.jpeg")]
  NasaStarmap
}
#[derive(Assoc, Copy, Clone, Hash, Eq, PartialEq)]
#[func(pub fn val(&self, h: Handle<Image>) -> StandardMaterial)]
#[func(pub fn img(&self) -> MySprite)]
enum MyImageMaterial {
  #[assoc(img = MySprite::Snow)]
  #[assoc(val = StandardMaterial { perceptual_roughness: 0.4,
                                   metallic: 0.0,
                                   reflectance: 0.5,
                                   ior: 1.31,
                                   base_color_texture: Some(h),
                                   ..default() })]
  Snow,
  #[assoc(img = MySprite::Water)]
  #[assoc(val = StandardMaterial { perceptual_roughness: 0.3,
                                                      metallic: 0.0,
                                                      reflectance: 0.5,
                                                      base_color_texture:
                                                      Some(h),
                                                      ..default() })]
  Water,
  #[assoc(img = MySprite::Stone)]
  #[assoc(val = StandardMaterial { perceptual_roughness: 0.8,
                                                      metallic: 0.0,
                                                      reflectance: 0.3,
                                                      base_color_texture:
                                                      Some(h),
                                                      ..default() })]
  Stone,
  #[assoc(img = MySprite::Bricks)]
  #[assoc(val = StandardMaterial { perceptual_roughness: 0.95,
                                                      metallic: 0.0,
                                                      reflectance: 0.1,
                                                      base_color_texture:
                                   Some(h),
                                                      ..default() })]
  Bricks,
  #[assoc(img = MySprite::Grass)]
  #[assoc(val = StandardMaterial { perceptual_roughness: 0.8,
                                                      metallic: 0.0,
                                                      reflectance: 0.2,
                                                      base_color_texture:
                                   Some(h),
                                                      ..default() })]
  Grass,
  #[assoc(img = MySprite::Penguin)]
  #[assoc(val = StandardMaterial::from(h))]
  Penguin
}
#[derive(Assoc, Copy, Clone, Hash, Eq, PartialEq)]
#[func(pub fn val(&self) -> StandardMaterial)]
pub enum MyMaterial {
  #[assoc(val = StandardMaterial { unlit: true,
                                   alpha_mode: AlphaMode::Mask(0.0),
                                   ..GLOWY_COLOR.into() })]
  GlowyMaterial,
  #[assoc(val = StandardMaterial { unlit: true,
                                   alpha_mode: AlphaMode::Mask(0.0),
                                   ..GLOWY_COLOR_2.into() })]
  GlowyMaterial2,
  #[assoc(val = StandardMaterial { unlit: true,
                                   alpha_mode: AlphaMode::Mask(0.0),
                                   ..GLOWY_COLOR_3.into() })]
  GlowyMaterial3,
  #[assoc(val = StandardMaterial { unlit: true,
                                   alpha_mode: AlphaMode::Mask(0.0001),
                                   ..EXPLOSION_COLOR.into() })]
  ExplosionMaterial,
  #[assoc(val = StandardMaterial { unlit: true,
                                   alpha_mode: AlphaMode::Mask(0.0001),
                                   ..LASER_COLOR.into() })]
  LaserMaterial,
  #[assoc(val = StandardMaterial::from(Color::srgb(0.2, 0.7, 0.9)))]
  ParticleMaterial,
  #[assoc(val = StandardMaterial { unlit: true,
                                   alpha_mode: AlphaMode::Blend,
                                   ..Color::srgba(0.0, 0.0, 0.0, 0.0).into() })]
  InvisibleMaterial,
  #[assoc(val = StandardMaterial { unlit: true,
                                   alpha_mode: AlphaMode::Blend,
                                   ..Color::srgba(0.0, 0.3, 1.0, 0.1).into() })]
  HoveredMaterial,
  #[assoc(val = StandardMaterial { unlit: true,
                                   alpha_mode: AlphaMode::Blend,
                                   ..Color::srgba(0.0, 0.3, 1.0, 0.3).into() })]
  PressedMaterial,
  #[assoc(val = StandardMaterial { unlit: true,
                                   alpha_mode: AlphaMode::Blend,
                                   ..Color::srgba(0.0, 0.3, 1.0, 0.2).into() })]
  SelectedMaterial
}
#[derive(Assoc, Copy, Clone, Hash, Eq, PartialEq)]
#[func(pub const fn path_and_label(&self) -> (&'static str,&'static str))]
pub enum MyScene {
  #[assoc(path_and_label = ("lunarlander.glb", "Scene0"))]
  LunarLander,
  #[assoc(path_and_label = ("character_controller_demo.glb", "Scene0"))]
  CharacterControllerDemo,
  #[assoc(path_and_label = ("level.glb", "Scene0"))]
  Level,
  #[assoc(path_and_label = ("alevel.gltf", "Scene0"))]
  ALevel,
  #[assoc(path_and_label = ("this_here_level.glb", "Scene0"))]
  IslandLevel,
  #[assoc(path_and_label = ("somesketchlevel.glb", "Scene0"))]
  SomeSketchLevel,
  #[assoc(path_and_label = ("snowman.glb", "Scene0"))]
  Snowman,
  #[assoc(path_and_label = ("coffee.glb", "Scene0"))]
  CoffeeScene,
  #[assoc(path_and_label = ("goxel_level.glb", "Scene0"))]
  GoxelLevel,
  #[assoc(path_and_label = ("turtle level.gltf", "Scene0"))]
  TurtleLevel,
  #[assoc(path_and_label = ("wat.glb", "Scene0"))]
  Wat
}
// struct GenMesh2(pub fn() -> Mesh);
// const CUBE: GenMesh2 = GenMesh2(|| Cuboid::new(0.7, 0.7, 0.7).into());
#[derive(Assoc, Copy, Clone, Hash, Eq, PartialEq)]
#[func(pub fn gen(&self) -> Mesh)]
pub enum GenMesh {
  #[assoc(gen = Cuboid::new(1.0, 1.0, 1.0).into())]
  UnitCube,
  #[assoc(gen = primitives::Cylinder::new(1.0, 1.0).into())]
  UnitCylinder,
  #[assoc(gen = Cuboid::new(0.7, 0.7, 0.7).into())]
  Cube,
  #[assoc(gen = Cuboid::new(2.0, 1.0, 1.0).into())]
  BoxMesh,
  #[assoc(gen = Cuboid::new(2.1, 0.3, 2.1).into())]
  FlatBox,
  #[assoc(gen = primitives::Capsule3d::default().into())]
  Capsule,
  #[assoc(gen = primitives::Torus::default().into())]
  Torus,
  #[assoc(gen = primitives::Sphere { radius: 1.0 }.into())]
  Sphere,
  #[assoc(gen = Cuboid::new(25.0, 0.1, 25.0).into())]
  PlaneSize50,
  #[assoc(gen = primitives::Rectangle::new(BILLBOARD_REL_SCALE, BILLBOARD_REL_SCALE).into())]
  BillboardMeshSquare
}

#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct Visuals {
  text: Option<String>,
  material_mesh: Option<(MyMaterial, GenMesh)>,
  shield_active: bool,
  sprite: Option<MySprite>,
  unlit: bool,
  targeted: bool,
  done: bool
}

impl Visuals {
  fn none() -> Self { default() }
  fn sprite(sprite: MySprite) -> Self {
    Self { sprite: Some(sprite),
           ..default() }
  }
  fn unlit_sprite(sprite: MySprite) -> Self {
    Self { sprite: Some(sprite),
           unlit: true,
           ..default() }
  }
  fn material_mesh(material: MyMaterial, mesh: GenMesh) -> Self {
    Self { material_mesh: Some((material, mesh)),
           ..default() }
  }
  fn material_sphere(material: MyMaterial) -> Self {
    Self::material_mesh(material, GenMesh::Sphere)
  }
  fn with_text(self, text: impl ToString) -> Self {
    Self { text: Some(text.to_string()),
           ..self }
  }
}
pub fn set_visuals(mut visuals_q: Query<(Entity, &mut Visuals)>,
                   mut player_q: Query<&Player>) {
  // if let Ok(player) = player_q.get_single() {
  //   for (e, mut visuals) in &mut visuals_q {
  //     let should_have_target = player.target() == Some(e);
  //     // let new_visuals = Visuals{targeted:should_have_target,}
  //     let has_target = visuals.as_ref().targeted;
  //     if has_target != should_have_target {
  //       visuals.targeted = should_have_target;
  //     }
  //   }
  // }
}

// #[derive(Component)]
// pub struct Billboard {
//   pub transform: Transform,
//   pub image_handle: Handle<Image>,
//   pub unlit: bool
// }
// pub fn gib_billboard(mut sprite_3d_params: Sprite3dParams,
//                      mut c: Commands,
//                      q: Query<(Entity, &Billboard)>) {
//   for (e,
//        Billboard { transform,
//                    image_handle,
//                    unlit }) in &q
//   {
//     if let Some(image) = sprite_3d_params.images.get(image_handle.clone()) {
//       c.entity(e)
//        .remove::<Billboard>()
//        .insert(bevy_sprite3d::Sprite3d { image: image_handle.clone(),
//                                          transform: *transform,
//                                          pixels_per_metre: image.height() as f32,
//                                          double_sided: true,
//                                          unlit: false,
//                                          ..default() }.bundle(&mut sprite_3d_params));
//     }
//   }
// }
#[derive(Component, Clone)]
pub struct VisualSprite;

pub fn visuals(camq: Query<&GlobalTransform, With<Camera3d>>,
               serv: Res<AssetServer>,
               mut c: Commands,
               mut n: Local<u32>,
               mut visuals_q: Query<(Entity, Mut<Visuals>)>,
               mut visuals_sprites_q: Query<(&mut Transform, &GlobalTransform),
                     With<VisualSprite>>,
               mut sprite_3d_params: bevy_sprite3d::Sprite3dParams,
               mut sprite_handles: Local<HashMap<MySprite, Handle<Image>>>,
               mut mesh_handles: Local<HashMap<GenMesh, Handle<Mesh>>>,
               mut material_handles: Local<HashMap<MyMaterial, Handle<StandardMaterial>>>,
               mut visual_child_entities: Local<HashMap<Entity, Entity>>) {
  let mut get_material_handle = |material: MyMaterial| {
    material_handles.entry(material)
                    .or_insert_with(|| serv.add(material.val()))
                    .clone()
  };

  let mut get_mesh_handle = |mesh: GenMesh| {
    mesh_handles.entry(mesh)
                .or_insert_with(|| serv.add(mesh.gen()))
                .clone()
  };

  let mut get_sprite_handle = |sprite: MySprite| {
    sprite_handles.entry(sprite)
                  .or_insert_with(|| serv.load(format!("embedded://{}", sprite.path())))
                  .clone()
  };

  let text_style = TextStyle { font_size: 30.0,
                               ..default() };
  let invisible_material = get_material_handle(MyMaterial::InvisibleMaterial);

  for (entity, mut visuals) in &mut visuals_q {
    if visuals.is_changed() || !visuals.done {
      visuals.done = true;
      *n += 1;
      if *n % 100 == 0 {
        println!("{}", *n);
      }

      let main_visual_child = *visual_child_entities.entry(entity).or_insert_with(|| {
                                                                    c.spawn((
                    PbrBundle {
                        material: invisible_material.clone(),
                        mesh: get_mesh_handle(GenMesh::Sphere),
                        ..default()
                    },
                ))
                .set_parent(entity)
                .id()
                                                                  });

      c.entity(main_visual_child).despawn_descendants();

      if let Some(text) = visuals.text.clone() {
        c.spawn(Text2dBundle {
                    text: Text::from_section(text, text_style.clone()),
                    transform: Transform::from_xyz(0.0, 1.5, 0.0).with_scale(Vec3::splat(0.07)),
                    ..default()
                })
                .set_parent(main_visual_child);
      }

      if let Some(sprite) = visuals.sprite {
        let sprite_handle = get_sprite_handle(sprite);
        // sprite_3d_params.images.get(image_handle.clone())
        if let Some(image) = sprite_3d_params.images.get(&sprite_handle) {
          let image_height = image.height();
          c.spawn((VisualSprite,
                   bevy_sprite3d::Sprite3d { image: sprite_handle,
                                             pixels_per_metre: image_height as f32,
                                             double_sided: true,
                                             unlit: visuals.unlit,
                                             transform: Transform::from_xyz(0.0, 0.0,
                                                                            0.0),
                                             ..default() }.bundle(&mut sprite_3d_params)))
           .set_parent(main_visual_child);
        } else {
          visuals.done = false;
        }
      }

      if visuals.targeted {
        let target_overlay = get_sprite_handle(MySprite::WhiteCorners);
        c.spawn((bevy_sprite3d::Sprite3d { image: target_overlay,
                                           pixels_per_metre: 100.0,
                                           double_sided: true,
                                           unlit: true,
                                           transform:
                                             Transform::from_scale(Vec3::splat(1.7)),
                                           ..default() }.bundle(&mut sprite_3d_params),))
         .set_parent(main_visual_child);
      }

      if let Some((material, gen_mesh)) = visuals.material_mesh {
        let material = get_material_handle(material);
        let mesh = get_mesh_handle(gen_mesh);
        c.spawn(PbrBundle { material,
                            mesh,
                            ..default() })
         .set_parent(main_visual_child);
      }
    }
  }
}
#[derive(Component, Clone)]
struct FaceCamera;
pub fn face_camera(camq: Query<&GlobalTransform, With<Camera3d>>,
                   mut camera_facers_q: Query<(&mut Transform, &GlobalTransform),
                         (With<FaceCamera>, Without<Camera3d>)>) {
  if let Ok(cam_globaltransform) = camq.get_single() {
    for (mut transform, globaltransform) in &mut camera_facers_q {
      let dir = Vec3 { y: 0.0,
                       ..(globaltransform.translation()
                          - cam_globaltransform.translation()) };
      transform.look_to(dir, Vec3::Y);
    }
  }
}
#[derive(Clone, Debug)]
pub struct PlayerTargetInteractionState {
  target: Entity,
  approaching: bool,
  shooting: bool,
  in_dialogue: bool
}

impl Player {}

#[derive(Component)]
struct PlayerFlashlight;
#[derive(Component)]
struct PlayerAmbientlight;
fn toggle_flashlight(mut c: Commands,
                     mut playerq: Query<&mut Player>,
                     mut flashlightq: Query<&mut Visibility, With<PlayerFlashlight>>,
                     keyboard_input: Res<ButtonInput<KeyCode>>) {
  if let Ok(mut player) = playerq.get_single_mut()
     && keyboard_input.just_pressed(KeyCode::KeyF)
  {
    for mut flashlight_visibility in &mut flashlightq {
      player.light_on = !player.light_on;
      println("toggled flashlight");
      *flashlight_visibility = match *flashlight_visibility {
        Visibility::Inherited => Visibility::Hidden,
        Visibility::Hidden => Visibility::Visible,
        Visibility::Visible => Visibility::Hidden
      }
    }
  }
}
#[derive(Component)]
struct OriginTime(u32);
fn origin_time(q: Query<Entity, Without<OriginTime>>,
               time_ticks: Res<TimeTicks>,
               mut c: Commands) {
  for e in &q {
    c.entity(e).insert(OriginTime(time_ticks.0));
  }
}
// fn filter_least_map<O: Ord + Clone, T, R>(f: impl Fn(T) -> Option<(R, O)>,
//                                           coll: impl IntoIterator<Item = T>)
//                                           -> Option<R> {
//   coll.into_iter()
//       .filter_map(f)
//       .min_by_key(|(_, o)| o.clone())
//       .map(|(r, _)| r)
// }

// fn filter_least<O: Ord + Clone, T>(f: impl Fn(&T) -> Option<O>,
//                                    coll: impl IntoIterator<Item = T>)
//                                    -> Option<T> {
//   filter_least_map(|t| f(&t).map(|v| (t, v)), coll)
// }
// fn filter_most_map<O: Ord + Clone, T, R>(f: impl Fn(T) -> Option<(R, O)>,
//                                          coll: impl IntoIterator<Item = T>)
//                                          -> Option<R> {
//   coll.into_iter()
//       .filter_map(f)
//       .max_by_key(|(_, o)| o.clone())
//       .map(|(r, _)| r)
// }
// fn filter_most<O: Ord + Clone, T>(f: impl Fn(&T) -> Option<O>,
//                                   coll: impl IntoIterator<Item = T>)
//                                   -> Option<T> {
//   filter_most_map(|t| f(&t).map(|v| (t, v)), coll)
// }

// #[derive(Component, Default, Clone)]
// pub struct Container(pub HashSet<Entity>);
// impl Container {
//   pub fn empty() -> Container { Container::default() }
// }
pub fn name(s: &'static str) -> Name { Name::new(s) }
#[derive(Component, Clone)]
pub struct TimedAnimation {
  pub num_frames: usize,
  pub time_per_frame_in_ticks: usize
}
// #[derive(Component, Clone)]
// pub struct PlayerFollower;
pub fn pick<T>(coll: impl IntoIterator<Item = T>) -> Option<T> {
  rand::seq::IteratorRandom::choose(coll.into_iter(), &mut thread_rng())
}
fn avg<T: std::iter::Sum + std::ops::Div<f32, Output = T>>(coll: impl IntoIterator<Item = T>)
                                                           -> Option<T> {
  let v = vec(coll);
  let n = v.len();
  let s = v.into_iter().sum::<T>();
  (n != 0).then(|| s / (n as f32))
}
pub fn capsule_from_height_and_radius(height: f32, radius: f32) -> Collider {
  Collider::capsule(height - (radius * 2.0), radius)
}
#[derive(Component, Clone, Default)]
pub struct SpaceObject {
  pub scale: f32,
  pub click_target_entity: Option<Entity>
}
#[derive(Component, Clone)]
pub struct ClickTarget;
pub fn click_target(mut parent_q: Query<&Parent>,
                    mut click_events: EventReader<bevy_mod_picking::events::Pointer<bevy_mod_picking::events::Click>>,
                    mut player_q: Query<&mut Player>) {
  if let Ok(mut player) = player_q.get_single_mut() {
    for event in click_events.read() {
      println(debugfmt(event));
      let mut root_entity = event.target;
      while let Ok(parent) = parent_q.get(root_entity) {
        root_entity = parent.get();
      }
      println!("Player target set to {root_entity}");
    }
  }
}
// type ClickTargetChild = (PbrBundle,
//                          NotShadowCaster,
//                          NotShadowReceiver,
//                          Highlight<StandardMaterial>,
//                          PickableBundle);

fn camera_follow_player(mut camq: Query<&mut PanOrbitCamera>,
                        playerq: Query<&Transform, With<Player>>) {
  if let Ok(player_transform) = playerq.get_single()
     && let Ok(mut cam) = camq.get_single_mut()
  {
    cam.target_focus = player_transform.translation;
  }
}
#[derive(Component, Debug, Clone, Copy, new)]
pub struct Navigation {
  max_speed: f32,
  #[new(default)]
  navigation_kind: NavigationKind
}

#[derive(Default, Debug, Clone, Copy)]
enum NavigationKind {
  #[default]
  None,
  // Dir2(Dir2),
  Vec2(Vec2),
  Pos(Vec2),
  Chase(Entity) // ChaseAtRange(Entity, f32)
}

fn navigation(mut navigators_q: Query<(&Navigation,
                     &Transform,
                     &mut ExternalForce,
                     &mut LinearVelocity)>,
              chase_targets_q: Query<&Transform>,
              time: Res<Time>) {
  let to_vec3 = |Vec2 { x, y }| Vec3 { x, y: 0.0, z: y };
  let to_vec2 = |Vec3 { x, y, z }| Vec2 { x, y: z };
  for (nav, transform, mut force, mut velocity) in navigators_q.iter_mut() {
    let linvelnew =
      nav.max_speed
      * match nav.navigation_kind {
        NavigationKind::None => default(),
        NavigationKind::Vec2(vec2) => to_vec3(vec2.normalize_or_zero()),
        NavigationKind::Pos(vec2) => {
          (to_vec3(vec2) - transform.translation).normalize_or_zero()
        }
        NavigationKind::Chase(entity) => {
          (chase_targets_q.get(entity)
                          .map(|t| {
                            (t.translation - transform.translation).normalize_or_zero()
                          })
                          .unwrap_or_default())
        }
      };
    velocity.0 = linvelnew;
  }
}

fn monster_movement(mut monsterq: Query<(&mut Navigation, &Monster, &Transform)>,
                    playerq: Query<(Entity, &Player, &Transform)>,
                    time: Res<TimeTicks>) {
  if let Ok((player_entity, player, player_transform)) = playerq.get_single() {
    for (mut monsternav, monster, monster_transform) in &mut monsterq {
      let dist = player_transform.translation
                                 .distance(monster_transform.translation);

      if (dist < MONSTER_SEE_DARK_RANGE)
         || (player.light_on && (dist < MONSTER_SEE_LIT_RANGE))
      {
        *monsternav = Navigation { max_speed: MONSTER_MAX_SPEED_CHASE,
                                   navigation_kind: NavigationKind::Chase(player_entity) };
      } else {
        if time.0 % 600 == 0 {
          let dir = random::<Dir2>().as_vec2().normalize_or_zero();
          *monsternav = Navigation { max_speed: MONSTER_MAX_SPEED,
                                     navigation_kind: NavigationKind::Vec2(dir) };
        }
      }
    }
  }
}
pub fn player_movement(keyboard_input: Res<ButtonInput<KeyCode>>,
                       camera_query: Query<&Transform, With<Camera3d>>,
                       mut player_query: Query<&mut Navigation, With<Player>>) {
  let Ok(camera_transform) = camera_query.get_single() else {
    return;
  };
  let Ok(mut navigation) = player_query.get_single_mut() else {
    return;
  };
  let forward = Vec2::new(camera_transform.forward().x, camera_transform.forward().z).normalize_or_zero();
  let right = Vec2::new(-forward.y, forward.x);
  let movement_direction =
    Vec2::new((keyboard_input.pressed(KeyCode::KeyD) as i32
               - keyboard_input.pressed(KeyCode::KeyA) as i32) as f32,
              (keyboard_input.pressed(KeyCode::KeyW) as i32
               - keyboard_input.pressed(KeyCode::KeyS) as i32) as f32).normalize_or_zero();
  let world_space_direction =
    (movement_direction.x * right + movement_direction.y * forward).normalize_or_zero();
  navigation.navigation_kind = NavigationKind::Vec2(world_space_direction);
}
#[derive(Bundle)]
struct SceneSpaceObjectBundle((Handle<Scene>, CharacterBundle));
impl SceneSpaceObjectBundle {
  fn new(translation: Vec3, scale: f32, can_move: bool, scene: Handle<Scene>) -> Self {
    Self((scene,
          CharacterBundle::new(translation, can_move, Visuals::sprite(MySprite::Coffee))))
  }
}
#[derive(Default, Resource)]
pub struct TimeTicks(pub u32);
pub fn increment_time(mut time: ResMut<TimeTicks>) { time.0 += 1; }
pub fn timed_animation_system(time_ticks: Res<TimeTicks>,
                              mut q: Query<(&TimedAnimation, &mut TextureAtlas)>) {
  for (&TimedAnimation { num_frames,
                         time_per_frame_in_ticks },
       mut atlas) in &mut q
  {
    let time = time_ticks.0 as usize;
    let index = |time| (time / time_per_frame_in_ticks) % num_frames;
    let old_index = index(time.saturating_sub(1));
    let new_index = index(time);
    if new_index != old_index {
      atlas.index = new_index;
    }
  }
}

fn close_on_esc(mut exit: EventWriter<AppExit>, keyboard_input: Res<ButtonInput<KeyCode>>) {
  if keyboard_input.just_pressed(KeyCode::Escape) {
    exit.send(AppExit::Success);
  }
}

fn namefmt(oname: Option<&Name>) -> String {
  match oname {
    Some(name) => name.to_string(),
    None => "unnamed entity".to_string()
  }
}

comment! {}

pub fn intersperse_newline<T: ToString>(coll: impl IntoIterator<Item = T>) -> String {
  concat_strings(coll.into_iter()
                     .map(|v| v.to_string())
                     .intersperse("\n".to_string()))
}
#[derive(Resource, Default, Clone)]
pub struct UIData {
  pub current_time_ticks: u32,
  pub interact_message: Option<String>,
  pub note: Vec<String>,
  pub infobox_data: Vec<String>
}

const UI_BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);
const UI_BORDER_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
pub fn common_style(sb: &mut StyleBuilder) {
  sb.font_size(32.0)
    .display(Display::Block)
    .border(1)
    .border_color(UI_BORDER_COLOR)
    .background_color(UI_BACKGROUND_COLOR)
    .position(bevy::ui::PositionType::Absolute)
    .padding(3)
    .pointer_events(false);
}
#[derive(Clone, PartialEq)]
pub struct UIPopup {
  pub style: fn(&mut StyleBuilder),
  pub display_text_fn: fn(&UIData) -> Vec<String>
}
impl ViewTemplate for UIPopup {
  type View = impl View;
  fn create(&self, cx: &mut Cx) -> Self::View {
    let &Self { display_text_fn,
                style } = self;
    let uidata = cx.use_resource::<UIData>();
    let display_text = display_text_fn(uidata);
    Element::<NodeBundle>::new().style((common_style, style))
                                .children(intersperse_newline(display_text.clone()))
  }
}
// const MESSAGE_SHOW_TIME_TICKS: u32 = 600;
fn ui(mut c: Commands,
      // camq: Query<(Entity, &GlobalTransform), With<Camera3d>>,
      playerq: Query<(Entity, &Player, &Transform)>,
      mut ui_data: ResMut<UIData>,
      time: Res<TimeTicks>,
      view_root_q: Query<Entity, With<ViewRoot>>) {
  if view_root_q.is_empty() {
    let note_popup = UIPopup { style: |sb| {
                                 sb.justify_self(JustifySelf::Center)
                                   .top(Val::Percent(50.0));
                               },
                               display_text_fn: |uidata| uidata.note.clone() };
    let infobox = UIPopup { style: |sb| {
                              sb.left(0).top(0);
                            },
                            display_text_fn: |uidata| uidata.infobox_data.clone() };
    let root = vec![ViewChild::new(note_popup), ViewChild::new(infobox),];
    c.spawn(root.to_root());
  }
  if let Ok((player_entity, player, player_transform)) = playerq.get_single() {
    let player_pos = player_transform.translation;
    let infobox_data = map(ToString::to_string, [format!("{:.1}", player_pos).as_str(),
                                                 format!("you've found {} notes",
                                                         player.notes_found).as_str(),
                                                 "w,a,s,d: move",
                                                 "f: toggle flashlight"]) // .chain(map(|(item, n)| {
                                                                         //              format!("{} {:?}s", n, item)
                                                                         //            },
                                                                         //            player_inventory.0.clone()))
                                                                         .collect();

    let current_time = time.0;
    let current_time_ticks = current_time;
    // let message_log = rust_utils::filterv(|Message { string, time }| {
    //                                         time + MESSAGE_SHOW_TIME_TICKS > current_time
    //                                       },
    //                                       ui_data.message_log.clone());

    let old_ui_data = ui_data.clone();
    *ui_data = UIData { // target_data,
                        // overview_data,
                        current_time_ticks,
                        // message_log,
                        infobox_data,
                        ..old_ui_data };
    // .as_mut().update(|old_ui_data| UIData { target_data,
    //                                              overview_data,
    //                                              current_time_ticks,
    //                                              message_log,
    //                                              infobox_data,
    //                                              ..old_ui_data });
  }
}
pub fn string(t: impl ToString) -> String { t.to_string() }
#[derive(Component, Clone, Default)]
struct CanBeFollowedByNPC;

pub const BLOOM_SETTINGS: BloomSettings =
  BloomSettings { intensity: 0.5,
                  low_frequency_boost: 0.0,
                  prefilter_settings: BloomPrefilterSettings { threshold: 2.2,
                                                               threshold_softness: 0.0 },

                  composite_mode: BloomCompositeMode::Additive,
                  ..BloomSettings::NATURAL };

const TONEMAPPING: bevy::core_pipeline::tonemapping::Tonemapping =
  bevy::core_pipeline::tonemapping::Tonemapping::BlenderFilmic;

const FOG_SETTINGS: FogSettings =
  FogSettings { color: Color::srgb(0.25, 0.25, 0.25),
                falloff: FogFalloff::ExponentialSquared { density: 0.5 },
                directional_light_color: Color::NONE,
                directional_light_exponent: 8.0 };

pub const AMBIENT_LIGHT: AmbientLight = AmbientLight { color: Color::WHITE,
                                                       brightness: 10.0 };
const PLAYER_LIGHT_FLASHLIGHT: SpotLight =
  SpotLight { color: Color::WHITE,
              intensity: 600_000.0,
              range: 17.0,
              radius: 0.0,
              shadows_enabled: true,
              shadow_depth_bias: SpotLight::DEFAULT_SHADOW_DEPTH_BIAS,
              shadow_normal_bias: SpotLight::DEFAULT_SHADOW_NORMAL_BIAS,
              inner_angle: 0.3,
              outer_angle: 0.7 };
const TORCH_LIGHT: PointLight =
  PointLight { color: Color::hsv(33.0, 1.0, 0.5),
               intensity: 60_000.0,
               radius: 0.0,
               range: 3.8,
               shadows_enabled: true,
               shadow_depth_bias: PointLight::DEFAULT_SHADOW_DEPTH_BIAS / 10.0,
               shadow_normal_bias: PointLight::DEFAULT_SHADOW_NORMAL_BIAS / 10.0 };
const PLAYER_LIGHT_AMBIENT: PointLight =
  PointLight { color: Color::WHITE,
               intensity: 9_000.0,
               radius: 0.0,
               range: 16.0,
               shadows_enabled: false,
               shadow_depth_bias: PointLight::DEFAULT_SHADOW_DEPTH_BIAS / 10.0,
               shadow_normal_bias: PointLight::DEFAULT_SHADOW_NORMAL_BIAS / 10.0 };
const PLAYER_MAX_SPEED: f32 = 3.0;
const MONSTER_MAX_SPEED_CHASE: f32 = 2.0;
const MONSTER_MAX_SPEED: f32 = 1.0;
const PLAYER_INTERACTION_RANGE: f32 = 3.0;
const MONSTER_CATCH_RANGE: f32 = 1.5;
const MONSTER_SEE_DARK_RANGE: f32 = 5.0;
const MONSTER_SEE_LIT_RANGE: f32 = 12.0;

#[derive(Component, new)]
struct Proximal {
  distance: f32
}
#[derive(Component)]
struct Note;
#[derive(Component, Clone, Debug, Default)]
pub struct Player {
  notes_found: usize,
  light_on: bool
}
fn proximity_system(mut c: Commands,
                    mut player_query: Query<(Entity,
                           &mut Player,
                           &Transform,
                           &mut LockedAxes)>,
                    proximal_entities: Query<(Entity, &Transform, &Proximal),
                          Without<Player>>,
                    monsterq: Query<(Entity, &Monster)>,
                    noteq: Query<(Entity, &Note)>,
                    flashlightq: Query<Entity, With<PlayerFlashlight>>,
                    playerambientlightq: Query<Entity, With<PlayerAmbientlight>>,
                    mut prev_proximal_entities: Local<HashSet<Entity>>) {
  if let Ok((player_entity, mut player, player_transform, mut player_lockedaxes)) =
    player_query.get_single_mut()
  {
    let player_pos = player_transform.translation;
    let current_proximal_entities: HashSet<Entity> =
      filter_map(|(e, &Transform { translation, .. }, &Proximal { distance })| {
                   (translation.distance(player_pos) < distance).then_some(e)
                 },
                 &proximal_entities).collect();
    let new_proximal_entities =
      HashSet::difference(&current_proximal_entities, &prev_proximal_entities);
    let is_note = |e| noteq.get(e).is_ok();
    let is_monster = |e| monsterq.get(e).is_ok();
    for &e in &prev_proximal_entities {}
    for &e in new_proximal_entities {
      println("new proximal entity");
      if is_monster(e) {
        // monster got player
        c.entity(player_entity)
         .remove::<LockedAxes>()
         .remove::<FaceCamera>();

        for e in &playerambientlightq {
          c.entity(e).remove_parent_in_place();
        }
        for e in &flashlightq {
          c.entity(e).remove_parent_in_place();
        }
        println("monster got player");
      }
      if is_note(e) {
        // found note
        println("found note");
        let note = NOTES[player.notes_found];
        player.notes_found += 1;
      }
    }
    *prev_proximal_entities = current_proximal_entities;
  }
}

const TILE_SIZE: f32 = 1.0;
const CHARACTER_HEIGHT: f32 = TILE_SIZE;
const CHARACTER_RADIUS: f32 = CHARACTER_HEIGHT * 0.5;
#[derive(Bundle, Clone)]
pub struct CharacterBundle((Visuals,
                             FaceCamera,
                             LockedAxes,
                             ColliderMassProperties,
                             Collider,
                             RigidBody,
                             Friction,
                             // LinearDamping,
                             // AngularDamping,
                             LinearVelocity,
                             AngularVelocity,
                             ExternalForce,
                             ExternalImpulse,
                             SpatialBundle));
impl CharacterBundle {
  fn new(translation: Vec3, can_move: bool, visuals: Visuals) -> Self {
    let cube_mesh = Cuboid::default().mesh().build();
    let cube_collider = Cuboid::default().collider();
    let cylinder_collider = Cylinder::new(CHARACTER_RADIUS, CHARACTER_HEIGHT).collider();
    let sphere_collider = Sphere::new(1.0).collider();
    // capsule_from_height_and_radius
    let capsule_collider =
      Capsule3d::new(CHARACTER_RADIUS, CHARACTER_HEIGHT - CHARACTER_RADIUS * 2.0).collider();
    // Friction::ZERO
    // let mesh = Capsule3d::new(CHARACTER_RADIUS, CHARACTER_RADIUS + CHARACTER_HEIGHT).collider()
    // let mesh = Capsule3d::new(CHARACTER_RADIUS, CHARACTER_RADIUS + CHARACTER_HEIGHT).mesh()
    //                                                                                 .build();
    // let collider = Collider::convex_hull_from_mesh(&mesh).unwrap();
    // let collider = Collider::convex_hull_from_mesh(&cube_mesh).unwrap();
    let collider = capsule_collider;
    // let collider = capsule_from_height_and_radius(CHARACTER_HEIGHT, CHARACTER_RADIUS);
    // FogSettings
    Self((visuals,
          FaceCamera,
          LockedAxes::ROTATION_LOCKED,
          // LockedAxes::new().lock_rotation_x().lock_rotation_z(),
          ColliderMassProperties::new(&collider, 1.0),
          collider,
          if can_move {
            RigidBody::Dynamic
          } else {
            RigidBody::Static
          },
          Friction::ZERO,
          // LinearDamping(1.6),
          // AngularDamping(1.2),
          LinearVelocity::default(),
          AngularVelocity::default(),
          ExternalForce::default().with_persistence(false),
          ExternalImpulse::default(),
          SpatialBundle { transform: Transform { translation,
                                                 ..default() },
                          ..default() }))
  }
  fn sprite(translation: Vec3, scale: f32, can_move: bool, sprite: MySprite) -> Self {
    Self::new(translation, can_move, Visuals::sprite(sprite))
  }
}

pub fn from<B, A: From<B>>(b: B) -> A { A::from(b) }

fn rangerand(lo: f32, hi: f32) -> f32 { lo.lerp(hi, rand::random::<f32>()) }
fn random_normalized_vector() -> Vec3 { random::<Quat>() * Vec3::X }
fn prob(p: f32) -> bool { p > rand::random::<f32>() }

use bevy_mesh_terrain::{terrain_config::TerrainConfig, TerrainMeshPlugin};
const NOTES:&[&'static str] = &[
  "Diary entry 1: me and my friends are camping in this forest. I heard some strange sounds. could be a bear. yikes.",
  "Diary entry 2: cant find my friend.",
  "Diary entry 3: been looking for him. hear more strange sounds",
  "Diary entry 4: i swear i saw a tree wink at me",
  "Diary entry 5: oh my god the trees are chasing me!",
  "Diary entry 6: they got me. I'm turning into a tree. it's over",
];
#[derive(Component)]
struct Monster;
const WORLD_MAP: &[&'static str] = &["wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww",
                                     "w   n       t      t                          w",
                                     "w                                n            w",
                                     "w      t  l    t         t                t   w",
                                     "w                                             w",
                                     "w     g     t    n             g              w",
                                     "w                   T    T  t       t         w",
                                     "w      t                                      w",
                                     "w                 T   n     T                 w",
                                     "w   n          t         t              n     w",
                                     "w                                             w",
                                     "w        t         t  T        g              w",
                                     "w t                                   t       w",
                                     "w t                                   t       w",
                                     "w t                                   t       w",
                                     "w t                                   t       w",
                                     "w        t     n     t      t                 w",
                                     "w                                             w",
                                     "w t    t                         n      p     w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                         l   w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w                                             w",
                                     "w      t        t       t            t        w",
                                     "w        t     n     t      t                 w",
                                     "w                                             w",
                                     "w t    t    l                    n            w",
                                     "w                                             w",
                                     "w      t        t       t            t        w",
                                     "w        t     n     t      t                 w",
                                     "w                                             w",
                                     "w t    t                         n            w",
                                     "w                                             w",
                                     "w      t        t       t            t        w",
                                     "w        t     n     t      t                 w",
                                     "w                                             w",
                                     "w t    t                         n            w",
                                     "w                                             w",
                                     "w      t        t       t            t        w",
                                     "wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww"];
const NOTE_FIND_RANGE: f32 = 0.8;
fn note(translation: Vec3) -> impl Bundle {
  (Visuals::sprite(MySprite::Note),
   Proximal { distance: NOTE_FIND_RANGE },
   SpatialBundle { transform: Transform { translation,
                                          ..default() },
                   ..default() })
}
fn torch(pos: Vec3) -> impl Bundle {
  (Visuals::unlit_sprite(MySprite::Torch),
   FaceCamera,
   PointLightBundle { transform: Transform::from_translation(pos),
                      point_light: TORCH_LIGHT,
                      ..default() })
}
fn ghost(pos: Vec3) -> impl Bundle {
  (Monster,
   name("ghost"),
   Navigation::new(PLAYER_MAX_SPEED),
   CharacterBundle::new(pos, true, Visuals::sprite(MySprite::SpaceWizard)))
}
fn monster(pos: Vec3) -> impl Bundle {
  (Monster,
   Proximal { distance: MONSTER_CATCH_RANGE },
   name("monster"),
   Navigation::new(MONSTER_MAX_SPEED),
   CharacterBundle::new(pos, true, Visuals::sprite(MySprite::SpaceWizard)))
}
fn treemonster(pos: Vec3) -> impl Bundle {
  (Monster,
   name("tree monster"),
   Proximal { distance: MONSTER_CATCH_RANGE },
   Navigation::new(MONSTER_MAX_SPEED),
   CharacterBundle::new(pos, true, Visuals::sprite(MySprite::TreeMonster)))
}
fn tree(pos: Vec3) -> impl Bundle {
  (name("ghost"), CharacterBundle::new(pos, false, Visuals::sprite(MySprite::Tree)))
}
fn tent(pos: Vec3) -> impl Bundle {
  (name("tent"), CharacterBundle::new(pos, false, Visuals::sprite(MySprite::Tent)))
}
fn wall(pos: Vec3) -> impl Bundle {}

fn player(translation: Vec3) -> impl Bundle {
  (Player::default(),
   name("You"),
   Navigation::new(PLAYER_MAX_SPEED),
   CharacterBundle::new(translation, true, Visuals::sprite(MySprite::Player)))
}

pub fn setup(playerq: Query<&Transform, With<Player>>,
             serv: Res<AssetServer>,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             mut c: Commands) {
  let cube_collider = Cuboid::default().collider();
  let ground_mesh =
    bevy::math::primitives::Plane3d::new(Vec3::Y, Vec2::new(50.0, 50.0)).mesh()
                                                                        .build();
  let ground_collider = avian3d::prelude::Collider::trimesh_from_mesh(&ground_mesh).unwrap();
  let ground_texture = serv.load("embedded://grass.png");
  let ground_material = serv.add(StandardMaterial { perceptual_roughness: 0.8,
                                                    metallic: 0.0,
                                                    reflectance: 0.2,
                                                    base_color_texture:
                                                      Some(ground_texture),
                                                    ..default() });
  let ground = (ground_collider,
                RigidBody::Static,
                PbrBundle { mesh: serv.add(ground_mesh),
                            material: ground_material.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..default() });
  c.spawn(ground);
  let cube_mesh = Cuboid::default().mesh().build();
  let cube_collider = Collider::convex_hull_from_mesh(&cube_mesh).unwrap();
  let cube_mesh_handle = serv.add(cube_mesh);
  let position = Vec3::new(0.0, 4.0, 0.0);
  // c.spawn((RigidBody::Dynamic,
  //          ColliderMassProperties::new(&cube_collider, 1.0),
  //          cube_collider.clone(),
  //          PbrBundle { mesh: cube_mesh_handle.clone(),
  //                      material: ground_material.clone(),
  //                      transform:
  //                        Transform::from_translation(position).with_scale(Vec3::splat(3.5)),
  //                      ..default() }));
  for (y, row) in WORLD_MAP.iter().enumerate() {
    for (x, tile) in row.chars().enumerate() {
      let pos = Vec3::new(x as f32 * TILE_SIZE,
                          CHARACTER_HEIGHT * 0.5,
                          -(y as f32) * TILE_SIZE);

      match tile {
        'w' => {
          c.spawn((RigidBody::Static,
                   cube_collider.clone(),
                   PbrBundle { mesh: cube_mesh_handle.clone(),
                               material: ground_material.clone(),
                               transform: Transform::from_translation(pos),
                               ..default() }));
        }
        'l' => {
          c.spawn(torch(pos));
        }
        't' => {
          c.spawn(tree(pos));
        }
        'T' => {
          c.spawn(tent(pos));
        }
        'n' => {
          c.spawn(note(pos));
        }
        'g' => {
          c.spawn(treemonster(pos));
        }
        'p' => {
          let player_entity = c.spawn(player(pos)).id();
          c.spawn((PlayerFlashlight,
                   SpotLightBundle { spot_light: PLAYER_LIGHT_FLASHLIGHT,

                                     transform: Transform::from_xyz(0.0, 2.0, 2.3), // .with_rotation(Quat::from_rotation_z(4.0))
                                     ..default() }))
           .set_parent(player_entity);
          // c.spawn((PlayerFlashlight,
          //          PointLightBundle { point_light: PLAYER_LIGHT_AMBIENT,
          //                             transform: Transform::from_xyz(0.0,
          //                                                            1.7,
          //                                                            num_ambient_lights
          //                                                            as f32
          //                                                            * 2.0
          //                                                            + 0.3),
          //                             ..default() }))
          //  .set_parent(player_entity);
          let mut num_ambient_lights = 0;
          let mut spawn_ambient_light = || {
            c.spawn((PlayerAmbientlight,
                     PointLightBundle { point_light: PLAYER_LIGHT_AMBIENT,
                                        transform: Transform::from_xyz(0.0,
                                                                       1.7,
                                                                       num_ambient_lights
                                                                       as f32
                                                                       * 2.0
                                                                       + 0.3),
                                        ..default() }))
             .set_parent(player_entity);
            num_ambient_lights += 1;
          };

          spawn_ambient_light();
          spawn_ambient_light();
          spawn_ambient_light();
          spawn_ambient_light();
          spawn_ambient_light();
        }
        _ => {}
      }
    }
  }

  // c.spawn((SpatialBundle::default(),
  //          TerrainConfig::load_from_file("assets/default_terrain/terrain_config.ron"),
  //          TerrainData::new()));

  c.spawn(PbrBundle {
    mesh: meshes.add(Circle::new(4.0)),
    material: materials.add(Color::WHITE),
    transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ..default()
  });
  // let colorful_mat = serv.add(StandardMaterial::from(serv.add(colorful_texture())));
  c.spawn(PointLightBundle { point_light: PointLight { shadows_enabled: true,
                                                       ..default() },
                             transform: Transform::from_xyz(4.0, 8.0, 4.0),
                             ..default() });

  let fov = std::f32::consts::PI / 4.0;

  let pitch_upper_limit_radians = 1.0;
  let pitch_lower_limit_radians = 0.2;
  let camera =
    (IsDefaultUiCamera,
     BLOOM_SETTINGS,
     // Skybox { image: skybox_handle.clone(),
     //          brightness: 600.0 },
     Camera2d,
     // FOG_SETTINGS,
     Camera3dBundle { camera: Camera { hdr: true,

                                       ..default() },

                      tonemapping: TONEMAPPING,
                      projection:
                        Projection::Perspective(PerspectiveProjection { fov, ..default() }),
                      exposure: bevy::render::camera::Exposure { ev100: 10.0 },
                      // tonemapping:
                      //   bevy::core_pipeline::tonemapping::Tonemapping::Reinhard,
                      ..default() },
     PanOrbitCamera { // radius: Some(5.0),

                      // focus: todo!(),
                      // yaw: todo!(),
                      // pitch: todo!(),
                      // target_focus: todo!(),
                      // target_yaw: todo!(),
                      // target_pitch: todo!(),
                      // target_radius: todo!(),
                      // yaw_upper_limit: todo!(),
                      // yaw_lower_limit: todo!(),
                      pitch_upper_limit: Some(pitch_upper_limit_radians),
                      pitch_lower_limit: Some(pitch_lower_limit_radians),
                      zoom_upper_limit: Some(10.0),
                      zoom_lower_limit: Some(2.0),
                      // orbit_sensitivity: todo!(),
                      orbit_smoothness: 0.0,
                      pan_sensitivity: 0.0,
                      pan_smoothness: 0.5,
                      zoom_sensitivity: 2.5,
                      // zoom_smoothness: todo!(),
                      // button_orbit: todo!(),
                      // button_pan: todo!(),
                      // modifier_orbit: todo!(),
                      // modifier_pan: todo!(),
                      // touch_enabled: todo!(),
                      // touch_controls: todo!(),
                      // reversed_zoom: todo!(),
                      // is_upside_down: todo!(),
                      // allow_upside_down: todo!(),
                      // enabled: todo!(),
                      // initialized: todo!(),
                      // force_update: todo!(),
                      ..default() });
  c.spawn(camera);
  println("setup");
}

fn spawn_skybox(serv: Res<AssetServer>,
                mut images: ResMut<Assets<Image>>,
                mut camq: Query<Entity, With<Camera>>,
                mut c: Commands,
                mut skybox_handle: Local<Option<Handle<Image>>>,
                mut done: Local<bool>) {
  if let Ok(cam_entity) = camq.get_single()
     && !*done
  {
    // VolumetricFogSettings
    let skybox_handle = skybox_handle.get_or_insert_with(|| {
                                       serv.load(format!("embedded://{}",
                                                         MySprite::NasaStarmap.path()))
                                     })
                                     .clone();
    println("hmm1");
    if let Some(mut skybox) = images.get_mut(&skybox_handle) {
      println("hmm2");
      skybox.reinterpret_stacked_2d_as_array(skybox.height() / skybox.width());

      skybox.texture_view_descriptor =
        Some(TextureViewDescriptor { dimension: Some(bevy::render::render_resource::TextureViewDimension::Cube),
                                     ..default() });
      c.entity(cam_entity)
       .insert(Skybox { image: skybox_handle.clone(),
                        brightness: 600.0 });
      *done = true;
    }
  }
}
#[bevy_main]
pub fn main() {
  let gravity = avian3d::dynamics::integrator::Gravity::default();
  let solver_config = SolverConfig { contact_damping_ratio: 0.5,
                                     // contact_frequency_factor: 1.5,
                                     // max_overlap_solve_speed: 4.0,
                                     // warm_start_coefficient: 1.0,
                                     // restitution_threshold: 1.0,
                                     // restitution_iterations: 1,
                                     ..default() };
  let address_mode = ImageAddressMode::ClampToBorder;
  let default_sampler = ImageSamplerDescriptor { // address_mode_u: address_mode,
                                                 //                        address_mode_v: address_mode,
                                                 //                        address_mode_w: address_mode,
                                                 mag_filter: ImageFilterMode::Nearest,
                                                 min_filter: ImageFilterMode::Linear,
                                                 mipmap_filter: ImageFilterMode::Linear,
                                                 // compare:
                                                 //   Some(ImageCompareFunction::Less),
                                                 // lod_min_clamp: 10.0,
                                                 // lod_max_clamp: 100.0,
                                                 // border_color:
                                                 //   Some(ImageSamplerBorderColor::TransparentBlack),
                                                 // anisotropy_clamp: 1000,
                                                 ..default() };
  App::new()
    .add_plugins((
      EmbeddedAssetPlugin::default(),
      // bevy::pbr::ScreenSpaceAmbientOcclusionPlugin
      DefaultPlugins
      // .set(RenderPlugin {
      //   render_creation: bevy::render::settings::RenderCreation::Automatic(bevy::render::settings::WgpuSettings {
      //     backends: Some(bevy::render::settings::Backends::VULKAN),
      //     ..default()
      //   }),
      //   ..default()
      // })
        .set(ImagePlugin{default_sampler})
        .set(WindowPlugin {
          primary_window: Some(Window {
            // resolution: WindowResolution


            mode:WindowMode::Windowed,

            present_mode: bevy::window::PresentMode::AutoVsync,
            title: "bevy spooky game".to_string(),
            canvas: Some("#bevy".to_string()),
            ..default()
          }),
          ..default()
        }),
      bevy_vox_scene::VoxScenePlugin,
      bevy_sprite3d::Sprite3dPlugin,
      bevy_panorbit_camera::PanOrbitCameraPlugin,
      bevy_mod_billboard::prelude::BillboardPlugin,
      bevy_mod_picking::DefaultPickingPlugins,
      avian3d::PhysicsPlugins::default(),
      QuillPlugin,
      QuillOverlaysPlugin,
    ))// .add_plugins(add_global_highlight)
    // .add_event::<GuiInputEvent>()
    .init_resource::<UIData>()
    .init_resource::<TimeTicks>()
    .insert_resource(gravity)
    .insert_resource(solver_config)
  // .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(bevy_mod_picking::debug::DebugPickingMode::Normal)
    .init_asset::<bevy_vox_scene::VoxelScene>()
    .insert_resource(AMBIENT_LIGHT)
    .add_systems(Startup, (setup// ,add_global_highlight
                           // ,ui
    ).chain())

  // .add_systems(Startup, setup.run_if(in_state))
    .add_systems(Update,(
      close_on_esc,
      toggle_flashlight,
      navigation,
      player_movement,
      monster_movement,
      camera_follow_player,
      increment_time,
      origin_time,
      timed_animation_system,
      // combat_visual_effects,
      // player_actions,

    ).chain())
    .add_systems(Update,(
      face_camera,
      proximity_system,
      spawn_skybox,
      click_target,
      set_visuals,
      visuals,
      ui,
    ).chain())
    .run();
}

// trunk build --release --public-url "bevyspacegame" --filehash false

// trunk serve

// cargo check --target wasm32-unknown-unknown
// cargo run --target x86_64-unknown-linux-gnu
// cargo check --target x86_64-unknown-linux-gnu
