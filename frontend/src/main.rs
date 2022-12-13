use std::rc::Rc;

use gloo_timers::{
    callback::{Interval, Timeout},
    future::{IntervalStream, TimeoutFuture},
};
use yew::prelude::*;

pub mod stopwatch;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window};

use crate::stopwatch::Stopwatch;

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
                <Stopwatch start={0} progress={50} end={100} />
            </div>
        </ContextProvider<Rc<Theme>>>
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
