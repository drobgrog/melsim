use crate::environment::{Environment, Location};
use crate::music::MusicState;
use crate::narrative::{NarrativeActions, NarrativeCriterion, NarrativeEvent};
use crate::player::Player;
use crate::sfx::{SFXSystem, SoundEffect};
use crate::{environment, narrative, teleportation, ui, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::{npc, pickup};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const STARTING_SANITY: i32 = 100;
const COVID_RISK_THRESHOLD: f32 = 0.05;

pub struct AreaAccessControl {
    home: bool,
    park: bool,
    shops: bool,
}

#[derive(Default)]
pub struct GameState {
    messages: Vec<TextMessage>,
    // Option<...> for the Default trait
    pub text_msg_parent: Option<Entity>,
    pub date: i32,
    pub last_date: i32,
    pub last_msg_animation_time: f64,

    pub area_access: AreaAccessControl,

    // Sanity related information
    sanity: i32,
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
    game_over_image: Handle<Image>,
    game_over: bool,
    game_over_image_entity: Option<Entity>,
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
    mut music_state: ResMut<MusicState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    player: Query<(&Player, &Transform)>,
) {
    if key.just_pressed(KeyCode::C) {
        state.show_covid_risk = !state.show_covid_risk;
        state.last_covid_risk_shown = time.seconds_since_startup();
    }
    if key.just_pressed(KeyCode::V) {
        state.covid_risk += 0.1;
    }
    if key.just_pressed(KeyCode::B) {
        state.covid_risk -= 0.1;
    }
    if key.just_pressed(KeyCode::P) {
        let (_, player_tx) = player.single();
        state.change_sanity(3);
        ui::spawn_sanity_number(
            3,
            &mut commands,
            asset_server.load("fonts/monofonto.ttf"),
            player_tx.translation,
        );
    }
    if key.just_pressed(KeyCode::O) {
        let (_, player_tx) = player.single();
        ui::spawn_sanity_number(
            -7,
            &mut commands,
            asset_server.load("fonts/monofonto.ttf"),
            player_tx.translation,
        );
    }
    if key.just_pressed(KeyCode::M) {
        println!("M PRESSED");
        let next_index = if music_state.changing_from == 0 { 1 } else { 0 };
        music_state.switch_tracks(next_index);
    }

    if key.just_pressed(KeyCode::G) {
        state.change_sanity(-100);
    }
}

pub fn setup_state(mut state: ResMut<GameState>, asset_server: Res<AssetServer>) {
    state.setup(&asset_server);
}

pub fn logic(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    player: Query<(&Player, &Transform)>,
    pickups_query: Query<(&pickup::Pickup,)>,
    environment_query: Query<(&environment::Environment,)>,
    mut sfx_system: ResMut<SFXSystem>,
) {
    if state.sanity <= 0 {
        game_over(&mut commands, &mut state);
        return;
    }

    state.date = 1 + (time.seconds_since_startup() / 5.) as i32;
    if state.last_date < state.date {
        state.last_date = state.date;
        state.new_day();
    }

    let (environment,) = environment_query.single();
    let sanity_change = state.deduct_sanity_on_timer(time.seconds_since_startup(), environment);
    if sanity_change != 0 {
        let (_, player_tx) = player.single();
        ui::spawn_sanity_number(
            sanity_change,
            &mut commands,
            asset_server.load("fonts/monofonto.ttf"),
            player_tx.translation,
        );
        if sanity_change > 0 {
            sfx_system.play_sfx(SoundEffect::SanityUp);
        } else {
            sfx_system.play_sfx(SoundEffect::SanityDown);
        }
    }

    state.run_narrative(
        &time,
        &mut commands,
        &asset_server,
        &player,
        &pickups_query,
        &environment_query,
        &mut sfx_system,
    );
}

fn game_over(commands: &mut Commands, state: &mut GameState) {
    if !state.game_over {
        state.game_over = true;
        state.game_over_image_entity = Some(
            commands
                .spawn_bundle(SpriteBundle {
                    texture: state.game_over_image.clone(),
                    transform: Transform {
                        translation: [0., -15., 100.].into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id(),
        );
    }

    // Do nothing, and wait for the player to close the window.
}

// How often should we lose (/gain) sanity just for existing?
fn time_for_sanity_loss() -> f64 {
    10.
}

// How much sanity do we lose then?
fn sanity_loss_tick() -> i32 {
    -1
}

impl GameState {
    fn setup(&mut self, asset_server: &Res<AssetServer>) {
        self.sanity = STARTING_SANITY;
        self.covid_risk = 0.5;
        self.main_narrative = narrative::load_csv("./narrative/main.csv");
        self.covid_narrative = narrative::load_csv("./narrative/covid.csv");
        self.game_over_image = asset_server.load("game_over.png");
        let _dummy: Handle<Image> = asset_server.load("close_contact_alert.png");
    }

    fn add_text_message(
        &mut self,
        sender: &str,
        msg: &str,
        time: &Res<Time>,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        sfx_system: &mut SFXSystem,
    ) {
        self.messages.push(TextMessage {
            sender: String::from(sender),
            text: String::from(msg),
            e: None,
        });
        self.last_msg_animation_time = time.seconds_since_startup();

        sfx_system.play_sfx(SoundEffect::Text);

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
            color: Color::rgba(0., 0., 0., 1.),
        };
        let text_style_message = TextStyle {
            font: asset_server.load("fonts/monofonto.ttf"),
            font_size: message_font_size,
            color: Color::rgba(0., 0., 0., 1.),
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
            let laid_out_message = ui::lay_out_text_monofonto(
                message_font_size,
                message_bubble_width - message_padding_right,
                &x.text,
            );

            // Containing box
            let ct_box_height = sender_font_size
                + line_spacing
                + (laid_out_message.len() as f32 * (line_spacing + message_font_size))
                + 4.;

            if height_of_first == 0. {
                height_of_first = ct_box_height + inter_message_spacing;
            }

            let ctr_bottom = bottom + ct_box_height / 2.;
            let mut ety = commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(175. / 255., 233. / 255., 198. / 255.),
                    custom_size: Some(Vec2::new(message_bubble_width, ct_box_height)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(msg_xpos, ctr_bottom - height_of_first, 11.),
                    ..Default::default()
                },
                ..Default::default()
            });
            ety.insert(ui::TextMessageTag {
                bottom_from: ctr_bottom - height_of_first,
                bottom_to: ctr_bottom,
            });

            ety.with_children(|parent| {
                // pretty bubble edges
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("ui/top_bubble.png"),
                    transform: Transform {
                        translation: [0., ct_box_height / 2. - 50. / 2., 11.5].into(),
                        ..Default::default()
                    },
                    ..Default::default()
                });
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("ui/bottom_bubble.png"),
                    transform: Transform {
                        translation: [0., -ct_box_height / 2. + 14. / 2., 11.55].into(),
                        ..Default::default()
                    },
                    ..Default::default()
                });

                // We're going from the bottom so spawn the message first, then the sender. Note that
                // lines are drawn bottom up
                let mut inside_bottom = -ct_box_height / 2. + message_font_size / 2. + 2.;
                for l in laid_out_message.iter().rev() {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::with_section(l.clone(), text_style_message.clone(), align),
                        transform: Transform {
                            translation: Vec3::new(message_padding_left, inside_bottom, 11.6),
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
                        translation: Vec3::new(sender_ofs, inside_bottom, 11.6),
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

    fn new_day(&mut self) {}

    // Returns the change, if any (so it can be displayed to the user)
    fn deduct_sanity_on_timer(&mut self, time_since_start: f64, environment: &Environment) -> i32 {
        // Only reduce sanity if we're at home.
        if environment.location != Location::Home {
            return 0;
        }

        if time_since_start - self.last_sanity_tick_update > time_for_sanity_loss() {
            self.last_sanity_tick_update += time_for_sanity_loss();
            self.sanity += sanity_loss_tick();
            return sanity_loss_tick();
        }
        return 0;
    }

    fn run_narrative(
        &mut self,
        time: &Res<Time>,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        player_query: &Query<(&Player, &Transform)>,
        pickups_query: &Query<(&pickup::Pickup,)>,
        environment_query: &Query<(&environment::Environment,)>,
        sfx_system: &mut SFXSystem,
    ) {
        if self.in_covid_narrative && self.next_covid_narrative_id >= self.covid_narrative.len() {
            // end of the covid narrative, so switch back to the regular narrative
            self.in_covid_narrative = false;
            self.next_covid_narrative_id = 0;
            // fall through
        }

        if self.in_covid_narrative {
            if self.criterion_met(
                &self.covid_narrative[self.next_covid_narrative_id].criterion,
                pickups_query,
                environment_query,
                time,
            ) {
                let (_, player_tx) = player_query.single();
                self.do_narrative_actions(
                    self.covid_narrative[self.next_covid_narrative_id]
                        .action
                        .clone(),
                    time,
                    commands,
                    asset_server,
                    player_tx,
                    sfx_system,
                );
                self.narrative_last_event = time.seconds_since_startup();
                self.next_covid_narrative_id += 1;
            }

            if self.next_covid_narrative_id > self.covid_narrative.len() {
                // switch back to main narrative
                self.in_covid_narrative = false;
            }
        } else if self.next_narrative_id >= self.main_narrative.len() {
            println!("Uh-oh, got to the end of the narrative!");
        } else {
            if self.criterion_met(
                &self.main_narrative[self.next_narrative_id].criterion,
                pickups_query,
                environment_query,
                time,
            ) {
                let (_, player_tx) = player_query.single();
                self.do_narrative_actions(
                    self.main_narrative[self.next_narrative_id].action.clone(),
                    time,
                    commands,
                    asset_server,
                    player_tx,
                    sfx_system,
                );
                if self.main_narrative[self.next_narrative_id].starts_act {
                    self.narrative_start_of_act = self.next_narrative_id;
                }
                self.narrative_last_event = time.seconds_since_startup();
                self.next_narrative_id += 1;
            }
        }
    }

    fn criterion_met(
        &self,
        c: &NarrativeCriterion,
        pickups_query: &Query<(&pickup::Pickup,)>,
        environment_query: &Query<(&environment::Environment,)>,
        time: &Res<Time>,
    ) -> bool {
        return match c {
            NarrativeCriterion::ElapsedRel(v) => {
                time.seconds_since_startup() - self.narrative_last_event > *v
            }
            NarrativeCriterion::ClearedAll => pickups_query.is_empty(),
            NarrativeCriterion::InEnvironment(l) => {
                let (current_env,) = environment_query.single();
                &current_env.location == l
            }
        };
    }

    pub fn change_sanity(&mut self, delta: i32) {
        // no need to clamp on the bottom -- that ends the game
        self.sanity = i32::min(self.sanity + delta, 100);
    }

    pub fn get_sanity(&self) -> i32 {
        return self.sanity;
    }

    pub fn do_narrative_actions(
        &mut self,
        a: NarrativeActions,
        time: &Res<Time>,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        player_tx: &Transform,
        sfx_system: &mut SFXSystem,
    ) {
        if let Some(ds) = a.change_sanity {
            self.change_sanity(ds);
            ui::spawn_sanity_number(
                ds,
                commands,
                asset_server.load("fonts/monofonto.ttf"),
                player_tx.translation,
            );
            if ds > 0 {
                sfx_system.play_sfx(SoundEffect::SanityUp);
            } else {
                sfx_system.play_sfx(SoundEffect::SanityDown);
            }
        }

        for m in a.send_texts {
            self.add_text_message(&m.sender, &m.body, time, commands, asset_server, sfx_system);
        }

        for s in a.spawn_item {
            pickup::spawn_pickup(
                s.prototype,
                [s.location.0, s.location.1],
                commands,
                asset_server,
                s.narrative_actions,
            );
        }

        for s in a.spawn_npc {
            npc::spawn_npc(commands, asset_server, s.location);
        }

        for (l, new_val) in a.teleporter_control {
            self.area_access.set_access(l, new_val);
        }
    }

    pub fn set_covid_risk(&mut self, covid_risk: f32, time: &Res<Time>) {
        let old_scr = self.show_covid_risk;
        self.covid_risk = covid_risk;
        if covid_risk > COVID_RISK_THRESHOLD {
            self.show_covid_risk = true;
        } else {
            self.show_covid_risk = false;
        }

        if old_scr != self.show_covid_risk {
            self.last_covid_risk_shown = time.seconds_since_startup();
        }
    }

    pub fn covid_narrative_switch(
        &mut self,
        time: &Res<Time>,
        player_position: &mut Mut<RigidBodyPositionComponent>,
        environment_query: &mut Query<(&mut TextureAtlasSprite, &mut environment::Environment)>,
        commands: &mut Commands,
        environment_collider_query: &Query<Entity, With<environment::EnvironmentCollider>>,
        music_state: &mut ResMut<MusicState>,
        asset_server: &Res<AssetServer>,
    ) {
        // The player has been exposed to covid so we need to switch over to the covid narrative,
        // and handle a million housekeeping questions

        // Narrative stuff
        self.in_covid_narrative = true;
        // when we return to the main narrative, back up to the start of the last act
        self.next_narrative_id = self.narrative_start_of_act;
        self.narrative_last_event = time.seconds_since_startup(); // establish the start of the Covid arc

        // Teleport back home
        let teleporter = teleportation::Teleporter::new(environment::Location::Home, [5, 5].into());
        teleportation::teleport(
            &teleporter,
            player_position,
            environment_query,
            commands,
            environment_collider_query,
            music_state,
            asset_server,
        );

        // Spawn the scary transition screen
        let xpos = -SCREEN_WIDTH / 2. + 1000. / 2.;
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("close_contact_alert.png"),
                transform: Transform {
                    translation: [xpos, 0., 50.].into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ui::CovidTransitionUiTag {
                time_left: ui::TRANSITION_LENGTH,
            });
    }
}

impl Default for AreaAccessControl {
    fn default() -> Self {
        AreaAccessControl{
            home: true,
            park: true,
            shops: true,
        }
    }
}

impl AreaAccessControl {
    pub fn can_access(&self, l: environment::Location) -> bool {
        match l {
            environment::Location::Home => self.home,
            environment::Location::Park => self.park,
            environment::Location::Shops => self.shops,
        }
    }

    pub fn set_access(&mut self, l: environment::Location, to: bool) {
        println!("access control: set {:?} to {}", l, if to { "unlocked" } else { "locked" });
        match l {
            environment::Location::Home => self.home = to,
            environment::Location::Park => self.park = to,
            environment::Location::Shops => self.shops = to,
        };
    }
}
