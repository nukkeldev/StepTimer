use std::rc::Rc;

use gloo_timers::callback::{Interval, Timeout};
use js_sys::Array;
use yew::prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window};

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
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
                <Dial progress={50} end={100} thickness={10}/>
            </div>
        </ContextProvider<Rc<Theme>>>
    }
}

#[derive(PartialEq, Properties)]
pub struct DialProps {
    progress: i32,
    end: i32,
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
}

impl Default for DialState {
    fn default() -> Self {
        Self {
            progress: 0,
            end: 100,
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
    } = props;

    let theme = use_context::<Rc<Theme>>().unwrap();

    let dial = use_reducer(|| DialState {
        progress: progress.clone(),
        end: end.clone(),
    });

    {
        let dial = dial.clone();

        let interval = use_state(|| {
            Interval::new(1000, move || {
                dial.dispatch(DialAction::Tick);
            })
        });
    }

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
