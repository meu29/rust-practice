use::seed::{prelude::*, *};
use rand::Rng;

/* ディレクトリではなくファイルの方を見ている */
mod layouts;

struct Model {
    items: Vec<Item>,
    name: String,
    price_string: String,
    tag: String,
    message: String,
    selected_tag: String
}

#[derive(Clone)]
struct Item {
    id: String,
    name: String,
    tag: String,
    price: i32
}

enum Msg {
    SetName(String),
    SetPrice(String),
    SetTag(String),
    Post,
    ChangeSelectedTag(String),
    Delete(String)
}

fn make_id() -> String {

    let chars: &str = "abcdefghijklmnopqrstuvwxyz1234567890";
    let mut id: String = "".to_string();
    let mut randnum;

    for _ in 0..15 {
        randnum = rand::thread_rng().gen_range(0, chars.len() - 2);
        id += &chars[randnum .. randnum + 1];
    }

    return id;

}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        items: Vec::new(),
        name: "".to_string(),
        price_string: "0".to_string(),
        tag: "".to_string(),
        message: "".to_string(),
        selected_tag: "".to_string()
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetName(name) => model.name = name,
        Msg::SetPrice(price_string) => model.price_string = price_string,
        Msg::SetTag(tag) => model.tag = tag,
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
                                id: make_id(),
                                name: model.name.clone(),
                                tag: model.tag.clone(),
                                price: price
                            });
                            model.name = "".to_string();
                            model.price_string = "0".to_string();
                            model.tag = "".to_string();
                            model.message = "".to_string();
                            model.selected_tag = "".to_string();
                        } else {
                            model.message = "商品/サービス価格は0以上の数値を入力してください".to_string()
                        }
                    } 
                    Err(_) => model.message = "商品/サービス価格は半角数字で入力してください".to_string()
                }
            }
        },
        Msg::ChangeSelectedTag(tag) => {
            if model.selected_tag == tag {
                model.selected_tag = "".to_string()
            } else {
                model.selected_tag = tag
            }
        },
        Msg::Delete(id) => {
            /* clone()との違いが分からない filterはイテレータを返すのでイテレータ専用のクローンメソッド? */
            model.items = model.items.iter().filter(| item | item.id != id).cloned().collect::<Vec<Item>>();
        }
    }
}

fn form_view(name: &str, price_string: &str, tag: &str, message: &str) -> Node<Msg> {
    div![
        style!{ St::Width => "90%", St::MarginTop => "auto", St::MarginLeft => "auto", St::MarginRight => "auto", St::MarginBottom => "3%" },
        h4!["購入履歴を追加", style!{ St::MarginBottom => "3%" }],
        div![
            attrs!{ At::Class => "mb-3" },
            style!{ St::MarginBottom => "2%" },
            label!["商品/サービス名"],
            input![attrs!{ At::Class => "form-control", At::Value => name }, input_ev(Ev::Input, Msg::SetName)],
        ],
        div![
            style!{ St::Display => "flex", St::MarginBottom => "2%" },
            /* 子要素が2つ以上ある場合はベクタを使う */
            vec![
                div![
                    label!["商品/サービス価格"],
                    input![attrs!{ At::Class => "form-control", At::Value => price_string }, input_ev(Ev::Input, Msg::SetPrice)],
                ],
                div![
                    label!["タグ"],
                    input![attrs!{ At::Class => "form-control", At::Value => tag }, input_ev(Ev::Input, Msg::SetTag)]
                ],
            ],
        ],
        button![
            "送信",
            attrs!{ At::Class => "btn btn-primary" },
            ev(Ev::Click, |_| Msg::Post)
        ],
        p![message]
    ]
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        layouts::header::view(),
        form_view(&model.name, &model.price_string, &model.tag, &model.message),
        /* 構造体要素のベクタを引数に取るビュー関数に分割しようとしたらエラーになった */
        div! [
            style!{ St::Width => "90%", St::Margin => "auto" },
            h4!["購入履歴", style!{ St::MarginBottom => "3%" }],
            p![if model.selected_tag == "".to_string() {"全購入履歴".to_string()} else {format!("タグ「{}」が付いた購入履歴 再度タグをクリックすると絞り込みが解除されます",  &model.selected_tag)}],
            table![
                attrs!{ At::Class => "table" },
                thead![
                    tr![
                        th!["商品/サービス名", attrs!{ At::Scope => "col" }],
                        th!["商品/サービス価格", attrs!{ At::Scope => "col" }],
                        th!["タグ", attrs!{ At::Scope => "col" }],
                        th!["", attrs!{ At::Scope => "col" }],
                    ],
                ],
                tbody![
                    model.items.clone().into_iter().map(| item | {
                        /* ブロックレベル要素で必ずラップする(divなど) */
                        tr![
                            style!{ St::Display => if model.selected_tag == "" || item.tag == model.selected_tag {""} else {"None"} },
                            td![&item.name],
                            td![&item.price],
                            td![
                                button![
                                    &item.tag, 
                                    attrs!{ At::Class => "btn btn-link" },
                                    ev(Ev::Click, |_| Msg::ChangeSelectedTag(item.tag))
                                ]
                            ],
                            td![
                                button![
                                    "削除",
                                    attrs!{ At::Class => "btn btn-danger" },
                                    /* ChangeSelectedTag()の引数にitemの値を取っているのでitemの所有権が関数側にムーブしてしまっている? */
                                    /* 参照を引数に取るように変更するとエラーになった */
                                    //ev(Ev::Click, |_| Msg::Delete(item.id)),
                                ]
                            ]
                        ]
                    })
                ]
            ]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}