use crate::{ShapeSprite, TessellationMode, Tessellator};
use bevy::prelude::*;

pub mod shape_plugin_stage {
    pub const SHAPE: &str = "shape";
}

pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let tessellator = Tessellator::new();
        app.add_resource(tessellator)
            .add_stage_after(
                stage::UPDATE,
                shape_plugin_stage::SHAPE,
                SystemStage::parallel(),
            )
            .add_system_to_stage(shape_plugin_stage::SHAPE, shapesprite_maker.system());
    }
}

pub struct ShapeDescriptor {
    pub shape: Box<dyn ShapeSprite + Send + Sync>,
    pub material: Handle<ColorMaterial>,
    pub mode: TessellationMode,
    pub transform: Transform,
}

fn shapesprite_maker(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tessellator: ResMut<Tessellator>,
    query: Query<(Entity, &ShapeDescriptor)>,
) {
    for (entity, shape_descriptor) in query.iter() {
        commands
            .spawn(shape_descriptor.shape.generate_sprite(
                shape_descriptor.material.clone(),
                &mut meshes,
                &mut tessellator,
                shape_descriptor.mode.clone(),
                shape_descriptor.transform.clone(),
            ))
            .despawn(entity);
    }
}