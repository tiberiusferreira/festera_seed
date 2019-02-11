use seed::*;
use seed::prelude::*;
use strum_macros::*;
//mod header;
//use header::*;
//mod body_content;
//use body_content::*;
//mod about_content;
//mod fotos_content;
use std::str::FromStr;
mod header_icons;
use header_icons::*;

#[derive(Clone)]
pub struct Model {
    current_page: Page
}

#[derive(Clone, PartialEq, Display, EnumString)]
pub enum Page {
    Fotos,
    Vagas,
    Sobre,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            current_page: Page::Sobre
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

        // This is separate, because in-app naviation needs to call push_route,
        // but we don't want to call it from browser navigation. (eg back button)
        Msg::ChangePage(current_page) => {
            Render (Model {current_page, ..model})
        },
        Msg::Nothing => {
            Skip(model)
        }
    }
}


// View


/// The top-level component we pass to the virtual dom.
fn view(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    div![attrs!{At::Class => "main-nav-organizer"},
        a![
            attrs!{At::Class => "navlink navlink-selected"},
            El::from_html(&header_icons::parties_icon("main-nav-items",  "deepskyblue"))
        ],
        a![
            attrs!{At::Class => "navlink"},
            El::from_html(&header_icons::new_event_icon("main-nav-items",  "lightgrey"))
        ],
        a![
            attrs!{At::Class => "navlink"},
            El::from_html(&header_icons::tickets_icon("main-nav-items",  "lightgrey"))
        ],
        a![
            attrs!{At::Class => "navlink"},
            El::from_html(&header_icons::setting_icon("main-nav-items",  "lightgrey"))
        ]
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
        return Msg::ChangePage(Page::Sobre)
    }

    match Page::from_str(&url.path[0]){
        Ok(page) => {
            return Msg::ChangePage(page)
        },
        Err(_) => {
            return Msg::ChangePage(Page::Sobre)
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