use custom_element::CustomElement;
use js_sys::Array;
use sycamore::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{HtmlElement, ShadowRootInit, ShadowRootMode};

#[component]
fn cool_button_view() -> View {
    let mut count = create_signal(0);
    let increment = move |_| count += 1;

    view! {
      button(on:click=increment) {
        "Click:" (count)
      }
    }
}

#[component]
fn root_view() -> View {
    view! {
      cool-button { }
    }
}

struct CoolButton;

impl CustomElement for CoolButton {}

impl CoolButton {
    fn new(instance: JsValue, _args: Array) -> Self {
        let instance: HtmlElement = instance.into();
        let shadow_root_init = ShadowRootInit::new(ShadowRootMode::Open);
        let shadow_root = instance.attach_shadow(&shadow_root_init).unwrap();

        sycamore::render_to(cool_button_view, &shadow_root);

        Self {}
    }

    fn define(name: &str) -> () {
        let (closure, constructor) = custom_element::create_custom_element(Self::new, vec![]);
        closure.forget();

        let window = web_sys::window().unwrap();
        window.custom_elements().define(name, &constructor).unwrap();
    }
}

pub fn main() {
    CoolButton::define("cool-button");

    sycamore::render(root_view);
}
