use super::*;


#[derive(Clone)]
pub struct Model {
    pub current_page: Page,
    pub events: Vec<Event>,
    pub event_begin_edited: IncompleteEvent,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IncompleteEvent {
    pub date: String,
    pub description: String,
    pub image_url: String,
    pub place: String,
    pub price: f32,
    pub sales_place: String,
    pub title: String,
}

impl Default for IncompleteEvent{
    fn default() -> Self {
        IncompleteEvent{
            date: "".to_string(),
            description: "".to_string(),
            image_url: "pkg/placeholder.jpg".to_string(),
            place: "".to_string(),
            price: 0.0,
            sales_place: "".to_string(),
            title: "".to_string()
        }
    }
}



#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Event {
    pub date: String,
    pub description: String,
    pub id: u64,
    pub image_alt: String,
    pub image_url: String,
    pub place: String,
    pub price: f32,
    pub sales_place: String,
    pub title: String,
    #[serde(default)]
    pub ui_state: EventUIState,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EventUIState{
    pub expanded: bool,
    pub saved: bool,
}

impl Default for EventUIState {
    fn default() -> Self {
        Self {
            expanded: false,
            saved: false
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            current_page: Page::Eventos,
            events: vec![Event{
                date: "Date".to_string(),
                description: "O CARNAVRAAU está chegando daquele jeitinho e sabe quem vai entrar na onda dos bloquinhos? 💥

A Cervejada mais raiz desse Mackenzie!

Ela vem em clima de carnaval, prontíssima pra primeira do ano!

E se você pensa que cachaça não é água não, vai preparando o seu fígado pra um Open Bar de respeito! Vem aí:

Mackejada - Carnaval 🎉🍻

Vamos curtir em ritmo carnavalesco e claro em ritmo de BAILÃO! 😈
".to_string(),
                id: 1,
                image_alt: "alt ".to_string(),
                image_url: "https://s3-us-west-2.amazonaws.com/pixel-solutions/event/banner/99e98ab25482f8eaae5742a9d94e91ed.jpg".to_string(),
                place: "Place".to_string(),
                price: 10.0,
                sales_place: "Sales Place".to_string(),
                title: "The Title".to_string(),
                ui_state: EventUIState::default()
            },Event{
                date: "Date 2".to_string(),
                description: "O CARNAVRAAU está chegando daquele jeitinho e sabe quem vai entrar na onda dos bloquinhos? 💥

A Cervejada mais raiz desse Mackenzie!

Ela vem em clima de carnaval, prontíssima pra primeira do ano!

E se você pensa que cachaça não é água não, vai preparando o seu fígado pra um Open Bar de respeito! Vem aí:

Mackejada - Carnaval 🎉🍻

Vamos curtir em ritmo carnavalesco e claro em ritmo de BAILÃO! 😈
".to_string(),
                id: 2,
                image_alt: "alt ".to_string(),
                image_url: "https://s3-us-west-2.amazonaws.com/pixel-solutions/event/banner/99e98ab25482f8eaae5742a9d94e91ed.jpg".to_string(),
                place: "Place 2".to_string(),
                price: 20.0,
                sales_place: "Sales Place".to_string(),
                title: "The Title".to_string(),
                ui_state: EventUIState::default()
            }],
            event_begin_edited: IncompleteEvent::default()
        }
    }
}