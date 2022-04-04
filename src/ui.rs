use crate::{game::*, SCREEN_WIDTH, SCREEN_HEIGHT};
use bevy::prelude::*;

#[derive(Component)]
pub struct DateTag {}

// For future expansion: change the colour of the material over time
#[derive(Component)]
pub struct SanityBarTag {}

#[derive(Component)]
pub struct SanityCoveringTag {}

#[derive(Component)]
pub struct CovidRiskElement {
    min_risk: f32,
}

#[derive(Component)]
pub struct SanityNumberTween {
    total_time: f32,
    time_left: f32,
    base_x: f32,
    base_y: f32,
}

#[derive(Component)]
pub struct TextMessageTag {
    pub bottom_from: f32,
    pub bottom_to: f32,
}

#[derive(Component)]
pub struct CovidTransitionUiTag {
    pub time_left: f32,
}

pub const TRANSITION_LENGTH: f32 = 3.;

pub fn covid_transition_ui(
    mut commands: Commands,
    mut query: Query<(&mut CovidTransitionUiTag, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut ctt, mut tx, e) in query.iter_mut() {
        ctt.time_left -= time.delta_seconds();
        if ctt.time_left < 0. {
            commands.entity(e).despawn();
        } else if ctt.time_left < 1. {
            tx.scale.x = ctt.time_left;
            tx.scale.y = ctt.time_left;
            let rot_fact = 3. * (1. - ctt.time_left);
            tx.rotation = Quat::from_rotation_z(rot_fact * rot_fact);
        }
    }
}

pub fn spawn_sanity_number(
    number: i32,
    commands: &mut Commands,
    font: Handle<Font>,
    player_location: Vec3, // Vec3 so we can pass a translation directly
) {
    let col = if number == 0 {
        Color::rgba(0., 0., 0., 1.)
    } else if number > 0 {
        Color::rgba(0., 0.7, 0., 1.)
    } else {
        Color::rgba(0.7, 0., 0., 1.)
    };
    let sgn = if number == 0 {
        ""
    } else if number > 0 {
        "+"
    } else {
        "-"
    };
    let text_style = TextStyle{
        font: font,
        font_size: 36.,
        color: col,
    };
    let align = TextAlignment{
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Left,
    };
    commands.spawn_bundle(Text2dBundle{
        text: Text::with_section(format!("{}{}", sgn, i32::abs(number)), text_style, align),
        transform: Transform {
            translation: player_location,
            ..Default::default()
        },
        ..Default::default()
    }).insert(SanityNumberTween{
        total_time: 1.2,
        time_left: 1.2,
        base_x: player_location.x,
        base_y: player_location.y,
    });
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            translation: [xpos, -15., 10.].into(),
            ..Default::default()
        },
        ..Default::default()
    });
    // and the phone texture
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("ui/phone.png"),
        transform: Transform {
            translation: [xpos, -15., 30.].into(),
            ..Default::default()
        },
        ..Default::default()
    });


    // The bundle for the "Sanity" bar
    let main_display_width = 1000.;
    //let mhb_bar_height = 40.;
    //let mhb_below_top = 30.;

    // I don't understand why this has changed, but these pixel coordinates seem to work
    let mhb_ypos = SCREEN_HEIGHT / 2. - 60.;//. - mhb_bar_height / 2. /* - mhb_below_top;

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
    commands.spawn_bundle(SpriteBundle{
        texture: asset_server.load("ui/mh_bar.png"),
        transform: Transform {
            translation: [(-SCREEN_WIDTH / 2.) + (mhb_bar_filling_width() / 2.) + mhb_bar_offset, mhb_ypos - 1., 11.].into(),
            ..Default::default()
        },
        sprite: Sprite{
            color: Color::rgb(1., 1., 1.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(SanityBarTag{});


    // The white zone covering the bar. Don't ask.
    let desired_width = mhb_bar_filling_width() * ( (100 - STARTING_SANITY) as f32 / 100.);
    commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: [
                    (-SCREEN_WIDTH / 2.) + (mhb_bar_filling_width()) + 210. - (desired_width/2.),
                    mhb_ypos-1.,
                    12.
                ].into(),
            ..Default::default()
        },
        sprite: Sprite{
            color: Color::rgb(1., 1., 1.),
            custom_size: Some(Vec2::new(desired_width, mhb_bar_filling_height())),
            ..Default::default()
        },
        ..Default::default()
    }).insert(SanityCoveringTag{});

    // The Covid risk indicator
    // background
    commands.spawn_bundle(SpriteBundle{
        texture: asset_server.load("ui/covid_risk_bg.png"),
        transform: Transform {
            translation: [-SCREEN_WIDTH/2. + 1162., -SCREEN_HEIGHT/2. + 788., 25.].into(),
            ..Default::default()
        },
        sprite: Sprite {
            ..Default::default()
        },
        ..Default::default()
    }).insert(CovidRiskElement{
        min_risk: 0.,
    });

    for i in 0..8 {
        commands.spawn_bundle(SpriteBundle{
            texture: asset_server.load("ui/covid_risk_slice.png"),
            transform: Transform {
                translation: [-SCREEN_WIDTH/2. + 1162., -SCREEN_HEIGHT/2. + 807., 26.].into(),
                rotation: Quat::from_rotation_z(i as f32 * (std::f32::consts::PI / 4.)),
                ..Default::default()
            },
            sprite: Sprite {
                ..Default::default()
            },
            ..Default::default()
        }).insert(CovidRiskElement{
            min_risk: (i as f32) * 1./8.,
        });
    }
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

pub fn update_sanity_bar_covering(mut query: Query<(&mut Sprite, &mut Transform, &SanityCoveringTag)>, state: Res<GameState>, time: Res<Time>) {
    let (mut sprite, mut tx, _) = query.single_mut();

    let old_width = match sprite.custom_size {
        Some(v) => v.x,
        None    => panic!("??"),
    };
    let desired_width = mhb_bar_filling_width() * ( (100 - state.sanity) as f32 / 100.);

    // animate between old and new width
    // this is an semi-elastic algorithm design to go faster the further we are from the true value
    let speed = 3. * if desired_width > old_width {
        f32::ceil(desired_width - old_width)
    } else {
        -f32::ceil(old_width - desired_width)
    };
    let mut new_width = old_width + speed*time.delta_seconds();
    // if the correction overshoots, clamp it
    if old_width > desired_width {
        // we're falling, so make sure we're not too *low*
        new_width = f32::max(new_width, desired_width);
    } else {
        // opposite logic
        new_width = f32::min(new_width, desired_width);
    }

    sprite.custom_size = Some(Vec2::new(new_width, mhb_bar_filling_height()));
    tx.translation = [
        (-SCREEN_WIDTH / 2.) + (mhb_bar_filling_width()) + 210. - (new_width/2.),
        tx.translation.y,
        tx.translation.z,
    ].into();

}

pub fn update_covid_risk(mut query: Query<(&CovidRiskElement, &mut Visibility, &mut Transform)>, state: Res<GameState>, time: Res<Time>) {
    let tween_time = ease_in_out_circ((1./0.3) * f64::min(0.3, time.seconds_since_startup() - state.last_covid_risk_shown) as f32);
    for (cre, mut v, mut t) in query.iter_mut() {
        if state.show_covid_risk && state.covid_risk >= cre.min_risk {
            v.is_visible = true;
            t.scale.x = tween_time;
            t.scale.y = tween_time;
        } else {
            if tween_time > 0.99 {
                v.is_visible = false;
            } else {
                t.scale.x = 1. - tween_time;
                t.scale.y = 1. - tween_time;
            }
        }
    }
}

pub fn text_message_animator(mut query: Query<(&TextMessageTag, &mut Transform)>, state:Res<GameState>, time: Res<Time>) {
    let tween_time = ease_in_out_circ((1./0.3) * f64::min(0.3, time.seconds_since_startup() - state.last_msg_animation_time) as f32);
    //let tween_time = ((1./0.3) * f64::min(0.3, time.seconds_since_startup() - state.last_msg_animation_time) as f32);
    for (tmt, mut t) in query.iter_mut() {
        let dt = tmt.bottom_to - tmt.bottom_from;
        t.translation.y = tmt.bottom_from + dt * tween_time;
    }
}

pub fn sanity_number_tween(mut commands: Commands, mut query: Query<(&mut SanityNumberTween, &mut Transform, &mut Text, Entity)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (mut mhn, mut t, mut txt, ety) in query.iter_mut() {
        mhn.time_left -= dt;
        if mhn.time_left < 0. {
            commands.entity(ety).despawn();
        } else {
            let tween_time = (mhn.total_time - mhn.time_left)/mhn.total_time;
            let eased_tween_time = ease_in_back(tween_time);

            t.translation.y = mhn.base_y - 220.*eased_tween_time;
            t.translation.x = mhn.base_x + 150.*tween_time;

            if tween_time > 0.5 {
                let opacity = (tween_time * 2.) - 1.;
                txt.sections[0].style.color.set_a(1. - opacity);
            }
        }
    }
}

// see: https://easings.net/#easeInOutCirc
fn ease_in_out_circ(x: f32) -> f32 {
    return if x < 0.5 {
      (1. - f32::sqrt(1. - f32::powf(2. * x, 2.))) / 2.
    } else {
      (f32::sqrt(1. - f32::powf(-2. * x + 2., 2.)) + 1.) / 2.
    }
}

// see: https://easings.net/#easeInBack
fn ease_in_back(x: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.;

    return c3 * x * x * x - c1 * x * x;
}

/* might be useful in future
fn ease_out_bounce(mut x: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;

    if x < 1. / d1 {
        return n1 * x * x;
    } else if x < 2. / d1 {
        x -= 1.5 / d1;
        return n1 * (x) * x + 0.75;
    } else if x < 2.5 / d1 {
        x -= 2.25 / d1;
        return n1 * (x) * x + 0.9375;
    } else {
        x -= 2.625 / d1;
        return n1 * (x) * x + 0.984375;
    }
}
*/

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
