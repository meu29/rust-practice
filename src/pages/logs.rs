use seed::{prelude::*, *};

struct Model {
    
} 

pub enum Msg {
    //DeleteLog(String)
}

pub fn view() -> Node<Msg> {
    div![
        h1![
            "ログ"
        ]
    ]
}