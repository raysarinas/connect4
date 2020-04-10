use yew::prelude::*;

pub struct ConnectFourBoard {
    link: ComponentLink<Self>,
    clicked: Vec<Vec<bool>>,
    player_token: String,
    // player_turn: i8,
}

pub enum Msg {
    Click(usize, usize),
}

impl ConnectFourBoard {
    fn is_filled(&self, r: usize, c: usize) -> bool {
        !self.clicked[r][c] //&& self.player_turn > 0
    }

    fn change_token(&mut self) {
        match self.player_token.as_str() {
            "X" => self.player_token = "O".to_string(),
            "O" => self.player_token = "X".to_string(),
            _ => { }
        };
    }

    // fn change_turn(&mut self) {
    //     match self.player_turn {
    //         1 => self.player_turn = -1,
    //         -1 => self.player_turn = 1,
    //         _ => { }
    //     }
    // }

    fn get_token(&self) -> String {
        self.player_token.clone()
    }
}

impl Component for ConnectFourBoard {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConnectFourBoard {
            link,
            clicked: vec![vec![false; 7]; 6],
            player_token: "X".to_string(),
            // player_turn: 1,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {

        match msg {
            Msg::Click(row, col) => { // pass in the COL to Pranav's code
                if !self.clicked[row][col] {
                    self.clicked[row][col] = true;
                    // self.change_turn();
                    self.change_token();
                }
                // self.change_token();
                // self.change_turn();
            }
        };
        true
    }

    fn view(&self) -> Html {
        let col = |r, c| {
            html! {
                <td class="board_column", onclick=self.link.callback(move |_| Msg::Click(r, c)),>
                <div hidden=self.is_filled(r, c) >{ self.get_token() }</div>
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