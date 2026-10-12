use custom_element::CustomElement;
use js_sys::Array;
use sycamore::prelude::*;
use wasm_bindgen::JsValue;
// use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, ShadowRootInit, ShadowRootMode};

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

// all custom element lifecycle hooks have a default
// no-op implementation
impl CustomElement for CoolButton {}

impl CoolButton {
    // called from the JavaScript custom element's `constructor`
    fn new(instance: JsValue, _args: Array) -> Self {
        let instance: HtmlElement = instance.into();
        let shadow_root_init = ShadowRootInit::new(ShadowRootMode::Open);
        let shadow_root = instance.attach_shadow(&shadow_root_init).unwrap();
        let document = web_sys::window().unwrap().document().unwrap();
        let div: Element = document.create_element("div").unwrap();

        shadow_root.append_child(&div).unwrap();

        sycamore::render_to(cool_button_view, &div);

        CoolButton {}
    }
}

pub fn main() {
    // create custom element constructor
    let component_name = "cool-button";
    let (closure, constructor) = custom_element::create_custom_element(CoolButton::new, vec![]);
    // we want our custom element to live forever
    closure.forget();

    // define the element
    let window = web_sys::window().unwrap();
    window
        .custom_elements()
        .define(component_name, &constructor)
        .unwrap();

    sycamore::render(root_view);
}
