use bevy::prelude::*;
use crate::game::*;

#[derive(Component)]
pub struct DateTag {}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    // The bundle holding the status bar i.e. the date
    commands.spawn_bundle(
        NodeBundle{
            style: Style {
                size: Size::new(Val::Percent(70.), Val::Px(30.)),
                position: Rect{ top: Val::Percent(0.), ..Default::default() },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::rgb(1., 1., 1.).into(),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    //size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    margin: Rect::all(Val::Px(5.)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "It’s Monday, 2nd March 2020",
                    TextStyle {
                        font: asset_server.load("fonts/SFPro.ttf"),
                        font_size: 20.,
                        color: Color::rgb(0., 0., 0.),
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Right,
                    }
                ),
                ..Default::default()
            }).insert(DateTag{});
        })
    ;
    // The bundle holding the RHS 30% of the display
    commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                //position: Rect{ top: Val::Px(0.), bottom: Val::Px(0.), right: Val::Percent(100.0), left: Val::Percent(70.) },
                position: Rect{ top: Val::Percent(0.), bottom: Val::Percent(0.), right: Val::Percent(100.), left: Val::Percent(70.) },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        }).with_children(|parent| {
            // the RHS background
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Percent(2.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.99, 0.65, 0.65).into(),
                    ..Default::default()
                }).with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            color: Color::rgb(0.65, 0.90, 0.65).into(),
                            ..Default::default()
                        });
                });
            })
        ;
}

pub fn update(mut query: Query<(&mut Text, &DateTag)>, state: Res<GameState>) {
    for (mut x, _) in query.iter_mut() {
        x.sections[0].value = format!("It’s {}, {}{} March 2020", march_2020_dow(state.date), state.date, english_ordinal(state.date));
    }
}

fn march_2020_dow(day: i32) -> &'static str {
    // 1 March 2020 was a Sunday
    return match day % 7 {
        0 => "Saturday",
        1 => "Sunday",
        2 => "Monday",
        3 => "Tuesday",
        4 => "Wednesday",
        5 => "Thursday",
        6 => "Friday",
        _ => "Badday",
    }
}

fn english_ordinal(day: i32) -> &'static str {
    if day % 100 == 11 || day % 100 == 12 || day % 100 == 13 {
        return "th";
    } else if day % 10 == 1 {
        return "st";
    } else if day % 10 == 2 {
        return "nd";
    } else if day % 10 == 3 {
        return "rd";
    } else {
        return "th";
    }
}
