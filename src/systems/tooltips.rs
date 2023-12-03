use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn tooltips(
	ecs: &SubWorld,
	#[resource] mouse_pos: &Point,
	#[resource] camera: &Camera,
){
	let mut positions = <(Entity, &Point, &Name)>::query();
	let mut fov = <&FieldOfView>::query().filter(component::<Player>());
	let offset = Point::new(camera.left_x, camera.top_y);
	let map_pos = *mouse_pos + offset;
	let mut draw_batch = DrawBatch::new();
	draw_batch.target(2);
	let player_fov = fov.iter(ecs).nth(0).unwrap();
	positions
		.iter(ecs)
		.filter(|(_, pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(&pos))//filter from all entities to only those with the same position as the mouse and visibles by player, with closure in filter() method
		.for_each(|(entity, _, name)| {
			let screen_pos = *mouse_pos * 4;
			let display = if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
				format!("{} : {} hp", &name.0, health.current)
			} else {//if the entity doesn't have a health component, just display the name
				name.0.clone()
			};
			draw_batch.print(screen_pos, &display);
		});
	draw_batch.submit(10100).expect("Batch error");

}