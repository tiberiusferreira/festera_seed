use super::*;


pub fn event_editor(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    let title = &model.event_begin_edited.title;
    let date = &model.event_begin_edited.date;
    let place = &model.event_begin_edited.place;
    let sales_place = &model.event_begin_edited.sales_place;
    let price = &model.event_begin_edited.price;
    let src = &model.event_begin_edited.image_url;
    div![ attrs!{At::Class => "event-posts-container"},
                div![attrs!{At::Class => "event-container"},
                        div![attrs!{At::Class => "event-summary-container"},
                                input![input_ev(Ev::Input, Msg::EventEditingTitle), attrs!{At::Class => "event-summary-title"; At::PlaceHolder => "Título"; At::Value => title; At::Type => "text";}],
                                input![input_ev(Ev::Input, Msg::EventEditingDate), attrs!{At::Class => "event-summary-date"; At::PlaceHolder => "Data"; At::Value => date; At::Type => "text";}],
                                input![input_ev(Ev::Input, Msg::EventEditingPlace), attrs!{At::Class => "event-summary-place"; At::PlaceHolder => "Lugar do Evento"; At::Value => place; At::Type => "text";}],
                                input![input_ev(Ev::Input, Msg::EventEditingSalesPlace), attrs!{At::Class => "event-summary-sales-place"; At::PlaceHolder => "Lugar de Vendas"; At::Value => sales_place; At::Type => "text";}],
                                input![raw_ev(Ev::KeyPress, move |ev| Msg::EventEditingPriceKeyDown(
                    ev
                )), attrs!{At::Class => "event-summary-price"; At::PlaceHolder => "Preço"; At::Value => price; At::Type => "number";}]
                        ],
                        label![
                                attrs!{At::Class => "event-img"; At::For => "event_img"},
                            img![style!["max-width" => "100%"; "display" => "block"],
                                attrs!{At::Src => src; At::Alt => "Event Image Preview"}
                            ]
                        ],
                        div![ attrs!{At::Class => "event-details"}, style!{"display" => "block"},
                                "Description",
                            ],
                            input![
                               raw_ev(Ev::Input,  move |event| Msg::EventEditingPicture(state.clone(), event)),
                                style!["display" => "none"],
                                attrs!{
                                    At::Id => "event_img"; At::Name => "event_img"; At::Accept => "image/png, image/jpeg"; At::Type => "file" }
                            ]
                        ]
    ]


}


pub fn botao(){

}