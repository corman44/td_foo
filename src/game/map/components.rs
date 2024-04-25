use bevy::prelude::*;
use bevy_ecs_ldtk::{ldtk::ldtk_fields::LdtkFields, EntityIid, EntityInstance};

#[derive(Component)]
pub struct AnimateRotation;

// #[derive(Debug,Component, Deref, DerefMut, Default)]
// pub struct UnresolvedMotherRef(Option<EntityIid>);

// impl UnresolvedMotherRef {
//     pub fn from_mother_field(entity_instance: &EntityInstance) -> UnresolvedMotherRef {
//         UnresolvedMotherRef(
//             entity_instance
//                 .get_maybe_entity_ref_field("mother")
//                 .expect("expected entity to have mother entity ref field")
//                 .as_ref()
//                 .map(|entity_ref| EntityIid::new(entity_ref.entity_iid.clone())),
//         )
//     }
// }