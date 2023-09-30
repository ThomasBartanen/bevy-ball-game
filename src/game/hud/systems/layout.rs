use bevy::prelude::*;

use crate::{
    game::hud::{
        components::*,
        styles::*
    },
    constants::{
        BUY_BOMB_KEY,
        DROP_BOMB_KEY,
        MAIN_MENU_KEY,
        PAUSE_GAME_KEY,
        CLOSE_APPLICATION_KEY
    }
};

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let _hud_entity: Entity = build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<CompleteHud>>
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
                style: COMPLETE_HUD_STYLE,
                background_color: BackgroundColor(Color::Rgba {red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0}),
                ..default()
            }, 
            CompleteHud {}
        ))
        .with_children(|parent: &mut ChildBuilder| {
            parent.spawn((NodeBundle{
                style: TOP_HUD_STYLE,
                background_color: BackgroundColor(Color::Rgba {red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0}),
                ..default()
                },            
                HudTopBar {}
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
            });
        })
        .with_children(|parent: &mut ChildBuilder| {
            parent.spawn((NodeBundle{
                style: SIDE_HUD_STYLE,
                background_color: BackgroundColor(Color::Rgba {red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0}),
                ..default()
                },            
                HudSideBar {}
            ))
            .with_children(|parent: &mut ChildBuilder| {
                // === Title ===
                parent.spawn(NodeBundle {
                    style: CONTROLS_STYLE,
                    ..default()
                })

                .with_children(|parent: &mut ChildBuilder| {
                    // Menu
                    parent.spawn((TextBundle {
                        text: Text{
                            sections: vec![
                                TextSection::new(
                                    "Main Menu: M",
                                    get_controls_text_style(&asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    },                         
                        MainMenuControl { }
                    ));
                    // Bomb Key
                    parent.spawn((TextBundle {
                        text: Text{
                            sections: vec![
                                TextSection::new(
                                    "Drop Bomb: F",
                                    get_controls_text_style(&asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    },
                        DropBombControl { }
                    ));
                    // Purchase Bomb
                    parent.spawn((TextBundle {
                        text: Text{
                            sections: vec![
                                TextSection::new(
                                    "Buy Bomb: Q",
                                    get_controls_text_style(&asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    },
                        BuyBombControl { }
                    ));
                    // Pause Game
                    parent.spawn((TextBundle {
                        text: Text{
                            sections: vec![
                                TextSection::new(
                                    "Pause: \n Enter",
                                    get_controls_text_style(&asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    },
                        PauseGameControl { }
                    ));
                });                
            });
        })
        .id();

    hud_entity
}