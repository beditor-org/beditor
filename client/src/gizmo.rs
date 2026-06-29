use bevy::prelude::*;

use crate::app::EditorCamera;

const VIEWPORT_W: f32 = 1280.0;
const VIEWPORT_H: f32 = 720.0;
/// Hit-test threshold in game-normalised [0,1] screen space (~5 % of width).
const HIT_THRESHOLD: f32 = 0.05;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis {
	X,
	Y,
	Z,
}

impl Axis {
	pub fn to_vec3(self) -> Vec3 {
		match self {
			Axis::X => Vec3::X,
			Axis::Y => Vec3::Y,
			Axis::Z => Vec3::Z,
		}
	}

	fn color(self, highlighted: bool) -> Color {
		if highlighted {
			Color::srgb(1.0, 1.0, 0.0) // yellow when active
		} else {
			match self {
				Axis::X => Color::srgb(1.0, 0.15, 0.15),
				Axis::Y => Color::srgb(0.15, 1.0, 0.15),
				Axis::Z => Color::srgb(0.15, 0.15, 1.0),
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct GizmoDrag {
	pub axis: Axis,
	pub entity_start_pos: Vec3,
	pub mouse_start: Vec2, // game-normalised [0,1]
}

/// The entity currently focused (double-clicked) in the editor.
#[derive(Resource, Default)]
pub struct GizmoTarget {
	pub entity: Option<u32>,
	pub drag: Option<GizmoDrag>,
}

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<GizmoTarget>().add_systems(Update, draw_transform_gizmo);
	}
}

/// Gizmo arrow length proportional to camera distance so it looks consistent on screen.
pub fn compute_gizmo_scale(cam_pos: Vec3, entity_pos: Vec3) -> f32 {
	(cam_pos - entity_pos).length() * 0.15
}

/// Returns the axis whose arrow is closest to `mouse_norm` (game-normalised [0,1]),
/// or `None` if no axis is within `HIT_THRESHOLD`.
pub fn hit_test_gizmo(
	cam: &Camera,
	cam_tf: &GlobalTransform,
	entity_pos: Vec3,
	scale: f32,
	mouse_norm: Vec2,
) -> Option<Axis> {
	let vp = Vec2::new(VIEWPORT_W, VIEWPORT_H);
	// If the entity itself is off-screen, skip.
	let base = match cam.world_to_viewport(cam_tf, entity_pos) {
		Ok(v) => v / vp,
		Err(e) => {
			eprintln!("[gizmo] entity off-screen: {e:?}");
			return None;
		}
	};

	eprintln!("[gizmo] base_screen={base:?}  mouse={mouse_norm:?}");

	let mut best: Option<(Axis, f32)> = None;
	for axis in [Axis::X, Axis::Y, Axis::Z] {
		let tip_world = entity_pos + axis.to_vec3() * scale;
		// If this tip is off-screen, skip only this axis (not the whole function).
		let tip = match cam.world_to_viewport(cam_tf, tip_world) {
			Ok(v) => v / vp,
			Err(_) => continue,
		};
		let dist = point_to_segment_dist(mouse_norm, base, tip);
		eprintln!("[gizmo] axis={axis:?}  tip={tip:?}  dist={dist:.4}  threshold={HIT_THRESHOLD}");
		if dist < HIT_THRESHOLD && best.map_or(true, |(_, d)| dist < d) {
			best = Some((axis, dist));
		}
	}
	best.map(|(axis, _)| axis)
}

/// Project the mouse delta onto the world axis and return the scalar world-space movement.
/// Uses the screen-space projection of the axis for accurate constrained translation.
pub fn compute_axis_movement(
	cam: &Camera,
	cam_tf: &GlobalTransform,
	axis: Axis,
	entity_pos: Vec3,
	mouse_start: Vec2,
	mouse_current: Vec2,
) -> f32 {
	let vp = Vec2::new(VIEWPORT_W, VIEWPORT_H);
	let base = match cam.world_to_viewport(cam_tf, entity_pos) {
		Ok(v) => v / vp,
		Err(_) => return 0.0,
	};
	let tip = match cam.world_to_viewport(cam_tf, entity_pos + axis.to_vec3()) {
		Ok(v) => v / vp,
		Err(_) => return 0.0,
	};
	// Direction the axis moves in normalised screen space
	let screen_axis = tip - base;
	let denom = screen_axis.length_squared();
	if denom < 1e-6 {
		return 0.0;
	}
	(mouse_current - mouse_start).dot(screen_axis) / denom
}

fn point_to_segment_dist(p: Vec2, a: Vec2, b: Vec2) -> f32 {
	let ab = b - a;
	let len_sq = ab.length_squared();
	if len_sq < 1e-8 {
		return (p - a).length();
	}
	let t = ((p - a).dot(ab) / len_sq).clamp(0.0, 1.0);
	(p - (a + t * ab)).length()
}

fn draw_transform_gizmo(
	gizmo_target: Res<GizmoTarget>,
	mut gizmos: Gizmos,
	entity_query: Query<(Entity, &GlobalTransform)>,
	camera_query: Query<&GlobalTransform, With<EditorCamera>>,
) {
	let Some(target_id) = gizmo_target.entity else {
		return;
	};
	let Some((_, gt)) = entity_query.iter().find(|(e, _)| e.index_u32() == target_id) else {
		return;
	};
	let pos = gt.translation();
	let cam_pos = camera_query.iter().next().map(|c| c.translation()).unwrap_or(Vec3::ZERO);
	let scale = compute_gizmo_scale(cam_pos, pos).max(0.1);

	let dragged_axis = gizmo_target.drag.as_ref().map(|d| d.axis);
	for axis in [Axis::X, Axis::Y, Axis::Z] {
		gizmos.arrow(pos, pos + axis.to_vec3() * scale, axis.color(dragged_axis == Some(axis)));
	}
}
