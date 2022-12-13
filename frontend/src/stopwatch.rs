use std::rc::Rc;

use gloo_timers::callback::Interval;
use yew::{html, use_context, Callback, Component, Context, Html, Properties};

use crate::Theme;

pub struct Stopwatch {
    start: i32,
    progress: i32,
    end: i32,

    interval: Option<Interval>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct StopwatchProps {
    pub start: i32,
    pub progress: i32,
    pub end: i32,
}

pub enum StopwatchMsg {
    Start,
    Pause,
    Reset,
    Tick,
}

impl Component for Stopwatch {
    type Message = StopwatchMsg;
    type Properties = StopwatchProps;

    fn create(ctx: &Context<Self>) -> Self {
        let Self::Properties {
            start,
            progress,
            end,
        } = ctx.props();
        Self {
            start: start.clone(),
            progress: progress.clone(),
            end: end.clone(),
            interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StopwatchMsg::Start => {
                let ticker = {
                    let link = ctx.link().clone();
                    Interval::new(1000, move || {
                        link.send_message(StopwatchMsg::Tick);
                    })
                };
                self.interval = Some(ticker);
            }
            StopwatchMsg::Pause => {
                self.interval.take();
            }
            StopwatchMsg::Reset => {
                self.progress = self.start;
            }
            StopwatchMsg::Tick => {
                self.progress += 1;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (theme, _) = ctx
            .link()
            .context::<Rc<Theme>>(Callback::from(|_| {}))
            .unwrap();

        let is_paused = self.interval.is_none();

        let onclick = ctx.link().callback(move |_| {
            if is_paused {
                StopwatchMsg::Start
            } else {
                StopwatchMsg::Pause
            }
        });

        html! {
            <div class="dial"
                style={format!("--p:{};--b:{}px;--c:{};", self.progress, 10, theme.primary)} // --c: Color --p: Percent --b: Border Thickness
            >
                <button {onclick}>{format!("{}/{}", self.progress, self.end)}</button>
            </div>
        }
    }
}
