use yew::prelude::*;

pub struct ConnectFourBoard {
    link: ComponentLink<Self>,
    clicked: Vec<Vec<bool>>,
}

pub enum Msg {
    Click(usize, usize),
}

impl Component for ConnectFourBoard {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConnectFourBoard {
            link,
            clicked: vec![vec![false; 7]; 6],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {

        match msg {
            Msg::Click(row, col) => { // pass in the COL to Pranav's code
                self.clicked[row][col] = true; 
            }
        };
        true
    }

    fn view(&self) -> Html {
        let col = |r, c| {
            html! {
                <td class="board_column", onclick=self.link.callback(move |_| Msg::Click(r, c)),>
                <div hidden=!self.clicked[r][c]>{ "x" }</div>
                </td>
            }
        };

        let row = |r| {
            html! {
                <tr>
                    {col(r, 0)}
                    {col(r, 1)}
                    {col(r, 2)}
                    {col(r, 3)}
                    {col(r, 4)}
                    {col(r, 5)}
                    {col(r, 6)}
                </tr>
            }
        };

        html! {
            <div>
                <table class="board">
                    {row(0)}
                    {row(1)}
                    {row(2)}
                    {row(3)}
                    {row(4)}
                    {row(5)}
                </table>
                <br></br>
            </div>
        }
    }
}