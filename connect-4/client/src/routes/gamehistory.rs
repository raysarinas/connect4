use yew::prelude::*;

pub struct GameHistory {}

impl Component for GameHistory {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        GameHistory {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
            <div>
                <h1><b>{"Game History"}</b></h1>
                <div>
                    <table border="1">
                        <tr>
                            <th>{"Game-ID"}</th>
                            <th>{"Game Type"}</th>
                            <th>{"Player1"}</th>
                            <th>{"Player2"}</th>
                            <th>{"Winner"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                        <tr>
                            // populate here i guess
                        </tr>
                    </table>
                </div>
            </div>
        }
    }
}