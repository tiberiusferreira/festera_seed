use super::*;


pub fn events_list(model: &Model) -> El<Msg> {
    let mut events_el: Vec<El<Msg>> = Vec::new();
    for event in &model.events{
        let display_style = if event.ui_state.expanded{
            "block"
        }else{
            "none"
        };
        let button_color = if event.ui_state.saved{
            "lawngreen" // green
        }else{
            "deepskyblue"
        };
        let button_text = if event.ui_state.saved{
            "Salvo"
        }else{
            "Salvar"
        };
        events_el.push(
            div![attrs!{At::Class => "event-container"},
                        div![attrs!{At::Class => "event-summary-container"},
                        simple_ev("click", Msg::ToggleEvent(event.id)),
                                h2![event.title, attrs!{At::Class => "event-summary-title"}],
                                h3![event.date, attrs!{At::Class => "event-summary-date"}],
                                h4![event.place, attrs!{At::Class => "event-summary-place"}],
                                h4![event.sales_place, attrs!{At::Class => "event-summary-sales-place"}],
                                h2![format!("R${}", event.price), attrs!{At::Class => "event-summary-price"}]
                        ],
                        img![simple_ev("click", Msg::ToggleEvent(event.id)),
                            attrs!{At::Class => "event-img"; At::Src => event.image_url; At::Alt => event.image_alt}
                        ],
                        div![ attrs!{At::Class => "event-details"}, style!{"display" => display_style},
                                event.description,
                                    div![simple_ev("click", Msg::ToggleSaveEvent(event.id)),
                                    attrs!{At::Class => "event-posts-container-save-button"},
                                    style!{"background-color" => button_color;},
                                        h2![attrs!{At::Class => "event-posts-container-save-button-text"}, button_text]
                                    ]

                        ]
                ]
        );
    }
    div![ attrs!{At::Class => "event-posts-container"},
                 events_el
                ]
}