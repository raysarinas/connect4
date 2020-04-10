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
        let col = || {
            html! {
                <td class="board_column">{""}</td>
            }
        };

        let row = || {
            html! {
                <tr>
                    {col()}
                    {col()}
                    {col()}
                    {col()}
                    {col()}
                    {col()}
                    {col()}
                </tr>
            }
        };

        html! {
            <div>
                <table class="board">
                    {row()}
                    {row()}
                    {row()}
                    {row()}
                    {row()}
                    {row()}
                </table>
                <br></br>
            </div>
        }
    }
}