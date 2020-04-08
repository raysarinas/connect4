use yew::prelude::*;

pub struct HowToTootOtto {}

impl Component for HowToTootOtto {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        HowToTootOtto {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
            <div>
                <h1><b>{"How to Play TOOT-OTTO"}</b></h1>
                <p>
                    {"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}
                </p>
                <h2>{"To play TOOT-OTTO follow the following steps:"}</h2>
                <ul>
                    <li>{"A new game describes which player is TOOT and which is OTTO"}</li>
                    <li>{"Select the disc type T or O that you want to place"}</li>
                    <li>{"Click on the desired column on the game board to place your disc"}</li>
                    <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally or vertically or diagonally"}</li>
                </ul>
                {"For More information on TOOT-OTTO click "}<a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a>
            </div>
        }
    }
}