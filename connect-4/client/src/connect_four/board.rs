use crate::game_elements::board::*;

use yew::prelude::*;

pub struct ConnectFourBoard {}

impl Component for ConnectFourBoard {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ConnectFourBoard {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html!{
            <div>
                <Board rows=6 cols=7 />
            </div>
        }
    }
}