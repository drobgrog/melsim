use bevy::prelude::Component;

use crate::pickup;
use crate::environment::Location;
use std::fs::File;
use std::collections::HashMap;
use csv::StringRecord;
use std::str::FromStr;

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

pub fn load_csv(file: &str) -> Vec<NarrativeEvent> {
    let file = File::open(file).expect("error opening narrative csv");
    let mut rdr = csv::Reader::from_reader(file);

    // First, make the header
    let headers = rdr.headers().expect("error reading row").clone();
    let h = csv_header(&headers);

    let mut rv = Vec::new();
    for x in rdr.records() {
        let x = x.unwrap();
        let non_time_condition = if non_empty(get(&h, &x, "Cleared All Pickups?")) {
            Some(NarrativeCriterion::ClearedAll)
        } else if non_empty(get(&h, &x, "Location change?")) {
            Some(NarrativeCriterion::InEnvironment(str2location(get(&h, &x, "Location change?"))))
        } else {
            None
        };

        let time = get(&h, &x, "Elapsed Time");
        if non_empty(time) && non_time_condition.is_some() {
            // Special case - if there is a time and other criteria, generate dummy criterion
            rv.push(NarrativeEvent{
                starts_act: false, // TODO
                criterion: non_time_condition.unwrap(),
                action: action(),
            });
        }

        // Now create the real thing iff 'time' is non-empty
        // The logic of this is that we end up skipping stuff with no criteria at all
        if non_empty(time) {
            let criterion = NarrativeCriterion::ElapsedRel(f64::from_str(time).expect("bad parse time"));

            let mut a = action();

            // Process the action
            if non_empty(get(&h, &x, "Sender")) {
                let polished = get(&h, &x, "Body (Polished)");
                let rough = get(&h, &x, "Body (Rough)");
                a.send_texts.push(NarrativeTextMessage{
                    sender: String::from(get(&h, &x, "Sender")),
                    body: String::from(if non_empty(polished) { polished } else { rough }),
                });
            }

            let sanity = get(&h, &x, "Change Sanity?");
            if non_empty(sanity) {
                a.change_sanity = Some(i32::from_str_radix(sanity, 10).expect("bad parse sanity"));
            }

            let spawn_item = get(&h, &x, "Spawn Item?");
            if non_empty(spawn_item) {
                a.spawn_item.push(str2spawnitem(spawn_item));
            }

            rv.push(NarrativeEvent{
                starts_act: true, // TODO
                criterion: criterion,
                action: a,
            });
        }
    }

    return rv;
}

fn str2location(s: &str) -> Location {
    match s {
        "Park" => Location::Park,
        "Home" => Location::Home,
        _      => panic!("bad location >>{}<<", s),
    }
}

fn str2spawnitem(s: &str) -> SpawnablePickup {
    match s {
        "Care Package" | "TV" | "Fridge" | "Pillow" | "Soap" | "Towel" | "Video Game" => SpawnablePickup{
            // TODO FIX Kane
            prototype: pickup::Pickup::Potplant,
            location: (5, 5),
            narrative_actions: action().change_sanity(10),
        },
        _ => panic!("bad spawn: {}", s),
    }
}

// TODO: whitespace?
fn non_empty(s: &str) -> bool {
    s.len() > 0
}

fn get<'a>(h: &'a HashMap<&str, usize>, r: &'a StringRecord, v: &'a str) -> &'a str {
    let idx = h.get(v).unwrap();
    return &r[*idx];
}

fn csv_header(rec: &StringRecord) -> HashMap<&str, usize> {
    let mut rv = HashMap::new();

    for (i, field) in rec.iter().enumerate() {
        rv.insert(field, i);
    }

    return rv;
}

#[allow(dead_code)]
pub fn hardcoded_main_narrative() -> Vec<NarrativeEvent> {
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

#[allow(dead_code)]
pub fn hardcoded_covid_narrative() -> Vec<NarrativeEvent> {
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
