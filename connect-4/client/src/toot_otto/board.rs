use crate::game_elements::board::*;

use yew::prelude::*;

pub struct TootOttoBoard {}

impl Component for TootOttoBoard {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TootOttoBoard {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html!{
            <div>
                <Board rows=4 cols=6 />
            </div>
        }
    }
}