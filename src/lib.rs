use seed::*;
use seed::prelude::*;
use strum_macros::*;
use std::str::FromStr;
mod header;
use serde::{Serialize, Deserialize};
use serde_json::from_str;
mod events_view;
use events_view::events_list;
use seed::{Request, Method, spawn_local};
use futures::Future;

#[derive(Clone)]
pub struct Model {
    current_page: Page,
    events: Vec<Event>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Event {
    date: String,
    description: String,
    id: u64,
    image_alt: String,
    image_url: String,
    place: String,
    price: f32,
    sales_place: String,
    title: String,
    #[serde(default)]
    ui_state: EventUIState
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct EventUIState{
    expanded: bool,
    saved: bool,
}

impl Default for EventUIState {
    fn default() -> Self {
        Self {
            expanded: false,
            saved: false
        }
    }
}

#[derive(Clone, PartialEq, Display, EnumString)]
pub enum Page {
    Eventos,
    EventosSalvos,
    AdicioneEvento,
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
            }]
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    ChangePageAndHistory(Page),
    ChangePage(Page),
    ToggleEvent(u64),
    ToggleSaveEvent(u64),
    NewEvents(Vec<Event>),
    Nothing,
}

impl Model{
    fn toggle_event(&mut self, event_id: u64){
        for event in &mut self.events{
            if event.id == event_id{
                event.ui_state.expanded = !event.ui_state.expanded;
                return
            }
        }
        log(format!("Tried to toggle non existing event: {}", event_id));
    }

    fn toggle_save_event(&mut self, event_id: u64){
        log(format!("Toggled event: {}", event_id));
        for event in &mut self.events{
            if event.id == event_id{
                event.ui_state.saved = !event.ui_state.saved;
                return
            }
        }
        log(format!("Tried to toggle non existing event: {}", event_id));
    }
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, mut model: Model) -> Update<Model> {
    match msg {
        // The change page pushes the history and THEN changes the page itself
        Msg::ChangePageAndHistory(page) => {
            // This just pushes the history
            seed::push_path(vec![&page.to_string()]);
            // Now we call the routing
            update(Msg::ChangePage(page), model)
        },
        // This is separate, because in-app navigation needs to call push_route,
        // but we don't want to call it from browser navigation. (eg back button)
        Msg::ChangePage(current_page) => {
            Render (Model {current_page, ..model})
        },
        Msg::ToggleEvent(event_id) => {
            model.toggle_event(event_id);
            Render(model)
        },
        Msg::ToggleSaveEvent(event_id) => {
            model.toggle_save_event(event_id);
            Render(model)
        },
        Msg::NewEvents(events) => {
            model.events = events;
            Render(model)
        },
        Msg::Nothing => {
            Skip(model)
        }
    }
}


/// The top-level component we pass to the virtual dom.
fn view(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    let page_body = match model.current_page{
        Page::Eventos => {events_list(model)},
        Page::EventosSalvos => {empty()},
        Page::AdicioneEvento => {empty()},
    };
    div![
            header::header(model),
            page_body,
            div![attrs!{At::Class => "footer"},]
        ]
}

fn routes(url: &seed::Url) -> Msg {
    if url.path.is_empty() || url.path[0].is_empty(){
        return Msg::Nothing
    }

    match Page::from_str(&url.path[0]){
        Ok(page) => {
            return Msg::ChangePage(page)
        },
        Err(e) => {
            log(format!("{:#?} is not a Page", &url.path[0]));
            log(format!("Error: {:#?}", e));
            return Msg::Nothing
        }
    };
}

fn get_data(state: seed::App<Msg, Model>) -> impl Future<Item = (), Error = JsValue> {
    let url = "http://localhost:1024/api/events";
    Request::new(url)
        .method(Method::Get)
        .fetch_json()
        .map(move |events_json| {
            state.update(Msg::NewEvents(events_json));
        })
}

#[wasm_bindgen]
pub fn render() {
    let state = seed::App::build(Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
    spawn_local(get_data(state));
}