use yew::prelude::*;

pub struct Board {
    props: Props
}

#[derive(Properties, Clone)]
pub struct Props {
    pub rows: usize,    // do self.props.rows to access
    pub cols: usize     // do self.props.cols to access
}

impl Board {
    fn view_board(&self) -> Html {
        let board_style = "border:1px solid #00B7FF; background-color: #00B7FF";

        html! {
            <div style="">
            // <script src=js></script>
                <canvas id="gameboard"
                    style=board_style
                    width="640"
                    height="480"></canvas>
            </div>
        }
    }
}

impl Component for Board {
    type Message = ();
    type Properties = Props;

    fn create(p: Self::Properties, _: ComponentLink<Self>) -> Self {
        Board {
            props: p
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html!{
            
            // <table border="1">
            <h3>
            {self.view_board()}
            </h3>
            //     <tr>
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //     </tr>
            //     <tr>
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //     </tr>
            //     <tr>
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //     </tr>
            //     <tr>
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //     </tr>
            //     <tr>
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //     </tr>
            //     <tr>
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td />
            //         <td /> 
            //     </tr>
            // </table>
        }
    }
}