use super::*;


pub fn event_editor(model: &Model) -> El<Msg> {
    let title = &model.event_begin_edited.title;
    let date = &model.event_begin_edited.date;
    let place = &model.event_begin_edited.place;
    let sales_place = &model.event_begin_edited.sales_place;
    let price = &model.event_begin_edited.price;
    form![attrs!{At::Class => "event-container"},
                        div![attrs!{At::Class => "event-summary-container"},
                                input![input_ev(Ev::Input, Msg::EventEditingTitle), attrs!{At::Class => "event-summary-title"; At::PlaceHolder => "Título"; At::Value => title; At::Type => "text";}],
                                input![input_ev(Ev::Input, Msg::EventEditingDate), attrs!{At::Class => "event-summary-date"; At::PlaceHolder => "Data"; At::Value => date; At::Type => "text";}],
                                input![input_ev(Ev::Input, Msg::EventEditingPlace), attrs!{At::Class => "event-summary-place"; At::PlaceHolder => "Lugar do Evento"; At::Value => place; At::Type => "text";}],
                                input![input_ev(Ev::Input, Msg::EventEditingSalesPlace), attrs!{At::Class => "event-summary-sales-place"; At::PlaceHolder => "Lugar de Vendas"; At::Value => sales_place; At::Type => "text";}],
                                input![input_ev(Ev::Input, Msg::EventEditingPrice), attrs!{At::Class => "event-summary-price"; At::PlaceHolder => "Preço"; At::Value => price; At::Type => "number";}]
                        ],
                        img![
                            attrs!{At::Class => "event-img"; At::Src => "https://s3-us-west-2.amazonaws.com/pixel-solutions/event/banner/99e98ab25482f8eaae5742a9d94e91ed.jpg"; At::Alt => "alt"}
                        ],
                        div![ attrs!{At::Class => "event-details"}, style!{"display" => "block"},
                                "Description",
                                    ]

                        ]
}