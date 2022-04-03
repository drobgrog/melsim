use crate::{ui, SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::prelude::*;
use crate::player::Player;

#[derive(Default)]
pub struct GameState {
    messages: Vec<TextMessage>,
    // Option<...> for the Default trait
    pub text_msg_parent: Option<Entity>,
    pub date: i32,
    pub last_date: i32,
    last_msg_date: i32,
    pub last_msg_animation_time: f64,

    // Sanity related information
    pub sanity: i32,
    // The last time sanity changed due to the passage of time
    // This gets updated (a) when we change sanity, or (b) when we switch environment
    pub last_sanity_tick_update: f64,

    // Covid risk related information
    pub show_covid_risk: bool,
    pub covid_risk: f32,
    pub last_covid_risk_shown: f64,
}

struct TextMessage {
    text: String,
    sender: String,
    e: Option<Entity>,
}

pub fn debug_keys(
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
    mut state: ResMut<GameState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    player: Query<(&Player, &Transform)>,
) {
    if key.just_pressed(KeyCode::C) {
        state.show_covid_risk = !state.show_covid_risk;
        state.last_covid_risk_shown  = time.seconds_since_startup();
    }
    if key.just_pressed(KeyCode::V) {
        state.covid_risk += 0.1;
    }
    if key.just_pressed(KeyCode::B) {
        state.covid_risk -= 0.1;
    }
    if key.just_pressed(KeyCode::P) {
        let (_, player_tx) = player.single();
        ui::spawn_sanity_number(3, &mut commands, asset_server.load("fonts/monofonto.ttf"), player_tx.translation);
    }
    if key.just_pressed(KeyCode::O) {
        let (_, player_tx) = player.single();
        ui::spawn_sanity_number(-7, &mut commands, asset_server.load("fonts/monofonto.ttf"), player_tx.translation);
    }
}

pub fn setup_state(mut state: ResMut<GameState>) {
    state.sanity = 75;

    state.covid_risk = 0.5;

    // `date` handled automatically by `logic`
}

pub fn logic(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    player: Query<(&Player, &Transform)>,
) {
    state.date = 1 + (time.seconds_since_startup() / 5.) as i32;
    if state.last_date < state.date {
        state.last_date = state.date;
        state.new_day();
    }

    let sanity_change = state.update_sanity(time.seconds_since_startup());
    if sanity_change != 0 {
        let (_, player_tx) = player.single();
        ui::spawn_sanity_number(sanity_change, &mut commands, asset_server.load("fonts/monofonto.ttf"), player_tx.translation);
    }

    if state.last_msg_date != state.date {
        if state.date % 2 == 1 {
            state.add_text_message("DROBGob Pathology", "test", time.seconds_since_startup());
        } else {
            state.add_text_message(
                "DROBGob Pathology",
                "Swab collection date: 3/3/2020|Result: Covid-19 virus NEGATIVE|Tele-consult your doctor for advice applicable to your particular circumstances",
                time.seconds_since_startup()
            );
        }

        state.last_msg_date = state.date;
        // Trigger a full rebuild -- delete everything else
        for x in &mut state.messages {
            if let Some(ety) = x.e {
                commands.entity(ety).despawn_recursive();
                x.e = None;
            }
        }

        let message_font_size = 24.;
        let sender_font_size = 18.;
        let line_spacing = 2.;
        let inter_message_spacing = 20.;

        let text_style_sender = TextStyle {
            font: asset_server.load("fonts/monofonto.ttf"),
            font_size: sender_font_size,
            color: Color::rgba(1., 0., 0., 1.),
        };
        let text_style_message = TextStyle {
            font: asset_server.load("fonts/monofonto.ttf"),
            font_size: message_font_size,
            color: Color::rgba(1., 0., 0., 1.),
        };
        let align = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Left,
        };

        let msg_xpos = (SCREEN_WIDTH / 2.) - (ui::rhs_width() / 2.);
        let message_bubble_width = 215.;
        let sender_ofs = -message_bubble_width / 2. + 10.;
        let message_ofs = -message_bubble_width / 2. + 25.;

        // This is the "physical bottom", i.e., if we had a one pixel object, we'd position it here
        // in order to get it in the right place
        let mut bottom = (-SCREEN_HEIGHT / 2.) + 190.;
        let mut height_of_first = 0.;

        for x in &mut state.messages.iter_mut().rev() {
            let laid_out_message =
                ui::lay_out_text_monofonto(message_font_size, message_bubble_width, &x.text);

            // Containing box
            let ct_box_height = sender_font_size
                + line_spacing
                + (laid_out_message.len() as f32 * (line_spacing + message_font_size));

            if height_of_first == 0. {
                height_of_first = ct_box_height + inter_message_spacing;
            }

            let ctr_bottom = bottom + ct_box_height / 2.;
            let mut ety = commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 1.0, 0.8),
                    custom_size: Some(Vec2::new(message_bubble_width, ct_box_height)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(msg_xpos, ctr_bottom - height_of_first, 11.),
                    ..Default::default()
                },
                ..Default::default()
            });
            ety.insert(ui::TextMessageTag{
                bottom_from: ctr_bottom - height_of_first,
                bottom_to: ctr_bottom,
            });

            ety.with_children(|parent| {
                // We're going from the bottom so spawn the message first, then the sender. Note that
                // lines are drawn bottom up
                let mut inside_bottom = -ct_box_height / 2. + message_font_size / 2.;
                for l in laid_out_message.iter().rev() {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::with_section(l.clone(), text_style_message.clone(), align),
                        transform: Transform {
                            translation: Vec3::new(message_ofs, inside_bottom, 11.),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    inside_bottom += message_font_size + line_spacing;
                }

                // and now the sender
                parent.spawn_bundle(Text2dBundle {
                    text: Text::with_section(x.sender.clone(), text_style_sender.clone(), align),
                    transform: Transform {
                        translation: Vec3::new(sender_ofs, inside_bottom, 11.),
                        ..Default::default()
                    },
                    ..Default::default()
                });
                // Don't need this calculation but included for if we add things
                //inside_bottom += sender_font_size + line_spacing;
            });

            x.e = Some(ety.id());
            bottom += ct_box_height + inter_message_spacing;

            // could be S_H / 2, but we need to be a bit careful here because we over display. So
            // just go whole hog and don't divide by 2
            if bottom > SCREEN_HEIGHT / 1. {
                // don't need to render any more
                break;
            }
        }
    }
}

// How often should we lose (/gain) sanity just for existing?
fn time_for_sanity_loss() -> f64 { 5. }

// How much sanity do we lose then?
fn sanity_loss_tick() -> i32 { -1 }

impl GameState {
    fn add_text_message(&mut self, sender: &str, msg: &str, time: f64) {
        self.messages.push(TextMessage {
            sender: String::from(sender),
            text: String::from(msg),
            e: None,
        });
        self.last_msg_animation_time = time;
    }

    fn new_day(&mut self) {
    }

    // Returns the change, if any (so it can be displayed to the user)
    fn update_sanity(&mut self, time_since_start: f64) -> i32 {
        if time_since_start - self.last_sanity_tick_update > time_for_sanity_loss() {
            self.last_sanity_tick_update += time_for_sanity_loss();
            self.sanity += sanity_loss_tick();
            return sanity_loss_tick();
        }
        return 0;
    }
}
