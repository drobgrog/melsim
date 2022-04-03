use crate::{narrative, ui, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::narrative::{NarrativeEvent,NarrativeCriterion,NarrativeActions};
use bevy::prelude::*;
use crate::player::Player;
use crate::pickup;

#[derive(Default)]
pub struct GameState {
    messages: Vec<TextMessage>,
    // Option<...> for the Default trait
    pub text_msg_parent: Option<Entity>,
    pub date: i32,
    pub last_date: i32,
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

    // Narrative control
    main_narrative: Vec<NarrativeEvent>,
    covid_narrative: Vec<NarrativeEvent>,
    narrative_start_of_act: usize,
    next_narrative_id: usize,
    next_covid_narrative_id: usize,
    in_covid_narrative: bool,
    narrative_last_event: f64,
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

    state.main_narrative = narrative::make_main_narrative();
    state.covid_narrative = narrative::make_main_narrative();
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

    state.run_narrative(
        &time,
        &mut commands,
        &asset_server,
        &player,
    );
}

// How often should we lose (/gain) sanity just for existing?
fn time_for_sanity_loss() -> f64 { 5. }

// How much sanity do we lose then?
fn sanity_loss_tick() -> i32 { -1 }

impl GameState {
    fn add_text_message(&mut self,
        sender: &str,
        msg: &str,
        time: &Res<Time>,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        self.messages.push(TextMessage {
            sender: String::from(sender),
            text: String::from(msg),
            e: None,
        });
        self.last_msg_animation_time = time.seconds_since_startup();

        // Trigger a full rebuild -- delete everything else
        for x in &mut self.messages {
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
        let message_bubble_width = 235.;
        let sender_ofs = -message_bubble_width / 2. + 10.;
        let message_padding_left = -message_bubble_width / 2. + 25.;
        let message_padding_right = 10.;

        // This is the "physical bottom", i.e., if we had a one pixel object, we'd position it here
        // in order to get it in the right place
        let mut bottom = (-SCREEN_HEIGHT / 2.) + 190.;
        let mut height_of_first = 0.;

        for x in &mut self.messages.iter_mut().rev() {
            let laid_out_message =
                ui::lay_out_text_monofonto(message_font_size, message_bubble_width - message_padding_right, &x.text);

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
                            translation: Vec3::new(message_padding_left, inside_bottom, 11.),
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

    fn run_narrative(&mut self,
        time: &Res<Time>,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        player: &Query<(&Player, &Transform)>
    ) {
        if self.in_covid_narrative && self.next_covid_narrative_id >= self.covid_narrative.len() {
            // end of the covid narrative, so switch back to the regular narrative
            self.in_covid_narrative = false;
            self.next_covid_narrative_id = 0;
            // fall through
        }

        if self.in_covid_narrative {
            panic!("not implemented");
        } else if self.next_narrative_id >= self.main_narrative.len() {
            println!("Uh-oh, got to the end of the narrative!");
        } else {
            if self.criterion_met(&self.main_narrative[self.next_narrative_id].criterion, time) {
                self.do_narrative_actions(
                    self.main_narrative[self.next_narrative_id].action.clone(),
                    time,
                    commands,
                    asset_server,
                    player,
                );
                if self.main_narrative[self.next_narrative_id].starts_act {
                    self.narrative_start_of_act = self.next_narrative_id;
                }
                self.narrative_last_event = time.seconds_since_startup();
                self.next_narrative_id += 1;
            }
        }
    }

    fn criterion_met(&self, c: &NarrativeCriterion, time: &Res<Time>) -> bool {
        return match c {
            NarrativeCriterion::ElapsedRel(v) => time.seconds_since_startup() - self.narrative_last_event > *v,
            NarrativeCriterion::ClearedAll => panic!(" all cleareD?"),
        };
    }

    fn do_narrative_actions(&mut self,
        a: NarrativeActions,
        time: &Res<Time>,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        player: &Query<(&Player, &Transform)>
    ) {
        if let Some(ds) = a.change_sanity {
            self.sanity += ds;
            let (_, player_tx) = player.single();
            ui::spawn_sanity_number(ds, commands, asset_server.load("fonts/monofonto.ttf"), player_tx.translation);
        }

        for m in a.send_texts {
            self.add_text_message(
                &m.sender,
                &m.body,
                time,
                commands,
                asset_server,
            );
        }

        for s in a.spawn_item {
            pickup::spawn_pickup(s.prototype, [s.location.0, s.location.1], commands, asset_server);
        }
    }
}
