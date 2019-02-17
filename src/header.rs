use super::*;
mod header_icons;
use header_icons::*;
/// The top-level component we pass to the virtual dom.
pub fn header(model: &Model) -> El<Msg> {
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
    div![   attrs!{At::Class => "main-nav"},

                a![
                    simple_ev("click", Msg::ChangePageAndHistory(Page::Eventos)),
                    attrs!{At::Class => nav_class_parties},
                    El::from_html(&header_icons::parties_icon("main-nav-items",  "lightgrey"))
                ],
                a![
                    simple_ev("click", Msg::ChangePageAndHistory(Page::EventosSalvos)),
                    attrs!{At::Class => nav_class_tickets},
                    El::from_html(&header_icons::tickets_icon("main-nav-items",  "lightgrey"))
                ],
                a![
                    simple_ev("click", Msg::ChangePageAndHistory(Page::AdicioneEvento)),
                    attrs!{At::Class => nav_class_new_event},
                    El::from_html(&header_icons::new_event_icon("main-nav-items",  "lightgrey"))
                ],

         ]
}