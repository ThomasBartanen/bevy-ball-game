use bevy::prelude::*;

use crate::game::hud::{
    components::*,
    styles::*
};

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let _hud_entity: Entity = build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<Hud>>
) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}

pub fn build_hud(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: HUD_STYLE,
                background_color: BackgroundColor(Color::Rgba {red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0}),
                ..default()
            }, 
            Hud {}
        ))

        .with_children(|parent: &mut ChildBuilder| {
            // === Title ===
            parent.spawn(NodeBundle {
                style: DATA_STYLE,
                ..default()
            })
            .with_children(|parent: &mut ChildBuilder| {
                // Score
                parent.spawn((TextBundle {
                    text: Text{
                        sections: vec![
                            TextSection::new(
                                "Score: ",
                                get_data_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }                
                .with_no_wrap(),                         
                    ScoreText { }
                ));
                // Currency
                parent.spawn((TextBundle {
                    text: Text{
                        sections: vec![
                            TextSection::new(
                                "Currency: ",
                                get_data_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
                .with_no_wrap(),
                    CurrencyText { }
                ));
                // Kill Count
                parent.spawn((TextBundle {
                    text: Text{
                        sections: vec![
                            TextSection::new(
                                "Kills: ",
                                get_data_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
                .with_no_wrap(),
                    KillText { }
                ));
                // Bomb Count
                parent.spawn((TextBundle {
                    text: Text{
                        sections: vec![
                            TextSection::new(
                                "Bombs: ",
                                get_data_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
                .with_no_wrap(),
                    BombCount { }
                ));
            });
        })

        .id();

    hud_entity
}