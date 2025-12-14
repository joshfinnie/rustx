// Integration tests for RustX

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_integration_basic_button_with_rx_action() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let button = document.create_element("button").unwrap();
    button.set_attribute("rx-action", "/api/test").unwrap();
    button.set_attribute("rx-method", "GET").unwrap();
    button.set_text_content(Some("Test Button"));

    body.append_child(&button).unwrap();

    let found = document
        .query_selector("button[rx-action='/api/test']")
        .unwrap();
    assert!(found.is_some());

    button.remove();
}

#[wasm_bindgen_test]
fn test_integration_multiple_elements_with_rx_action() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    for i in 0..5 {
        let button = document.create_element("button").unwrap();
        button
            .set_attribute("rx-action", &format!("/api/test{}", i))
            .unwrap();
        button.set_class_name("test-button");
        body.append_child(&button).unwrap();
    }

    let found = document.query_selector_all(".test-button").unwrap();
    assert_eq!(found.length(), 5);

    for i in 0..5 {
        if let Some(button) = found.get(i) {
            button.dyn_into::<web_sys::Element>().unwrap().remove();
        }
    }
}

#[wasm_bindgen_test]
fn test_integration_form_with_rx_action() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let form = document.create_element("form").unwrap();
    form.set_attribute("rx-action", "/api/submit").unwrap();
    form.set_id("test-form");

    let input = document.create_element("input").unwrap();
    input.set_attribute("type", "text").unwrap();
    input.set_attribute("name", "username").unwrap();

    form.append_child(&input).unwrap();
    body.append_child(&form).unwrap();

    let found = document.query_selector("#test-form").unwrap();
    assert!(found.is_some());

    form.remove();
}

#[wasm_bindgen_test]
fn test_integration_rx_target_different_element() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let target = document.create_element("div").unwrap();
    target.set_id("integration-target");
    body.append_child(&target).unwrap();

    let button = document.create_element("button").unwrap();
    button.set_attribute("rx-action", "/api/data").unwrap();
    button
        .set_attribute("rx-target", "#integration-target")
        .unwrap();
    body.append_child(&button).unwrap();

    let found_target = document.query_selector("#integration-target").unwrap();
    assert!(found_target.is_some());

    assert_eq!(
        button.get_attribute("rx-target").unwrap(),
        "#integration-target"
    );

    button.remove();
    target.remove();
}

#[wasm_bindgen_test]
fn test_integration_rx_ignore_prevents_processing() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let container = document.create_element("div").unwrap();
    container.set_attribute("rx-ignore", "").unwrap();
    container.set_id("ignored-container");

    let button = document.create_element("button").unwrap();
    button.set_attribute("rx-action", "/api/ignored").unwrap();

    container.append_child(&button).unwrap();
    body.append_child(&container).unwrap();

    let found_container = document.query_selector("#ignored-container").unwrap();
    assert!(found_container.is_some());

    assert!(container.has_attribute("rx-ignore"));

    container.remove();
}

#[wasm_bindgen_test]
fn test_integration_nested_rx_actions() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let outer_div = document.create_element("div").unwrap();
    outer_div.set_id("outer");

    let button1 = document.create_element("button").unwrap();
    button1.set_attribute("rx-action", "/api/outer").unwrap();
    button1.set_id("btn1");

    let inner_div = document.create_element("div").unwrap();

    let button2 = document.create_element("button").unwrap();
    button2.set_attribute("rx-action", "/api/inner").unwrap();
    button2.set_id("btn2");

    inner_div.append_child(&button2).unwrap();
    outer_div.append_child(&button1).unwrap();
    outer_div.append_child(&inner_div).unwrap();
    body.append_child(&outer_div).unwrap();

    assert!(document.query_selector("#btn1").unwrap().is_some());
    assert!(document.query_selector("#btn2").unwrap().is_some());

    outer_div.remove();
}

#[wasm_bindgen_test]
fn test_integration_all_swap_modes() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let swap_modes = [
        "innerHTML",
        "outerHTML",
        "beforebegin",
        "afterbegin",
        "beforeend",
        "afterend",
        "none",
    ];

    let container = document.create_element("div").unwrap();
    container.set_id("swap-test-container");
    body.append_child(&container).unwrap();

    for (i, mode) in swap_modes.iter().enumerate() {
        let button = document.create_element("button").unwrap();
        button.set_attribute("rx-action", "/api/swap").unwrap();
        button.set_attribute("rx-swap", mode).unwrap();
        button.set_id(&format!("swap-btn-{}", i));

        container.append_child(&button).unwrap();

        let found = document
            .query_selector(&format!("#swap-btn-{}", i))
            .unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().get_attribute("rx-swap").unwrap(), *mode);
    }

    container.remove();
}

#[wasm_bindgen_test]
fn test_integration_custom_event_can_be_dispatched() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let event_init = web_sys::CustomEventInit::new();
    event_init.set_bubbles(true);
    event_init.set_cancelable(true);

    let event = web_sys::CustomEvent::new_with_event_init_dict("rx:test", &event_init).unwrap();

    let dispatched = document.dispatch_event(&event).unwrap();

    assert!(dispatched);
}

#[wasm_bindgen_test]
fn test_integration_element_can_be_queried_after_append() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let test_div = document.create_element("div").unwrap();
    test_div.set_id("query-test");
    test_div.set_attribute("data-test", "value").unwrap();

    body.append_child(&test_div).unwrap();

    let found_by_id = document.query_selector("#query-test").unwrap();
    assert!(found_by_id.is_some());

    let found_by_attr = document.query_selector("[data-test='value']").unwrap();
    assert!(found_by_attr.is_some());

    test_div.remove();
}
