pub struct Stopwatch;

pub enum StopwatchMsg {
}

impl Component for Stopwatch {
    type Message = StopwatchMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            
        }
    }
}