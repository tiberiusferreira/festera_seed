use seed::*;
use seed::prelude::*;
use strum_macros::*;
use std::str::FromStr;
mod header_icons;
use header_icons::*;

#[derive(Clone)]
pub struct Model {
    current_page: Page
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
            current_page: Page::Eventos
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    ChangePageAndHistory(Page),
    ChangePage(Page),
    Nothing,
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: Model) -> Update<Model> {
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
        Msg::Nothing => {
            Skip(model)
        }
    }
}


/// The top-level component we pass to the virtual dom.
fn view(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    let mut nav_class_parties = "navlink".to_string();
    let mut nav_class_tickets = nav_class_parties.clone();
    let mut nav_class_new_event = nav_class_parties.clone();
    match model.current_page {
        Page::Eventos => {
            nav_class_parties.push_str(" navlink-selected");
        },
        Page::EventosSalvos => {
            nav_class_tickets.push_str(" navlink-selected");
        },
        Page::AdicioneEvento => {
            nav_class_new_event.push_str(" navlink-selected");
        },
    }
        div![
            div![
                attrs!{At::Class => "main-nav"},
                a![
                    attrs!{At::Class => nav_class_parties},
                    El::from_html(&header_icons::parties_icon("main-nav-items",  "lightgrey"))
                ],
                a![
                    attrs!{At::Class => nav_class_tickets},
                    El::from_html(&header_icons::tickets_icon("main-nav-items",  "lightgrey"))
                ],
                a![
                    attrs!{At::Class => nav_class_new_event},
                    El::from_html(&header_icons::new_event_icon("main-nav-items",  "lightgrey"))
                ],

            ],

        ]


    /*
    <div className={isOrganizer ? ("main-nav-organizer") : ("main-nav")}>
                        <NavLink exact to={'/'} className={"navlink"} activeClassName="navlink-selected">
                            <HomeButton className={"main-nav-items"} color="lightgrey"/>
                        </NavLink>
                        <NavLink to={'/BoughtTickets'} className={"navlink"} activeClassName="navlink-selected">
                            <TicketButton className={"main-nav-items"} color="lightgrey"/>
                        </NavLink>
                        {isOrganizer ? (
                        <NavLink exact to={'/ManageEvents'} className={"navlink"} activeClassName="navlink-selected">
                            <EditButton className={"main-nav-items"} color="lightgrey"/>
                        </NavLink>) : (null)
                        }
                        <NavLink exact to={'/Settings'} className={"navlink"} activeClassName="navlink-selected">
                            <SettingsButton className={"main-nav-items"} color="lightgrey"/>
                        </NavLink>
                    </div>
    */
}

fn routes(url: &seed::Url) -> Msg {
    log(format!("{:#?}", url));
    if url.path.is_empty() {
        return Msg::ChangePage(Page::AdicioneEvento)
    }

    match Page::from_str(&url.path[0]){
        Ok(page) => {
            return Msg::ChangePage(page)
        },
        Err(_) => {
            return Msg::ChangePage(Page::AdicioneEvento)
        }
    };
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
//        .routes(routes)
        .finish()
        .run();
}