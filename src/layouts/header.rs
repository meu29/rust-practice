use::seed::{prelude::*, *};

pub fn view<Msg>() -> Node<Msg> {

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
    ]

}