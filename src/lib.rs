use::seed::{prelude::*, *};
use rand::prelude::*;

struct Model {
    items: Vec<Item>,
    name: String,
    price_string: String,
    message: String,
}

#[derive(Clone)]
struct Item {
    id: String,
    name: String,
    price: i32
}

enum Msg {
    SetName(String),
    SetPrice(String),
    Post,
    Delete(String)
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        items: Vec::new(),
        name: "".to_string(),
        price_string: "0".to_string(),
        message: "".to_string(),
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetName(name) => model.name = name,
        Msg::SetPrice(price_string) => model.price_string = price_string,
        Msg::Post => {
            if model.name == "" {
                model.message = "商品/サービス名が入力されていません".to_string();
            } else {
                let price = model.price_string.parse();
                match price {
                    /* Okの時は値をそのまま得られる Errのときはエラーオブジェクトを得る */
                    Ok(price) => {
                        if price >= 0 {
                            model.items.push(Item {
                                id: model.items.len().to_string(),
                                name: model.name.clone(),
                                price: price
                            });
                            model.name = "".to_string();
                            model.price_string = "0".to_string();
                            model.message = "".to_string();
                        } else {
                            model.message = "商品/サービス価格は0以上の数値を入力してください".to_string()
                        }
                    } 
                    Err(_) => model.message = "商品/サービス価格は半角数字で入力してください".to_string()
                }
            }
        },
        Msg::Delete(id) => {
            /* clone()との違いが分からない filterはイテレータを返すのでイテレータ専用のクローンメソッド? */
            model.items = model.items.iter().filter(| item | item.id != id).cloned().collect::<Vec<Item>>();
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        nav![
            attrs!{ At::Class => "navbar navbar-dark bg-dark" },
            style!{ St::MarginBottom => "3%" },
            div![
                attrs!{ At::Class => "container-fluid" },
                a!{
                    attrs!{ At::Class => "navbar-brand", At::Href => "/" },
                    "Buy Log"
                }
            ]
        ],
        div![
            style!{ St::Width => "90%", St::MarginTop => "auto", St::MarginLeft => "auto", St::MarginRight => "auto", St::MarginBottom => "3%" },
            h4![
                "購入履歴を追加"
            ],
            div![
                attrs!{ At::Class => "mb-3" },
                style!{ St::MarginBottom => "2%" },
                label![
                    "商品/サービス名",
                ],
                input![
                    attrs!{ At::Class => "form-control", At::Value => model.name },
                    input_ev(Ev::Input, Msg::SetName)
                ],
            ],
            div![
                style!{ St::MarginBottom => "2%" },
                label![
                    "商品/サービス価格"
                ],
                input![
                    attrs!{ At::Class => "form-control", At::Value => model.price_string },
                    input_ev(Ev::Input, Msg::SetPrice)
                ],
            ],
            button![
                "送信",
                ev(Ev::Click, |_| Msg::Post),
                attrs!{ At::Class => "btn btn-primary" }
            ],
            p![
                model.message.clone()
            ]
        ],
        div! [
            style!{ St::Width => "90%", St::Margin => "auto" },
            h4![
                "購入履歴",
                style!{ St::MarginBottom => "3%" }
            ],
            /*
            if model.items.len() == 0 {
                p![
                    "購入履歴はありません"
                ]
            },
            */
            model.items.clone().into_iter().map(| item | {
                div![
                    attrs!{ At::Class => "card" },
                    div![
                        attrs!{ At::Class => "card-body" },
                        h5![
                            /* ここをcloneしないと消去ボタンでコンパイルエラー */
                            item.name.clone(),
                            attrs!{ At::Class => "card-title" }
                        ],
                        p![
                            item.price,
                            attrs!{ At::Class => "card-text" }
                        ],
                        button![
                            "削除",
                            ev(Ev::Click, |_| Msg::Delete(item.id)),
                            attrs!{ At::Class => "btn btn-danger" }
                        ]
                    ],
                ]
            })
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}