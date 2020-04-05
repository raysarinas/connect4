use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yew::prelude::*;

// MODEL
pub struct Model {
    link: ComponentLink<Self>,
    value: String,
}

// CONTROLLER
pub enum Msg {
    GotInput(String),
    Clicked,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            value: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.value = new_value;
            }
            Msg::Clicked => {
                self.value = "Changed Value".to_string();
            }
        }
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <textarea rows=5
                        value=&self.value
                        oninput=self.link.callback(|e: InputData| Msg::GotInput(e.value))
                        placeholder="placeholder">
                    </textarea>
                    <button onclick=self.link.callback(|_| Msg::Clicked)>{ "change value" }</button>
                </div>
                <div>
                    {&self.value}
                </div>
            </div>
        }
    }
}


impl Model {
    // HTML FILES SHOULD GO HERE AND BE RE-WRITTEN I THINK
    fn view_connect4_computer(&self) -> Html {
        html! {

        }
    }

    fn view_connect4_human(&self) -> Html {
        html! {
            
        }
    }

    fn view_howto_connect4(&self) -> Html {
        html! {
            
        }
    }

    fn view_toototto_computer(&self) -> Html {
        html! {

        }
    }

    fn view_toototto_human(&self) -> Html {
        html! {
            
        }
    }

    fn view_howto_toototto(&self) -> Html {
        html! {
            
        }
    }

    fn view_scoreboard(&self) -> Html {
        html! {

        }
    }

    fn view_human_player(&self) -> Html {
        html! {
            
        }
    }

    fn view_main(&self) -> Html {
        html! {
            
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}