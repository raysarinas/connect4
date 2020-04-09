use yew::prelude::*;

pub struct Board {
    props: Props
}

#[derive(Properties, Clone)]
pub struct Props {
    pub rows: usize,    // do self.props.rows to access
    pub cols: usize     // do self.props.cols to access
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
            <table border="1">
                <tr>
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                </tr>
                <tr>
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                </tr>
                <tr>
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                </tr>
                <tr>
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                </tr>
                <tr>
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                </tr>
                <tr>
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td />
                    <td /> 
                </tr>
            </table>
        }
    }
}