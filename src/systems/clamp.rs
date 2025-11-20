use crate::components::bounded::Bounded;
use crate::map::MapConfig;
use bevy::prelude::*;

pub fn clamp(map_config: Res<MapConfig>, mut query: Query<(&mut Transform, &Bounded, &Sprite)>) {
    let map_max_pos = Vec3::new(
        map_config.width as f32 / 2.,
        map_config.height as f32 / 2.,
        0.,
    );
    let map_min_pos = -map_max_pos;
    for (mut transform, bounded, sprite) in query.iter_mut() {
        let min = bounded.min.unwrap_or(map_min_pos);
        let max = bounded.max.unwrap_or(map_max_pos);

        let sprite_size = sprite.custom_size.unwrap_or(Vec2::ZERO);
        let (sprite_half_w, sprite_half_h) = (sprite_size.x / 2., sprite_size.y / 2.);

        transform.translation.x = transform
            .translation
            .x
            .clamp(min.x + sprite_half_w, max.x - sprite_half_w);
        transform.translation.y = transform
            .translation
            .y
            .clamp(min.y + sprite_half_h, max.y - sprite_half_h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::MapConfig;

    const MAP_CONFIG: MapConfig = MapConfig {
        width: 100,
        height: 100,
        tile_size: 10,
    };
    const START_POS: Vec3 = Vec3::new(MAP_CONFIG.width as f32, MAP_CONFIG.height as f32, 0.0);

    fn setup_app(bounded: Option<Bounded>) -> App {
        // App setup
        let mut app = App::new();
        app.insert_resource(MAP_CONFIG.clone())
            .add_systems(Update, clamp);

        // Add single entity
        let entity = app
            .world_mut()
            .spawn((Transform::from_translation(START_POS), Sprite::default()))
            .id();

        // Insert the `Bounded` component, if given
        if let Some(bounded) = bounded {
            app.world_mut().entity_mut(entity).insert(bounded);
        }

        app.update();
        app
    }

    fn assert_in_bounds(pos: &Vec3, min: &Vec3, max: &Vec3) {
        assert!(
            (min.x..=max.x).contains(&pos.x) && (min.y..=max.y).contains(&pos.y),
            "Position {:?} is out of bounds: min {:?}, max {:?}",
            pos,
            min,
            max
        );
    }

    #[test]
    fn clamp_default_bound() {
        let mut app = setup_app(Some(Bounded::default()));
        let mut query = app.world_mut().query::<&Transform>();
        let transform = query
            .single(app.world())
            .expect("Failed to get single entity's transform");

        assert_in_bounds(
            &transform.translation,
            &Vec3::new(
                -(MAP_CONFIG.width as f32) / 2.0,
                -(MAP_CONFIG.height as f32) / 2.0,
                0.,
            ),
            &Vec3::new(
                MAP_CONFIG.width as f32 / 2.0,
                MAP_CONFIG.height as f32 / 2.0,
                0.,
            ),
        );
    }

    #[test]
    fn clamp_custom_bound() {
        let max_pos = Vec3::new(
            MAP_CONFIG.width as f32 / 10.,
            MAP_CONFIG.height as f32 / 10.,
            0.,
        );
        let mut app = setup_app(Some(Bounded {
            min: Some(-max_pos),
            max: Some(max_pos),
        }));

        let mut query = app.world_mut().query::<&Transform>();
        let transform = query
            .single(app.world())
            .expect("Failed to get single entity's transform");

        assert_in_bounds(&transform.translation, &(-max_pos), &max_pos)
    }

    #[test]
    fn no_clamp_for_unbounded_entities() {
        let mut app = setup_app(None);

        let mut query = app.world_mut().query::<&Transform>();
        let transform = query
            .single(app.world())
            .expect("Failed to get single entity's transform");

        assert_eq!(transform.translation, START_POS);
    }
}
