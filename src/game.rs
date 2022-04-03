use bevy::prelude::*;
use crate::ui;

#[derive(Default)]
pub struct GameState {
    messages: Vec<TextMessage>,
    // Option<...> for the Default trait
    pub text_msg_parent: Option<Entity>,
    pub date: i32,
    last_msg_date: i32,
}

struct TextMessage {
    text: String,
    sender: String,
    e: Option<Entity>,
}

#[derive(Component)]
struct TextMessagePosition {
    pub bottom: f32,
}

pub fn logic(mut commands: Commands, mut state: ResMut<GameState>, time: Res<Time>,

        asset_server: Res<AssetServer>,
                 ) {
    state.date = 1 + (time.seconds_since_startup() / 5.) as i32;

    if state.last_msg_date != state.date {
        if state.date % 2 == 1 {
            state.add_text_message(
                "DROBGob Pathology",
                "test",
            );
        } else {
            state.add_text_message(
                "DROBGob Pathology",
                "Swab collection date: 3/3/2020|Result: Covid-19 virus NEGATIVE|Tele-consult your doctor for advice applicable to your particular circumstances",
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

        let message_font_size = 30.;
        let sender_font_size = 24.;
        let line_spacing = 2.;
        let inter_message_spacing = 20.;

        let text_style_sender = TextStyle{
            font: asset_server.load("fonts/monofonto.ttf"),
            font_size: sender_font_size,
            color: Color::rgb(1., 0., 0.),
        };
        let text_style_message = TextStyle{
            font: asset_server.load("fonts/monofonto.ttf"),
            font_size: message_font_size,
            color: Color::rgb(1., 0., 0.),
        };
        let align = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Left,
        };

        // this positions the containing sprite, so again we need /3 not /6
        let rhs_left = 0. + (super::win_width()/3.);
        let message_bubble_width = 275.;
        let sender_ofs = -message_bubble_width/2. + 10.;
        let message_ofs = -message_bubble_width/2. + 25.;

        // This is the "physical bottom", i.e., if we had a one pixel object, we'd position it here
        // in order to get it in the right place
        let mut bottom = (-super::win_height() / 2.) + 190.;

        for x in &mut state.messages.iter_mut().rev() {
            let laid_out_message = ui::lay_out_text_monofonto(message_font_size, message_bubble_width, &x.text);

            // Containing box
            let ct_box_height = sender_font_size + line_spacing + (laid_out_message.len() as f32 * (line_spacing + message_font_size));
            println!("c_b_h = sfs + ls + (#line * (spc + msg_f_s)) = {} + {} ({} * ({} + {})) = {}", sender_font_size, line_spacing, laid_out_message.len(), line_spacing, message_font_size, ct_box_height);

            let mut ety = commands.spawn_bundle(
                SpriteBundle{
                    sprite: Sprite{
                        color: Color::rgb(0.8, 1.0, 0.8),
                        custom_size: Some(Vec2::new(message_bubble_width, ct_box_height)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(rhs_left, bottom + ct_box_height/2., 11.,),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            );

            ety.with_children(|parent| {
                // We're going from the bottom so spawn the message first, then the sender. Note that
                // lines are drawn bottom up
                let mut inside_bottom = -ct_box_height / 2. + message_font_size / 2.;
                for l in laid_out_message.iter().rev() {
                    parent.spawn_bundle(
                        Text2dBundle{
                            text: Text::with_section(l.clone(), text_style_message.clone(), align),
                            transform: Transform {
                                translation: Vec3::new(message_ofs, inside_bottom, 11.,),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    );
                    inside_bottom += message_font_size + line_spacing;
                }

                // and now the sender
                parent.spawn_bundle(
                    Text2dBundle{
                        text: Text::with_section(x.sender.clone(), text_style_sender.clone(), align),
                        transform: Transform {
                            translation: Vec3::new(sender_ofs, inside_bottom, 11.,),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                );
                inside_bottom += sender_font_size + line_spacing;
            });

            x.e = Some(ety.id());
            bottom += ct_box_height + inter_message_spacing;
        }
        /*
        let mut e: Option<Entity> = None;
        commands.entity(state.text_msg_parent.unwrap())
            .with_children(|parent| {
                for x in &state.messages {
                    let e = Some(parent.spawn_bundle(
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                border: Rect::all(Val::Percent(3.)),
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        }
                    ).with_children(|parent| {
                        parent.spawn_bundle(
                            TextBundle {
                                style: Style {
                                    size: Size::new(Val::Px(1324.*0.25), Default::default()),
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    format!("penis {} this is some really long text to see if we can get wrapping to occur and if so see what happens and what all the situations are for the purposes of caluclating things. but ahhh allt hings considered i'm not amazingly pelased with this whole situation. Stupid covid", state.date),
                                    TextStyle {
                                        font: asset_server.load("fonts/monofonto.ttf"),
                                        font_size: 16.,
                                        color: Color::rgb(1., 0., 0.),
                                    },
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Left,
                                    }
                                ),
                                ..Default::default()
                        });
                    }).id());
                }
            });
    */
    }
}

impl GameState {
    fn add_text_message(&mut self, sender: &str, msg: &str) {
        self.messages.push(TextMessage{
            sender: String::from(sender),
            text: String::from(msg),
            e: None,
        });
    }
}
