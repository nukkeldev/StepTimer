use std::rc::Rc;

use gloo_timers::{callback::{Interval, Timeout}, future::{IntervalStream, TimeoutFuture}};
use yew::prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window};

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

#[macro_use]
macro_rules! dial {
    ($progress:expr,$end:expr,$thickness:expr) => {
        html! {
            <Dial progress={50}
        }
    };
}

#[function_component]
fn App() -> Html {
    let theme = use_memo(
        |_| Theme {
            foreground: "#ffffff".to_owned(),
            background: "#000000".to_owned(),
            primary: "#ff00ff".to_owned(),
            secondary: "#0000ff".to_owned(),
        },
        (),
    );

    html! {
        <ContextProvider<Rc<Theme>> context={theme}>
            <div class="dial-wrapper">
                <Dial progress={50} end={100} thickness={10} tick_time={1000} />
            </div>
        </ContextProvider<Rc<Theme>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialProps {
    progress: i32,
    end: i32,
    tick_time: u32,
    thickness: u8,
}

enum DialAction {
    Tick,
    Pause,
}

#[derive(Debug, Clone, PartialEq)]
struct DialState {
    progress: i32,
    end: i32,
    tick_time: u32,
}

impl Default for DialState {
    fn default() -> Self {
        Self {
            progress: 0,
            end: 100,
            tick_time: 500,
        }
    }
}

impl Reducible for DialState {
    type Action = DialAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next = match action {
            DialAction::Tick => self.progress + 1,
            DialAction::Pause => self.progress,
        };

        Self {
            progress: next,
            end: self.end,
            tick_time: self.tick_time,
        }
        .into()
    }
}

#[function_component]
pub fn Dial(props: &DialProps) -> Html {
    let DialProps {
        progress,
        end,
        thickness,
        tick_time
    } = props;

    let theme = use_context::<Rc<Theme>>().unwrap();

    let dial = use_reducer(|| DialState {
        progress: progress.clone(),
        end: end.clone(),
        tick_time: tick_time.clone(),
    });

    let dial_clone = dial.clone();

    use_state(|| spawn_local(async move {
        let dial = dial_clone.clone();

        let interval = Interval::new(dial.tick_time, move || {
            let dial = dial.clone();
            dial.dispatch(DialAction::Tick);
        });

        let dial = dial_clone.clone();

        TimeoutFuture::new((dial.end - dial.progress + 1) as u32 * dial.tick_time).await;

        interval.cancel();
    }));

    html! {
        <div class="dial"
            style={format!("--p:{};--b:{}px;--c:{};", dial.progress, thickness, theme.primary)} // --c: Color --p: Percent --b: Border Thickness
        >
            {format!("{}/{}", dial.progress, end)}
        </div>
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    foreground: String,
    background: String,

    primary: String,
    secondary: String,
}

fn main() {
    yew::Renderer::<App>::new().render();
}
