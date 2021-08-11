use seed::{prelude::*, *};

/* 自身と同じディレクトリに存在するpages.rsを読み込む */
mod pages;

struct Model {
    base_url: Url,
    page: Page
}

enum Msg {
    UrlChanged(subs::UrlChanged)
}

enum Page {
    Home,
    Logs
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home
        }
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
    fn logs(self) -> Url {
        self.base_url().add_path_part("logs")
    }
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url)
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        }
    }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        div![
            a![
                "記録を追加",
                attrs!{
                    At::Href => Urls::new(&model.base_url).home()
                }
            ],
            a![
                "ウォーキング履歴",
                attrs!{
                    At::Href => Urls::new(&model.base_url).logs()
                }
            ],
        ],
        view_main(&model.page)
    ]
}

fn view_main(model_page_field_value: &Page) -> Node<Msg> {
    div![
        match model_page_field_value {
            /* pages => modで読み込んだやつ */
            Page::Home => pages::home::view(),
            //Page::Logs => pages::logs::view()
        }
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}