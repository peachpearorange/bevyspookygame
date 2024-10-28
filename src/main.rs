#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]
#![allow(dead_code)]
// #![feature(const_trait_impl)]
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
// #![feature(const_mut_refs)]

// #![feature(int_roundings)]
// #![recursion_limit = "1024"]
// #![feature(const_fn_floating_point_arithmetic)]

// pub mod bundletree;
// pub mod ui;

pub use bevy::prelude::Name;
use {avian3d::prelude::*,
     bevy::{app::AppExit,
            asset::{AssetServer, Handle},
            core_pipeline::{bloom::{BloomCompositeMode, BloomPrefilterSettings,
                                    BloomSettings},
                            Skybox},
            math::{primitives, vec2, vec3, Vec3},
            pbr::StandardMaterial,
            prelude::*,
            render::{render_resource::TextureViewDescriptor,
                     texture::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor}},
            utils::{HashMap, HashSet},
            window::WindowMode},
     bevy_embedded_assets::*,
     bevy_panorbit_camera::PanOrbitCamera,
     bevy_quill::{prelude::*, QuillPlugin, ViewChild},
     bevy_quill_overlays::QuillOverlaysPlugin,
     dynamics::solver::SolverConfig,
     fancy_constructor::new,
     rand::{random, thread_rng},
     rust_utils::*,
     std::f32::consts::PI};
// ui::UIData

pub const AMBIENT_LIGHT_COLOR: Color = Color::hsv(301.0, 1.0, 1.0);
pub const CLEAR_COLOR: Color = Color::hsv(301.0, 1.0, 0.07);

pub const GLOWY_COLOR: Color = Color::srgb(13.99, 11.32, 50.0);
pub const GLOWY_COLOR_2: Color = Color::srgb(30.0, 20.7, 10.5);
pub const GLOWY_COLOR_3: Color = Color::srgb(0.0, 30.0, 0.0);
pub const EXPLOSION_COLOR: Color = Color::srgb(8.0, 3.0, 3.0);
pub const LASER_COLOR: Color = Color::hsv(60.0, 1.0, 4.0);
// hsv(61, 100%, 100%)
pub const BILLBOARD_REL_SCALE: f32 = 2.0;
pub const TEXT_SCALE: f32 = 0.013;
pub const ENABLE_SHADOWS_OTHER_THAN_SUN: bool = false;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct MySprite {
  path: &'static str
}
impl MySprite {
  const fn new(path: &'static str) -> Self { Self { path } }
  const CAR: Self = Self::new("car.png");
  const COIN: Self = Self::new("coin.png");
  const FIRE: Self = Self::new("fire.png");
  const GATE: Self = Self::new("gate.png");
  const GRASS: Self = Self::new("grass.png");
  const GROUND: Self = Self::new("ground.png");
  const MISSILE: Self = Self::new("pixelc/missile.png");
  const NOTE: Self = Self::new("note.png");
  const PLAYER: Self = Self::new("player.png");
  const PORTAL: Self = Self::new("portal.png");
  const SIGN: Self = Self::new("sign.png");
  const SNOW: Self = Self::new("snow.png");
  const STICKMAN: Self = Self::new("stickman.png");
  const STONE: Self = Self::new("stone.png");
  const SUN: Self = Self::new("sun.png");
  const TENT: Self = Self::new("tent.png");
  const TORCH: Self = Self::new("torch.png");
  const TREE: Self = Self::new("tree.png");
  const TREEMONSTER: Self = Self::new("treemonster.png");
  const WATER: Self = Self::new("water.png");
}
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct MyNewImageMaterial {
  img: MySprite,
  mat_fn: fn(Handle<Image>) -> StandardMaterial
}
impl MyNewImageMaterial {
  const fn new(mat_fn: fn(Handle<Image>) -> StandardMaterial, img: MySprite) -> Self {
    Self { img, mat_fn }
  }
  pub fn val(&self, h: Handle<Image>) -> StandardMaterial { (self.mat_fn)(h) }
  pub fn img(&self) -> MySprite { self.img }
  const GROUND: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.8,
                                                        metallic: 0.0,
                                                        reflectance: 0.2,
                                                        ..h.into() },
                                 MySprite::GROUND);
  const SNOW: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.4,
                                                      metallic: 0.0,
                                                      reflectance: 0.5,
                                                      ior: 1.31,
                                                      ..h.into() },
                               MySprite::SNOW);
  const WATER: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.3,
                                                       metallic: 0.0,
                                                       reflectance: 0.5,
                                                       ..h.into() },
                                MySprite::WATER);
  const STONE: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.8,
                                                       metallic: 0.0,
                                                       reflectance: 0.3,
                                                       ..h.into() },
                                MySprite::STONE);
  // const BRICKS: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.95,
  //                                                       metallic: 0.0,
  //                                                       reflectance: 0.1,
  //                                                       ..h.into() },
  //                                MySprite::BRICKS);
  const GRASS: Self = Self::new(|h| StandardMaterial { perceptual_roughness: 0.8,
                                                       metallic: 0.0,
                                                       reflectance: 0.2,
                                                       ..h.into() },
                                MySprite::GRASS);
  // const PENGUIN: Self = Self::new(From::from, MySprite::PENGUIN);
}
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct MyNewMaterial {
  mat_fn: fn() -> StandardMaterial
}
impl MyNewMaterial {
  const fn new(mat_fn: fn() -> StandardMaterial) -> Self { Self { mat_fn } }
  pub fn val(&self) -> StandardMaterial { (self.mat_fn)() }
  const GLOWY: Self = Self::new(|| StandardMaterial { unlit: true,
                                                      alpha_mode: AlphaMode::Mask(0.0),
                                                      ..GLOWY_COLOR.into() });
  const GLOWY_2: Self = Self::new(|| StandardMaterial { unlit: true,
                                                        alpha_mode: AlphaMode::Mask(0.0),
                                                        ..GLOWY_COLOR_2.into() });
  const GLOWY_3: Self = Self::new(|| StandardMaterial { unlit: true,
                                                        alpha_mode: AlphaMode::Mask(0.0),
                                                        ..GLOWY_COLOR_3.into() });
  const EXPLOSION: Self = Self::new(|| StandardMaterial { unlit: true,
                                                          alpha_mode:
                                                            AlphaMode::Mask(0.0001),
                                                          ..EXPLOSION_COLOR.into() });
  const LASER: Self = Self::new(|| StandardMaterial { unlit: true,
                                                      alpha_mode:
                                                        AlphaMode::Mask(0.0001),
                                                      ..LASER_COLOR.into() });
  const PARTICLE: Self = Self::new(|| StandardMaterial::from(Color::srgb(0.2, 0.7, 0.9)));
  const INVISIBLE: Self = Self::new(|| StandardMaterial { unlit: true,
                                                          alpha_mode: AlphaMode::Blend,
                                                          ..Color::srgba(0.0, 0.0, 0.0,
                                                                         0.0).into() });
  const HOVERED: Self = Self::new(|| StandardMaterial { unlit: true,
                                                        alpha_mode: AlphaMode::Blend,
                                                        ..Color::srgba(0.0, 0.3, 1.0,
                                                                       0.1).into() });
  const PRESSED: Self = Self::new(|| StandardMaterial { unlit: true,
                                                        alpha_mode: AlphaMode::Blend,
                                                        ..Color::srgba(0.0, 0.3, 1.0,
                                                                       0.3).into() });
  const SELECTED: Self = Self::new(|| StandardMaterial { unlit: true,
                                                         alpha_mode: AlphaMode::Blend,
                                                         ..Color::srgba(0.0, 0.3, 1.0,
                                                                        0.2).into() });
}
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct MyNewScene {
  path: &'static str,
  label: &'static str
}
impl MyNewScene {
  pub const fn new(path: &'static str, label: &'static str) -> Self { Self { path, label } }
  pub const fn path_and_label(&self) -> (&'static str, &'static str) {
    (self.path, self.label)
  }
  pub const LUNAR_LANDER: Self = Self::new("lunarlander.glb", "Scene0");
  pub const CHARACTER_CONTROLLER_DEMO: Self =
    Self::new("character_controller_demo.glb", "Scene0");
  pub const LEVEL: Self = Self::new("level.glb", "Scene0");
  pub const A_LEVEL: Self = Self::new("alevel.gltf", "Scene0");
  pub const ISLAND_LEVEL: Self = Self::new("this_here_level.glb", "Scene0");
  pub const SOME_SKETCH_LEVEL: Self = Self::new("somesketchlevel.glb", "Scene0");
  pub const SNOWMAN: Self = Self::new("snowman.glb", "Scene0");
  pub const COFFEE_SCENE: Self = Self::new("coffee.glb", "Scene0");
  pub const GOXEL_LEVEL: Self = Self::new("goxel_level.glb", "Scene0");
  pub const TURTLE_LEVEL: Self = Self::new("turtle level.gltf", "Scene0");
  pub const WAT: Self = Self::new("wat.glb", "Scene0");
}
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct GenNewMesh {
  gen_fn: fn() -> Mesh
}
impl GenNewMesh {
  const fn new(gen_fn: fn() -> Mesh) -> Self { Self { gen_fn } }
  pub fn gen(&self) -> Mesh { (self.gen_fn)() }
  pub const UNIT_CUBE: Self = Self::new(|| Cuboid::new(1.0, 1.0, 1.0).into());
  pub const UNIT_CYLINDER: Self = Self::new(|| primitives::Cylinder::new(1.0, 1.0).into());
  pub const CUBE: Self = Self::new(|| Cuboid::new(0.7, 0.7, 0.7).into());
  pub const BOX: Self = Self::new(|| Cuboid::new(2.0, 1.0, 1.0).into());
  pub const FLAT_BOX: Self = Self::new(|| Cuboid::new(2.1, 0.3, 2.1).into());
  pub const CAPSULE: Self = Self::new(|| primitives::Capsule3d::default().into());
  pub const TORUS: Self = Self::new(|| primitives::Torus::default().into());
  pub const SPHERE: Self = Self::new(|| primitives::Sphere { radius: 1.0 }.into());
  pub const PLANE_SIZE_50: Self = Self::new(|| Cuboid::new(25.0, 0.1, 25.0).into());
  pub const BILLBOARD_MESH_SQUARE: Self = Self::new(|| {
    primitives::Rectangle::new(BILLBOARD_REL_SCALE, BILLBOARD_REL_SCALE).into()
  });
}

#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct Visuals {
  text: Option<String>,
  material_mesh: Option<(MyNewMaterial, GenNewMesh)>,
  sprite: Option<MySprite>,
  unlit: bool,
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
  fn material_mesh(material: MyNewMaterial, mesh: GenNewMesh) -> Self {
    Self { material_mesh: Some((material, mesh)),
           ..default() }
  }
  fn material_sphere(material: MyNewMaterial) -> Self {
    Self::material_mesh(material, GenNewMesh::SPHERE)
  }
  fn with_text(self, text: impl ToString) -> Self {
    Self { text: Some(text.to_string()),
           ..self }
  }
}
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
               mut mesh_handles: Local<HashMap<GenNewMesh, Handle<Mesh>>>,
               mut material_handles: Local<HashMap<MyNewMaterial,
                             Handle<StandardMaterial>>>,
               mut visual_child_entities: Local<HashMap<Entity, Entity>>) {
  let mut get_material_handle = |material: MyNewMaterial| {
    material_handles.entry(material)
                    .or_insert_with(|| serv.add(material.val()))
                    .clone()
  };

  let mut get_mesh_handle = |mesh: GenNewMesh| {
    mesh_handles.entry(mesh)
                .or_insert_with(|| serv.add(mesh.gen()))
                .clone()
  };

  let mut get_sprite_handle = |sprite: MySprite| {
    sprite_handles.entry(sprite)
                  .or_insert_with(|| serv.load(format!("embedded://{}", sprite.path)))
                  .clone()
  };

  let text_style = TextStyle { font_size: 30.0,
                               ..default() };
  let invisible_material = get_material_handle(MyNewMaterial::INVISIBLE);

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
                        mesh: get_mesh_handle(GenNewMesh::SPHERE),
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
                                             alpha_mode: AlphaMode::Blend,
                                             unlit: visuals.unlit,
                                             transform: Transform::from_xyz(0.0, 0.0,
                                                                            0.0),
                                             ..default() }.bundle(&mut sprite_3d_params)))
           .set_parent(main_visual_child);
        } else {
          visuals.done = false;
        }
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

#[derive(Component)]
struct PlayerFlashlight;
#[derive(Component)]
struct PlayerAmbientlight;
fn toggle_flashlight(mut c: Commands,
                     mut playerq: Query<&mut Player>,
                     mut flashlightq: Query<&mut Visibility, With<PlayerFlashlight>>,
                     keyboard_input: Res<ButtonInput<KeyCode>>) {
  if keyboard_input.just_pressed(KeyCode::KeyF)
     && let Ok(mut player) = playerq.get_single_mut()
  {
    player.light_on = !player.light_on;
    for mut flashlight_visibility in &mut flashlightq {
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
// pub fn click_target(mut parent_q: Query<&Parent>,
//                     mut click_events: EventReader<bevy_mod_picking::events::Pointer<bevy_mod_picking::events::Click>>,
//                     mut player_q: Query<&mut Player>) {
//   if let Ok(mut player) = player_q.get_single_mut() {
//     for event in click_events.read() {
//       println(debugfmt(event));
//       let mut root_entity = event.target;
//       while let Ok(parent) = parent_q.get(root_entity) {
//         root_entity = parent.get();
//       }
//       println!("Player target set to {root_entity}");
//     }
//   }
// }
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

#[derive(Component)]
struct Monster {
  is_dormant: bool
}
fn monster_movement(mut monsterq: Query<(&mut Navigation, &mut Monster, &Transform)>,
                    playerq: Query<(Entity, &Player, &Transform)>,
                    time: Res<TimeTicks>) {
  if let Ok((player_entity, player, player_transform)) = playerq.get_single() {
    for (mut monsternav, mut monster, monster_transform) in &mut monsterq {
      let dist = player_transform.translation
                                 .distance(monster_transform.translation);

      if (dist < MONSTER_SEE_DARK_RANGE)
         || (player.light_on && (dist < MONSTER_SEE_LIT_RANGE))
      {
        *monsternav = Navigation { max_speed: MONSTER_MAX_SPEED_CHASE,
                                   navigation_kind: NavigationKind::Chase(player_entity) };
        monster.is_dormant = false;
      } else {
        if !monster.is_dormant {
          if time.0 % 300 == 0 {
            let dir = random::<Dir2>().as_vec2().normalize_or_zero();
            *monsternav = Navigation { max_speed: MONSTER_MAX_SPEED,
                                       navigation_kind: NavigationKind::Vec2(dir) };
          }
        }
      }
    }
  }
}
pub fn player_movement(keyboard_input: Res<ButtonInput<KeyCode>>,
                       gameover: Res<GameOver>,
                       camera_query: Query<&Transform, With<Camera3d>>,
                       mut player_query: Query<&mut Navigation, With<Player>>) {
  if let Ok(camera_transform) = camera_query.get_single()
     && let Ok(mut navigation) = player_query.get_single_mut()
     && !gameover.0
  {
    let forward =
      vec2(camera_transform.forward().x, camera_transform.forward().z).normalize_or_zero();
    let right = vec2(-forward.y, forward.x);
    let Vec2 { x, y } = sum(filter_map(|(key, v)| keyboard_input.pressed(key).then_some(v),
                                       [(KeyCode::KeyA, Vec2::NEG_X),
                                        (KeyCode::KeyS, Vec2::NEG_Y),
                                        (KeyCode::KeyD, Vec2::X),
                                        (KeyCode::KeyW, Vec2::Y)])).normalize_or_zero();
    let keyb_dir = (x * right) + (y * forward);
    navigation.navigation_kind = NavigationKind::Vec2(keyb_dir);
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
  pub game_over: bool,
  pub infobox_data: Vec<String>
}

const UI_BACKGROUND_COLOR: Color = Color::srgba(0.9, 0.9, 0.7, 1.0);
const UI_BORDER_COLOR: Color = Color::srgba(0.8, 0.8, 0.7, 1.0);
const UI_FONT_COLOR: Color = Color::srgba(0.3, 0.3, 0.3, 1.0);
pub fn common_style(sb: &mut StyleBuilder) {
  sb.font_size(32.0)
    .display(Display::Block)
    .border(3)
    .border_color(UI_BORDER_COLOR)
    .background_color(UI_BACKGROUND_COLOR)
    .position(bevy::ui::PositionType::Absolute)
    .color(UI_FONT_COLOR)
    // .margin(5)
    // .padding(3)
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
    let game_over_popup = UIPopup { style: |sb| {
                                      sb.font_size(55.0)
                                        .justify_self(JustifySelf::Center)
                                        .top(Val::Percent(50.0));
                                    },
                                    display_text_fn: |&UIData { game_over, .. }| {
                                      if game_over {
                                        vec!["Game Over".to_string()]
                                      } else {
                                        default()
                                      }
                                    } };
    let note_popup = UIPopup { style: |sb| {
                                 sb.justify_self(JustifySelf::Center)
                                   .top(Val::Percent(50.0));
                               },
                               display_text_fn: |uidata| uidata.note.clone() };
    let infobox = UIPopup { style: |sb| {
                              sb.left(0).top(0);
                            },
                            display_text_fn: |uidata| uidata.infobox_data.clone() };
    let root = vec![ViewChild::new(note_popup),
                    ViewChild::new(infobox),
                    ViewChild::new(game_over_popup),];
    c.spawn(root.to_root());
  }
  if let Ok((player_entity, player, player_transform)) = playerq.get_single() {
    let player_pos = player_transform.translation;
    let player_light_on = player.light_on;
    let infobox_data =
      map(ToString::to_string,
          [format!("{:.1}", player_pos).as_str(),
           format!("you've found {} notes", player.notes_found.len()).as_str(),
           format!("light on: {player_light_on}",).as_str(),
           "w,a,s,d: move",
           "f: toggle flashlight"]) // .chain(map(|(item, n)| {
                                   //              format!("{} {:?}s", n, item)
                                   //            },
                                   //            player_inventory.0.clone()))
                                   .collect();

    let current_time = time.0;
    let current_time_ticks = current_time;
    let old_ui_data = ui_data.clone();
    *ui_data = UIData { current_time_ticks,
                        infobox_data,
                        ..old_ui_data };
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

pub const AMBIENT_LIGHT: AmbientLight = AmbientLight { color: AMBIENT_LIGHT_COLOR,
                                                       brightness: 14.0 };
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
               range: 8.0,
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
const MONSTER_SEE_DARK_RANGE: f32 = 4.0;
const MONSTER_SEE_LIT_RANGE: f32 = 12.0;

#[derive(Component, new)]
struct Proximal {
  distance: f32
}
#[derive(Component)]
struct Note(&'static str);
#[derive(Component, Clone, Debug, Default)]
pub struct Player {
  notes_found: HashSet<Entity>,
  light_on: bool
}
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
  Running,
  #[default]
  NotStarted,
  GameOver
}
#[derive(Resource, Default)]
struct GameOver(bool);
fn proximity_system(mut c: Commands,
                    mut player_query: Query<(Entity,
                           &mut Player,
                           &Transform,
                           &mut LockedAxes)>,
                    proximal_entities: Query<(Entity, &Transform, &Proximal),
                          Without<Player>>,
                    monsterq: Query<(Entity, &Monster)>,
                    noteq: Query<(Entity, &Note)>,
                    mut game_over: ResMut<GameOver>,
                    time: Res<TimeTicks>,
                    mut uidata: ResMut<UIData>,
                    flashlightq: Query<Entity, With<PlayerFlashlight>>,
                    playerambientlightq: Query<Entity, With<PlayerAmbientlight>>,
                    mut prev_proximal_entities: Local<HashSet<Entity>>) {
  if let Ok((player_entity, mut player, player_transform, mut player_lockedaxes)) =
    player_query.get_single_mut()
  {
    // uidata.note = default();
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
    uidata.note = find_map(|&e| {
                             noteq.get(e)
                                  .map(|(_, &Note(text))| vec![text.to_string()])
                                  .ok()
                           },
                           &current_proximal_entities).unwrap_or_default();
    for &e in new_proximal_entities {
      println("new proximal entity");
      if is_monster(e) {
        game_over.0 = true;
        uidata.game_over = true;
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
        // let note = NOTES[player.notes_found];
        player.notes_found.insert(e);
      }
    }
    *prev_proximal_entities = current_proximal_entities;
  }
}

const TILE_SIZE: f32 = 1.0;
const CHARACTER_HEIGHT: f32 = TILE_SIZE;
const CHARACTER_RADIUS: f32 = CHARACTER_HEIGHT * 0.3;
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

const NOTES:&[&'static str] = &[
  "WIN",
  "Diary entry 11\n they got me. I'm turning into a tree. it's over",
  "Diary entry 10\n oh my god the trees are chasing me!",
  "Diary entry 9\n I think I saw a tree move. I must be seeing things",
  "Diary entry 8\n Where could they have gone?",
  "Diary entry 7\n Now I can't find Sallie. Damn",
  "Diary entry 6\n Now Sallie is also looking for Billy",
  "Diary entry 5\n I haven't been able to find Sallie",
  "Diary entry 4\n I've been looking for Billy. I heard more strange sounds. Don't see any bears.",
  "Diary entry 3\n can't find Billy. where could he be?",
  "Diary entry 1\n me and Billy and Sallie are camping in this forest.",
  "Diary entry 2\n I heard some strange sounds. could be a bear.\n yikes.",
  "Diary entry 7\n",
  "Diary entry 8\n",
  "Diary entry 9\n",
  "Diary entry 10\n",
  "Diary entry 11\n",
  "Diary entry 12\n",
  "Diary entry 13\n",
  "Diary entry 14\n",
  "Diary entry 15\n",
  "Diary entry 16\n",
  "Diary entry 17\n",
  "Diary entry 18\n",
  "Diary entry 19\n",
];
const WORLD_MAP: &[&'static str] =
  &["                                  ttttttt                                       ",
    "                                  t  l  t                                       ",
    "                                  t     t                                       ",
    "                                  t     t                                       ",
    "                                  t lnl t                                       ",
    "                                  t     t                                       ",
    "                                  ttttttt                                       ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww",
    "w         g                                  t      t                  g       w",
    "w         t t                                                                  w",
    "w     wwwww wwwwwwww                    t       t         t          t     t   w",
    "w     wt  tlt  tw                                 l                  tP        w",
    "w     w         w                      g     t    n             t    t         w",
    "w     w         w            t                                       t         w",
    "w     w  w   w  w                       t                                      w",
    "w     wttw   wttw                    l           g                             w",
    "w                                    n          t                              w",
    "w                                                                              w",
    "w              t        t                 t                     g              w",
    "w                                  t                                   t       w",
    "w  l                                                                   t       w",
    "w  n      t                                                            t       w",
    "w                                  t                                   t       w",
    "w                                         t     nl    t      t                 w",
    "w         t             t           g                                          w",
    "w                                  t    t                         nl           w",
    "w                                                                              w",
    "w                                                            t                 w",
    "w       t                                                           t          w",
    "w                                                                              w",
    "wggg                           t                                               w",
    "w                                                t                             w",
    "w      t                                                                       w",
    "wttttttt        t         t                                         t          w",
    "w  l                      t                                              nl    w",
    "w                                       t              t                       w",
    "w                              t         Tl                                    w",
    "w                     l                   n                         t          w",
    "w  tt                 n                                                        w",
    "w          t      t       t           Tl     Tl          nl   g                w",
    "w    t            g                   t                                        w",
    "w         t    t  t      ttt                    t                 t            w",
    "w                        tg                                                    w",
    "w                                                                              w",
    "w   t      t   t  g                                                            w",
    "w                                     t                                        w",
    "w                 t       t                                                    w",
    "w       l  t                                    t           t                  w",
    "w       n                                                                      w",
    "w          t      tw                      t                            t       w",
    "w                                                                              w",
    "w   ttt                               t                                        w",
    "w          ttt            t                                                    w",
    "wwwwwww p w                     t             t             t          t       w",
    "wwwwwww   w t  t tt                               nl                           w",
    "wwwwwww c w                                                                    w",
    "wwwwwww   w                      t             t          t           t        w",
    "w         w  tt   t t                                                          w",
    "wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww"];
// const WIN_MAP: &[&'static str] = &["ttttttttt",
//                                    "t       t",
//                                    "t  l    t",
//                                    "t       t",
//                                    "t       t",
//                                    "t       t",
//                                    "t l wl  t",
//                                    "t       t",
//                                    "ttttttttt"];
const NOTE_FIND_RANGE: f32 = 1.8;
fn note(translation: Vec3, note_data: &'static str) -> impl Bundle {
  (Visuals::sprite(MySprite::NOTE),
   Note(note_data),
   Proximal { distance: NOTE_FIND_RANGE },
   SpatialBundle { transform: Transform { translation,
                                          rotation:
                                            (Quat::from_rotation_y(avian3d::math::PI * 0.2)
                                             * Quat::from_rotation_x(avian3d::math::PI
                                                                     * 0.5)).into(),
                                          scale: Vec3::splat(0.5) },
                   ..default() })
}
fn torch(pos: Vec3) -> impl Bundle {
  (Visuals::unlit_sprite(MySprite::TORCH),
   FaceCamera,
   PointLightBundle { transform: Transform::from_translation(pos),
                      point_light: TORCH_LIGHT,
                      ..default() })
}
#[derive(Component, Clone)]
struct Portal;

#[derive(Resource, Default)]
struct Win(bool);
fn portal_win(mut c: Commands,
              mut playerq: Query<&mut Transform, With<Player>>,
              mut portalq: Query<&Transform, (With<Portal>, Without<Player>)>,
              mut flashlightq: Query<&mut Visibility, With<PlayerFlashlight>>,
              keyboard_input: Res<ButtonInput<KeyCode>>) {
  for mut player_transform in &mut playerq {
    for &Transform { translation: portal_translation,
                     .. } in &portalq
    {
      if player_transform.translation.distance(portal_translation) < 0.5 {
        player_transform.translation = vec3(37.0, 0.0, 3.0);
      }
    }
  }
}
fn portal(pos: Vec3) -> impl Bundle {
  (name("portal"),
   Note("what the heck is this?"),
   Proximal { distance: NOTE_FIND_RANGE },
   FaceCamera,
   Portal,
   Visuals::unlit_sprite(MySprite::PORTAL),
   PointLightBundle { transform: Transform::from_translation(pos),
                      point_light: TORCH_LIGHT,
                      ..default() })
}
// fn ghost(pos: Vec3) -> impl Bundle {
//   (Monster { is_dormant: true },
//    name("ghost"),
//    Navigation::new(PLAYER_MAX_SPEED),
//    CharacterBundle::new(pos, true, Visuals::sprite(MySprite::SPACEWIZARD)))
// }
// fn monster(pos: Vec3) -> impl Bundle {
//   (Monster { is_dormant: true },
//    Proximal { distance: MONSTER_CATCH_RANGE },
//    name("monster"),
//    Navigation::new(MONSTER_MAX_SPEED),
//    CharacterBundle::new(pos, true, Visuals::sprite(MySprite::SPACEWIZARD)))
// }
fn treemonster(pos: Vec3) -> impl Bundle {
  (Monster { is_dormant: true },
   name("tree monster"),
   Proximal { distance: MONSTER_CATCH_RANGE },
   Navigation::new(MONSTER_MAX_SPEED),
   CharacterBundle::new(pos, true, Visuals::sprite(MySprite::TREEMONSTER)))
}
fn tree(pos: Vec3) -> impl Bundle {
  (name("ghost"), CharacterBundle::new(pos, false, Visuals::sprite(MySprite::TREE)))
}
fn tent(pos: Vec3) -> impl Bundle {
  (name("tent"),
   Note("It's a tent"),
   Proximal { distance: NOTE_FIND_RANGE },
   CharacterBundle::new(pos, false, Visuals::sprite(MySprite::TENT)))
}
fn car(pos: Vec3) -> impl Bundle {
  (Visuals::unlit_sprite(MySprite::CAR),
   Note("You thought it'd be fun to drive through this forest but your car ran out of gas"),
   Proximal { distance: NOTE_FIND_RANGE },
   FaceCamera,
   PointLightBundle { transform: Transform::from_translation(pos),
                      point_light: TORCH_LIGHT,
                      ..default() })
}
fn wall(pos: Vec3) -> impl Bundle {}

fn player(translation: Vec3) -> impl Bundle {
  (Player::default(),
   name("You"),
   Navigation::new(PLAYER_MAX_SPEED),
   CharacterBundle::new(translation, true, Visuals::sprite(MySprite::PLAYER)))
}

// fn reset(mut world: &mut World) {
//   let mut gameover = world.resource_mut::<GameOver>();
//   if gameover.0 {
//     gameover.0 = false;

//     // world.clear_entities();
//     // bevy::ecs::system::RunSystemOnce::run_system_once(world, setup);
//   }
// }
// fn init(mut world: &mut World) { world.clear_entities() }
#[derive(Resource, Default)]
struct PlayerStartPos(Vec3);
pub fn setup(playerq: Query<&Transform, With<Player>>,
             serv: Res<AssetServer>,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             mut c: Commands) {
  let cube_collider = Cuboid::default().collider();
  let ground_mesh =
    bevy::math::primitives::Plane3d::new(Vec3::Y, Vec2::new(40.0, 25.5)).mesh()
                                                                        .build();
  // let ground_mesh =
  //   bevy::math::primitives::Plane3d::new(Vec3::Y, Vec2::new(100.0, 100.0)).mesh()
  //                                                                         .build();
  let ground_collider = avian3d::prelude::Collider::trimesh_from_mesh(&ground_mesh).unwrap();
  let ground_texture = serv.load("embedded://ground.png");
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
                            transform: Transform::from_xyz(39.5, 0.0, 42.5),
                            ..default() });
  c.spawn(ground);
  let small_ground_mesh =
    bevy::math::primitives::Plane3d::new(Vec3::Y, Vec2::new(3.5, 3.5)).mesh()
                                                                      .build();
  let small_ground_collider =
    avian3d::prelude::Collider::trimesh_from_mesh(&small_ground_mesh).unwrap();
  let small_ground = (small_ground_collider,
                      RigidBody::Static,
                      PbrBundle { mesh: serv.add(small_ground_mesh),
                                  material: ground_material.clone(),
                                  transform: Transform::from_xyz(37.0, 0.0, 3.0),
                                  ..default() });
  c.spawn(small_ground);
  let cube_mesh = Cuboid::default().mesh().build();
  let cube_collider = Collider::convex_hull_from_mesh(&cube_mesh).unwrap();
  let cube_mesh_handle = serv.add(cube_mesh);
  let position = Vec3::new(0.0, 4.0, 0.0);
  let mut num_notes_spawned = 0;

  for (x, y, tile) in enumerate(WORLD_MAP).flat_map(move |(y, line)| {
                                            line.char_indices()
                                                .map(move |(x, tile)| (x, y, tile))
                                          })
  {
    let pos = vec3(x as f32 * TILE_SIZE,
                   CHARACTER_HEIGHT * 0.5,
                   y as f32 * TILE_SIZE);

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
      'c' => {
        c.spawn(car(pos));
      }
      't' => {
        c.spawn(tree(pos));
      }
      'T' => {
        c.spawn(tent(pos));
      }
      'n' => {
        c.spawn(note(pos - Vec3::Y * 0.47,
                     NOTES.get(num_notes_spawned).unwrap_or(&"unwritten note")));
        num_notes_spawned += 1;
      }
      'g' => {
        c.spawn(treemonster(pos));
      }
      'P' => {
        c.spawn(portal(pos));
      }
      'p' => {
        let player_entity = c.spawn(player(pos)).id();
        c.spawn((PlayerFlashlight,
                 SpotLightBundle { spot_light: PLAYER_LIGHT_FLASHLIGHT,

                                   transform: Transform::from_xyz(0.0, 2.0, 2.3),
                                   visibility: Visibility::Hidden,
                                   ..default() }))
         .set_parent(player_entity);
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
        // spawn_ambient_light();
        // spawn_ambient_light();
      }
      _ => {}
    }
  }

  // c.spawn((SpatialBundle::default(),
  //          TerrainConfig::load_from_file("assets/default_terrain/terrain_config.ron"),
  //          TerrainData::new()));

  // c.spawn(PbrBundle {
  //   mesh: meshes.add(Circle::new(4.0)),
  //   material: materials.add(Color::WHITE),
  //   transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
  //   ..default()
  // });
  // let colorful_mat = serv.add(StandardMaterial::from(serv.add(colorful_texture())));
  // c.spawn(PointLightBundle { point_light: PointLight { shadows_enabled: true,
  //                                                      ..default() },
  //                            transform: Transform::from_xyz(4.0, 8.0, 4.0),
  //                            ..default() });

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
                      zoom_upper_limit: Some(8.0),
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
      .set(bevy::render::RenderPlugin {
        render_creation: bevy::render::settings::RenderCreation::Automatic(bevy::render::settings::WgpuSettings {
          backends: Some(bevy::render::settings::Backends::VULKAN),
          ..default()
        }),
        ..default()
      })
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

      // bevy_vox_scene::VoxScenePlugin,
      bevy_sprite3d::Sprite3dPlugin,
      bevy_panorbit_camera::PanOrbitCameraPlugin,
      bevy_mod_billboard::prelude::BillboardPlugin,
      // bevy_mod_picking::DefaultPickingPlugins,
      avian3d::PhysicsPlugins::default(),
      QuillPlugin,
      QuillOverlaysPlugin,
    ))// .add_plugins(add_global_highlight)
  // .add_event::<GuiInputEvent>()

    .init_state::<GameState>()
    .init_resource::<UIData>()
    .init_resource::<GameOver>()
    .init_resource::<TimeTicks>()
    .insert_resource(gravity)
    .insert_resource(solver_config)
    .insert_resource(ClearColor(CLEAR_COLOR))
    .insert_resource(AMBIENT_LIGHT)
    .add_systems(Startup, (setup// ,add_global_highlight
                           // ,ui
    ).chain())
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
    ).chain())
    .add_systems(Update,(
      portal_win,
      face_camera,
      proximity_system,
      visuals,
      ui,
    ).chain())
    .run();
}

// trunk build --release --public-url "bevyspookygame" --filehash false

// trunk serve

// cargo check --target wasm32-unknown-unknown
// cargo run --target x86_64-unknown-linux-gnu
// cargo check --target x86_64-unknown-linux-gnu
// cargo build --target x86_64-pc-windows-gnu --release
