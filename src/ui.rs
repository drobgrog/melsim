use crate::{game::*, SCREEN_WIDTH};
use bevy::prelude::*;

#[derive(Component)]
pub struct DateTag {}

// For future expansion: change the colour of the material over time
#[derive(Component)]
pub struct MentalHealthBarTag {}

#[derive(Component)]
pub struct MentalHealthCoveringTag {}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<GameState>) {
    commands.spawn_bundle(UiCameraBundle::default());
    // The bundle holding the status bar i.e. the date
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(30.)),
                position: Rect {
                    top: Val::Percent(0.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::rgb(0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "It’s Monday, 2nd March 2020",
                        TextStyle {
                            font: asset_server.load("fonts/SFPro.ttf"),
                            font_size: 20.,
                            color: Color::rgb(1., 1., 1.),
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Right,
                        },
                    ),
                    ..Default::default()
                })
                .insert(DateTag {});
        });
    // The bundle holding the RHS 30% of the display
    // The x position -- remember we translate the *centre* of the quad, so 1/3rd (not 1/6th) is
    // right
    let xpos = (SCREEN_WIDTH / 2.) - (rhs_width() / 2.);
    // this is white underneath
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("ui/white_bg.png"),
        transform: Transform {
            translation: [xpos, 0., 10.].into(),
            ..Default::default()
        },
        ..Default::default()
    });
    // and the phone texture
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("ui/phone.png"),
        transform: Transform {
            translation: [xpos, 0., 30.].into(),
            ..Default::default()
        },
        ..Default::default()
    });


    // The bundle for the "Mental Health" bar
    let main_display_height = 1000.;
    let main_display_width = 1000.;
    let mhb_bar_height = 40.;
    let mhb_below_top = 30.;

    let mhb_ypos = main_display_height / 2. - mhb_bar_height / 2. - mhb_below_top;

    commands.spawn_bundle(SpriteBundle{
        texture: asset_server.load("ui/mh_bg.png"),
        transform: Transform {
            translation: [-(SCREEN_WIDTH - main_display_width) / 2., mhb_ypos, 10.].into(),
            ..Default::default()
        },
        ..Default::default()
    });

    // The actual bar itself
    let mhb_bar_offset = 210.;
    state.mhb_the_bar = Some(commands.spawn_bundle(SpriteBundle{
        texture: asset_server.load("ui/mh_bar.png"),
        transform: Transform {
            translation: [(-SCREEN_WIDTH / 2.) + (mhb_bar_filling_width() / 2.) + mhb_bar_offset, mhb_ypos, 11.].into(),
            ..Default::default()
        },
        sprite: Sprite{
            color: Color::rgb(1., 1., 1.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(MentalHealthBarTag{}).id());


    // The white zone covering the bar. Don't ask.
    state.mhb_bar_covering = Some(commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: [(-SCREEN_WIDTH / 2.) + (mhb_bar_filling_width() / 2.) + mhb_bar_offset, mhb_ypos-1., 12.].into(),
            ..Default::default()
        },
        sprite: Sprite{
            color: Color::rgb(1., 1., 1.),
            custom_size: Some(Vec2::new(mhb_bar_filling_width() / 2., mhb_bar_filling_height())),
            ..Default::default()
        },
        ..Default::default()
    }).insert(MentalHealthCoveringTag{}).id());
}

pub fn rhs_width() -> f32 {
    324.
}

// Returns vector of lines
pub fn lay_out_text_monofonto(point_size: f32, width_px: f32, text: &String) -> Vec<String> {
    let mut last_word = 0;
    let mut start_of_line = 0;
    let mut rv: Vec<String> = vec![];
    for (i, c) in text.chars().enumerate() {
        if c == '|' {
            // forced line break
            rv.push(String::from(&text[start_of_line..i]));
            start_of_line = i + 1;
            last_word = start_of_line;
        } else if c == ' ' {
            // here we need to update the 'last word' situation
            last_word = i + 1;
        } else {
            // TODO: this breaks if start_of_line==last_word, i.e., a word is too long to fit in a
            // single line. Text and fix
            if estimate_width(point_size, 1 + i - start_of_line) > width_px {
                rv.push(String::from(&text[start_of_line..last_word]));
                start_of_line = last_word;
            }
        }
    }
    // copy the last line, if any
    if start_of_line < text.len() {
        rv.push(String::from(&text[start_of_line..]));
    }
    return rv;
}

fn estimate_width(point_size: f32, chars: usize) -> f32 {
    return 0.4417 * point_size * (chars as f32);
}

pub fn update(mut query: Query<(&mut Text, &DateTag)>, state: Res<GameState>) {
    for (mut x, _) in query.iter_mut() {
        x.sections[0].value = format!(
            "It’s {}, {}{} March 2020",
            march_2020_dow(state.date),
            state.date,
            english_ordinal(state.date)
        );
    }
}

pub fn update_mental_health_bar_covering(mut query: Query<(&mut Sprite, &mut Transform, &MentalHealthCoveringTag)>, state: Res<GameState>) {
    let (mut sprite, mut tx, _) = query.single_mut();

    let width = mhb_bar_filling_width() * (1. - state.mental_health);

    sprite.custom_size = Some(Vec2::new(width, mhb_bar_filling_height()));
    tx.translation = [
        (-SCREEN_WIDTH / 2.) + (mhb_bar_filling_width()) + 210. - (width/2.),
        tx.translation.y,
        tx.translation.z,
    ].into();

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
    };
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

fn mhb_bar_filling_width() -> f32 { 721. }
fn mhb_bar_filling_height() -> f32 { 28. }
