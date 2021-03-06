use seed::*;
use seed::prelude::*;
use strum_macros::*;
use std::str::FromStr;
mod header;
use serde::{Serialize, Deserialize};
use serde_json::from_str;
mod events_view;
use events_view::events_list;
mod model;
use model::*;
mod event_editor;
use seed::{Request, Method, spawn_local};
use futures::Future;
use wasm_bindgen::JsCast;


#[derive(Clone, PartialEq, Display, EnumString)]
pub enum Page {
    Eventos,
    EventosSalvos,
    AdicioneEvento,
}



#[derive(Clone)]
pub enum Msg {
    ChangePageAndHistory(Page),
    ChangePage(Page),
    ToggleEvent(u64),
    ToggleSaveEvent(u64),
    NewEvents(Vec<Event>),
    EventEditingTitle(String),
    EventEditingDate(String),
    EventEditingPlace(String),
    EventEditingSalesPlace(String),
    EventEditingPrice(String),
    EventEditingPriceKeyDown(web_sys::Event),
    EventEditingPicture(seed::App<Msg, Model>, web_sys::Event),
    EventEditingPictureNewSrc(String),
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
        Msg::EventEditingTitle(title) => {
            model.event_begin_edited.title = title;
            Render(model)
        },
        Msg::EventEditingDate(date) => {
            model.event_begin_edited.date = date;
            Render(model)
        },
        Msg::EventEditingPlace(place) => {
            model.event_begin_edited.place = place;
            Render(model)
        },
        Msg::EventEditingSalesPlace(sales_place) => {
            model.event_begin_edited.sales_place = sales_place;
            Render(model)
        },
        Msg::EventEditingPrice(price) => {
//            if price.parse::<f32>().is_err(){
//                log("Error parsing");
//            }
//            model.event_begin_edited.price = price.parse().unwrap_or(0.0);
            Render(model)
        },
        Msg::EventEditingPriceKeyDown(event) => {
            let target = event.target().unwrap();
            event.prevent_default();
            let keycode = seed::to_kbevent(&event).key_code();
            let letter = std::char::from_u32(keycode).expect("Invalid unicode");
            let input_el = seed::to_input(&target);

            let mut current: f32 = match input_el.value().parse(){
                Ok(value) => value,
                Err(_) => {
                    log("Invalid current value");
                    model.event_begin_edited.price = 0.0;
                    input_el.set_value(&model.event_begin_edited.price.to_string());
                    return Render(model);
                }
            };
            let mut current = current.to_string();
            log!("current ", current);
            current.push(letter);
            log!("current after push ", current);
            let maybe_new_value = current.parse::<f32>();
            match maybe_new_value {
                Ok(new) => {
                    log!("new ", new);
                    model.event_begin_edited.price = new;
                    input_el.set_value(&model.event_begin_edited.price.to_string());
                },
                Err(_) => {
                },
            }


//            if keycode < 48 || keycode > 57 {
//                log("Blocked");
//                event.prevent_default();
//            }


//            let target = event.target().unwrap();
//            let input_el = seed::to_input(&target);
//            = input_el.value().parse().unwrap_or(0.0);

//            log(text);
            Render(model)
        },
        Msg::EventEditingPicture(state , event) =>{
            let target = event.target().unwrap();
            let target_event = seed::to_input(&target);

            let target = event.target().unwrap();
            let text = seed::to_input(&target).value();

            let img = target_event.files().unwrap().get(0).unwrap();
            let file_reader = web_sys::FileReader::new().expect("error creating file reader");
            file_reader.read_as_data_url(&img);
            let cloned = file_reader.clone();

            let callback = move || {
//                state.update(Msg::NewEvents(events_json));
                log(format!("{:?}", cloned.result().unwrap()));
                let src = cloned.result().unwrap();
                state.update(Msg::EventEditingPictureNewSrc(src.as_string().expect("Jsvalue to string failed")));
                log("Done uploading");
            };

            let cb = Closure::wrap(Box::new(callback
            ) as Box<dyn Fn()>);

//            let callback = Closure::wrap(handler as Box<dyn Fn()>);
//            seed::set_interval()
            file_reader.set_onloadend(Some(
                cb.as_ref().unchecked_ref()
            ));


            cb.forget();
            log(format!("{:?}", img.name()));
            Skip(model)
        },
        Msg::EventEditingPictureNewSrc(src) => {
            model.event_begin_edited.image_url = src;
            Render(model)
        },
        Msg::Nothing => {
            log("Nothing");
            Skip(model)
        }
    }
}


/// The top-level component we pass to the virtual dom.
fn view(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    let page_body = match model.current_page{
        Page::Eventos => {events_list(model.events
            .iter()
            .collect::<Vec<&Event>>()
            .as_slice())
        },
        Page::EventosSalvos => {
            let saved_events = model.events
                .iter()
                .filter(|event| event.ui_state.saved)
                .collect::<Vec<&Event>>();
            events_list(saved_events.as_slice())
        },
        Page::AdicioneEvento => {empty()},
    };
    div![
            header::header(model),
            event_editor::event_editor(state, model),
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