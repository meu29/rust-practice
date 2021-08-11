use::seed::{prelude::*, *};

struct Model {
    items: Vec<Item>,
    name: String,
    price: i32,
}

#[derive(Clone)]
struct Item {
    id: String,
    name: String,
    price: i32,
}

enum Msg {
    SetName(String),
    SetPrice(String),
    Post
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        items: Vec::new(),
        name: "".to_string(),
        price: 0,
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetName(name) => model.name = name,
        Msg::SetPrice(price_string) => model.price = price_string.parse().unwrap(),
        Msg::Post => {
            model.items.push(Item {
                id: "hoge".to_string(),
                name: model.name.clone(),
                price: model.price.clone(),
            })
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            h4![
                "購入履歴を追加"
            ],
            div![
                attrs!{ At::Class => "mb-3" },
                label![
                    "商品/サービス名"
                ],
                input![
                    attrs!{ At::Class => "form-control", At::Value => model.name },
                    input_ev(Ev::Input, Msg::SetName)
                ],
            ],
            div![
                label![
                    "商品/サービス価格"
                ],
                input![
                    attrs!{ At::Class => "form-control", At::Value => model.price },
                    input_ev(Ev::Input, Msg::SetPrice)
                ],
            ],
            button![
                "送信",
                ev(Ev::Click, |_| Msg::Post),
                attrs!{ At::Class => "btn btn-primary" }
            ]
        ],
        div! [
            h4![
                "購入履歴"
            ],
            model.items.clone().into_iter().map(| item | {
                div![
                    item.name
                ]
            })
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}