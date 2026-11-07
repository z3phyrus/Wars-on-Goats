use crate::{
    game::Spell,
    types::{AbilityArea, AbilityDefinition, AbilityKind, AbilityTarget, AreaAnchor, StatusEffect},
};

pub struct ArcBlast {
    definition: AbilityDefinition,
}

impl Default for ArcBlast {
    fn default() -> Self {
        Self {
            definition: AbilityDefinition {
                id: "arc-blast".to_string(),
                name: "Arc Blast".to_string(),
                kind: AbilityKind::AreaOfEffect,
                target: AbilityTarget::Enemy,
                range: 420.0,
                cost: 15.0,
                cooldown_ms: 500,
                cast_time_ms: 0,
                duration_ms: 3000,
                follows_caster: false,
                area: AbilityArea::Circle {
                    radius: 90.0,
                    anchor: AreaAnchor::AimPoint,
                },
                effects: vec![StatusEffect::Slow],
                min_damage: 12.0,
                max_damage: 22.0,
                color: "#fbbf24".to_string(),
            },
        }
    }
}

impl Spell for ArcBlast {
    fn definition(&self) -> &AbilityDefinition {
        &self.definition
    }
}

pub fn spell_by_id(id: &str) -> Option<Box<dyn Spell>> {
    match id {
        "arc-blast" => Some(Box::new(ArcBlast::default())),
        _ => None,
    }
}
