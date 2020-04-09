use yew::prelude::*;

pub struct Board {}

impl Component for Board {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Board {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html!{
            <p>{"yes this is the gameboard"}</p>
        }
    }
}