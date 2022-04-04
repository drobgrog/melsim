use bevy::prelude::Component;

use crate::pickup;
use crate::environment::Location;

pub struct NarrativeEvent {
    pub starts_act: bool,
    pub criterion: NarrativeCriterion,
    pub action: NarrativeActions,
}

pub enum NarrativeCriterion {
    ElapsedRel(f64),         // at least this many seconds have elasped since last event
    ClearedAll,              // all items in the environment must be cleared
    InEnvironment(Location), // current location is here
}

#[derive(Default, Clone, Component)]
pub struct NarrativeActions {
    pub send_texts: Vec<NarrativeTextMessage>,
    pub change_sanity: Option<i32>, // Some(0) produces a literal '0' indicator
    pub spawn_item: Vec<SpawnablePickup>,
}

impl NarrativeActions {
    pub fn new_with_texts(send_texts: Vec<NarrativeTextMessage>) -> NarrativeActions {
        NarrativeActions {
            send_texts,
            ..Default::default()
        }
    }

    pub fn new_with_sanity(change_sanity: Option<i32>) -> NarrativeActions {
        NarrativeActions {
            change_sanity,
            ..Default::default()
        }
    }

    pub fn new_with_pickup(spawn_item: Vec<SpawnablePickup>) -> NarrativeActions {
        NarrativeActions {
            spawn_item,
            ..Default::default()
        }
    }
}

#[derive(Clone)]
pub struct NarrativeTextMessage {
    pub sender: String,
    pub body: String,
}

#[derive(Clone)]
pub struct SpawnablePickup {
    pub prototype: pickup::Pickup,
    pub location: (usize, usize),
    pub narrative_actions: NarrativeActions,
}

pub fn make_main_narrative() -> Vec<NarrativeEvent> {
    return vec![
        NarrativeEvent{
            starts_act: true,
            criterion: NarrativeCriterion::ElapsedRel(1.5),
            action: action().send_text(
                "Dictator DAN",
                "Fellow Victorians!|We must do what must be done.|STAY INDOORS. BY ORDER OF THE GOVERNMENT, THAT IS, ME",
            ),
        },
        NarrativeEvent{
            starts_act: false,
            criterion: NarrativeCriterion::ElapsedRel(1.5),
            action: action().change_sanity(3),
        },
        NarrativeEvent{
            starts_act: false,
            criterion: NarrativeCriterion::ElapsedRel(3.5),
            action: action().send_text(
                "Mum",
                "Hello dearie|Just sent you a little something in the mail. Hope you're well. xoxox|Mum",
            ).change_sanity(9),
        },
        NarrativeEvent{
            starts_act: false,
            criterion: NarrativeCriterion::ElapsedRel(2.5),
            action: action().spawn_pickup(
                pickup::Pickup::Potplant,
                (5, 5),
                Default::default(),
            ),
        },
        NarrativeEvent{
            starts_act: false,
            criterion: NarrativeCriterion::ClearedAll,
            action: action().send_text(
                "The Game",
                "You picked up the thing|Now go to the park",
            ),
        },
        NarrativeEvent{
            starts_act: false,
            criterion: NarrativeCriterion::InEnvironment(Location::Park),
            action: action().send_text(
                "The Game",
                "You have gone to the park. You are good at directions.",
            ),
        },
    ];
}

pub fn make_covid_narrative() -> Vec<NarrativeEvent> {
    vec![
        NarrativeEvent{
            starts_act: true,
            criterion: NarrativeCriterion::ElapsedRel(1.0),
            action: action().send_text(
                "Department of Health",
                "You have been exposed to Covid as a close contact with another person. You must isolate for seven days.|During this time, you must not leave your house.",
            ),
        },
        NarrativeEvent{
            starts_act: true,
            criterion: NarrativeCriterion::ElapsedRel(7.*5.),
            action: action().send_text(
                "Department of Health",
                "Your Covid quarantine has finished. You can now leave your house. Stay safe out there.",
            ),
        },
    ]
}

fn action() -> NarrativeActions {
    NarrativeActions {
        ..Default::default()
    }
}

impl NarrativeActions {
    fn send_text(mut self, sender: &str, body: &str) -> Self {
        self.send_texts.push(NarrativeTextMessage {
            sender: String::from(sender),
            body: String::from(body),
        });
        self
    }

    fn change_sanity(mut self, by: i32) -> Self {
        self.change_sanity = Some(by);
        self
    }

    fn spawn_pickup(
        mut self,
        what: pickup::Pickup,
        at: (usize, usize),
        narrative_actions: NarrativeActions,
    ) -> Self {
        self.spawn_item.push(SpawnablePickup {
            prototype: what,
            location: at,
            narrative_actions,
        });
        self
    }
}
