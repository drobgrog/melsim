use crate::pickup;

pub struct NarrativeEvent {
    pub starts_act: bool,
    pub criterion: NarrativeCriterion,
    pub action: NarrativeActions,
}

pub enum NarrativeCriterion {
    ElapsedRel(f64),    // at least this many seconds have elasped since last event
    ClearedAll,         // all items in the environment must be cleared
}

#[derive(Default,Clone)]
pub struct NarrativeActions {
    pub send_texts: Vec<NarrativeTextMessage>,
    pub change_sanity: Option<i32>,  // Some(0) produces a literal '0' indicator
    //spawn_item: Vec<SpawnablePickup>,
}

#[derive(Clone)]
pub struct NarrativeTextMessage {
    pub sender: String,
    pub body: String,
}

pub struct SpawnablePickup {
    prototype: pickup::Pickup,
    location: (usize, usize),
}

pub fn make_main_narrative() -> Vec<NarrativeEvent> {
    return vec![
        NarrativeEvent{
            starts_act: true,
            criterion: NarrativeCriterion::ElapsedRel(1.5),
            action: send_text(
                "Dictator DAN",
                "Fellow Victorians!|We must do what must be done.|STAY INDOORS. BY ORDER OF THE GOVERNMENT, THAT IS, ME",
            ),
        },
        NarrativeEvent{
            starts_act: false,
            criterion: NarrativeCriterion::ElapsedRel(1.5),
            action: change_sanity(3),
        },
        NarrativeEvent{
            starts_act: false,
            criterion: NarrativeCriterion::ElapsedRel(3.5),
            action: send_text(
                "Mum",
                "Hello dearie|Just sent you a little something in the mail. Hope you're well. xoxox|Mum",
            ),
        },
    ];
}

pub fn make_covid_narrative() -> Vec<NarrativeEvent> {
    vec![]
}

// helper constructors
fn send_text(sender: &str, body: &str) -> NarrativeActions {
    NarrativeActions{
        send_texts: vec![
            NarrativeTextMessage{
                sender: String::from(sender),
                body: String::from(body),
            }
        ],
        change_sanity: None,
        //spawn_item: vec![],
    }
}

fn change_sanity(by: i32) -> NarrativeActions {
    NarrativeActions{
        send_texts: Vec::new(),
        change_sanity: Some(by),
        //spawn_item: vec![],
    }

}
