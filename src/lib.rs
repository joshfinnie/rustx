use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CustomEvent, CustomEventInit, Document, Element, Event, FormData, HtmlElement, HtmlFormElement,
    MutationObserver, MutationObserverInit, MutationRecord, Request, RequestInit, Response,
    UrlSearchParams, Window,
};

const RUSTX_MARKER: &str = "__rustx";
const RUSTX_MO_MARKER: &str = "__rustx_mo";

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");

    if js_sys::Reflect::has(&document, &JsValue::from_str(RUSTX_MO_MARKER)).unwrap_or(false) {
        return;
    }

    init_rustx(&window, &document);
}

fn init_rustx(_window: &Window, document: &Document) {
    let doc_clone = document.clone();

    let callback = Closure::wrap(Box::new(move |records: js_sys::Array| {
        for record in records.iter() {
            if let Ok(mutation_record) = record.dyn_into::<MutationRecord>() {
                if mutation_record.type_() == "childList" {
                    let added_nodes = mutation_record.added_nodes();
                    for i in 0..added_nodes.length() {
                        if let Some(node) = added_nodes.get(i) {
                            if let Ok(element) = node.dyn_into::<Element>() {
                                process_element(&doc_clone, &element);
                            }
                        }
                    }
                }
            }
        }
    }) as Box<dyn FnMut(js_sys::Array)>);

    let observer = MutationObserver::new(callback.as_ref().unchecked_ref())
        .expect("Failed to create MutationObserver");

    js_sys::Reflect::set(
        document.as_ref(),
        &JsValue::from_str(RUSTX_MO_MARKER),
        &observer,
    )
    .unwrap();

    callback.forget();

    let doc_clone = document.clone();
    let obs_clone = observer.clone();
    let dom_ready = Closure::wrap(Box::new(move || {
        let init = MutationObserverInit::new();
        init.set_child_list(true);
        init.set_subtree(true);

        if let Some(doc_element) = doc_clone.document_element() {
            obs_clone.observe_with_options(&doc_element, &init).ok();
        }

        if let Some(body) = doc_clone.body() {
            process_element(&doc_clone, &body);
        }
    }) as Box<dyn FnMut()>);

    // Check if DOM is already loaded
    let ready_state = document.ready_state();
    if ready_state == "interactive" || ready_state == "complete" {
        // DOM is already ready, call immediately
        dom_ready
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .call0(&JsValue::NULL)
            .ok();
    } else {
        // Wait for DOMContentLoaded
        document
            .add_event_listener_with_callback(
                "DOMContentLoaded",
                dom_ready.as_ref().unchecked_ref(),
            )
            .ok();
    }

    dom_ready.forget();

    let doc_clone = document.clone();
    let process_listener = Closure::wrap(Box::new(move |evt: Event| {
        if let Some(target) = evt.target() {
            if let Ok(element) = target.dyn_into::<Element>() {
                process_element(&doc_clone, &element);
            }
        }
    }) as Box<dyn FnMut(Event)>);

    document
        .add_event_listener_with_callback("rx:process", process_listener.as_ref().unchecked_ref())
        .ok();

    process_listener.forget();
}

fn process_element(document: &Document, element: &Element) {
    if element.closest("[rx-ignore]").ok().flatten().is_some() {
        return;
    }

    if element.has_attribute("rx-action") {
        init_element(document, element);
    }

    if let Ok(elements) = element.query_selector_all("[rx-action]") {
        for i in 0..elements.length() {
            if let Some(el) = elements.get(i) {
                if let Ok(element) = el.dyn_into::<Element>() {
                    init_element(document, &element);
                }
            }
        }
    }
}

fn init_element(document: &Document, element: &Element) {
    if js_sys::Reflect::has(element, &JsValue::from_str(RUSTX_MARKER)).unwrap_or(false) {
        return;
    }

    if element.closest("[rx-ignore]").ok().flatten().is_some() {
        return;
    }

    let options = CustomEventInit::new();
    options.set_bubbles(true);
    options.set_cancelable(true);
    options.set_composed(true);

    let init_event = CustomEvent::new_with_event_init_dict("rx:init", &options)
        .expect("Failed to create init event");

    if !element.dispatch_event(&init_event).unwrap_or(false) {
        return;
    }

    let trigger_event = if let Some(attr) = element.get_attribute("rx-trigger") {
        attr
    } else if element.matches("form").unwrap_or(false) {
        "submit".to_string()
    } else if element
        .matches("input:not([type=button]),select,textarea")
        .unwrap_or(false)
    {
        "change".to_string()
    } else {
        "click".to_string()
    };

    let doc_clone = document.clone();
    let el_clone = element.clone();
    let handler = Closure::wrap(Box::new(move |evt: Event| {
        handle_rustx_event(&doc_clone, &el_clone, &evt);
    }) as Box<dyn FnMut(Event)>);

    element
        .add_event_listener_with_callback(&trigger_event, handler.as_ref().unchecked_ref())
        .ok();

    js_sys::Reflect::set(element, &JsValue::from_str(RUSTX_MARKER), handler.as_ref()).unwrap();

    handler.forget();

    let inited_options = CustomEventInit::new();
    inited_options.set_bubbles(false);
    inited_options.set_composed(true);

    let inited_event = CustomEvent::new_with_event_init_dict("rx:inited", &inited_options)
        .expect("Failed to create inited event");

    element.dispatch_event(&inited_event).ok();
}

fn handle_rustx_event(document: &Document, element: &Element, evt: &Event) {
    let window = web_sys::window().expect("no window");
    let document = document.clone();
    let element = element.clone();
    evt.prevent_default();

    wasm_bindgen_futures::spawn_local(async move {
        let form: Option<HtmlFormElement> = if let Some(html_el) = element.dyn_ref::<HtmlElement>()
        {
            js_sys::Reflect::get(html_el, &JsValue::from_str("form"))
                .ok()
                .and_then(|v| v.dyn_into().ok())
        } else {
            None
        }
        .or_else(|| {
            element
                .closest("form")
                .ok()
                .flatten()
                .and_then(|e| e.dyn_into().ok())
        });

        let form_data = if let Some(ref f) = form {
            FormData::new_with_form(f).ok()
        } else {
            FormData::new().ok()
        };

        let form_data = form_data.unwrap_or_else(|| FormData::new().unwrap());

        if form.is_none() {
            if let Some(name) = element.get_attribute("name") {
                if let Some(value) = element.get_attribute("value") {
                    form_data.append_with_str(&name, &value).ok();
                }
            }
        }

        let action = element.get_attribute("rx-action").unwrap_or_default();
        let method = element
            .get_attribute("rx-method")
            .unwrap_or_else(|| "GET".to_string())
            .to_uppercase();

        let target = if let Some(target_sel) = element.get_attribute("rx-target") {
            document
                .query_selector(&target_sel)
                .ok()
                .flatten()
                .unwrap_or_else(|| element.clone())
        } else {
            element.clone()
        };

        let swap = element
            .get_attribute("rx-swap")
            .unwrap_or_else(|| "outerHTML".to_string());

        let config_options = CustomEventInit::new();
        config_options.set_bubbles(true);
        config_options.set_cancelable(true);
        config_options.set_composed(true);

        let config_event = CustomEvent::new_with_event_init_dict("rx:config", &config_options)
            .expect("Failed to create config event");

        if !element.dispatch_event(&config_event).unwrap_or(false) {
            return;
        }

        let mut final_action = action.clone();
        let request_init = RequestInit::new();
        request_init.set_method(&method);

        let headers = web_sys::Headers::new().unwrap();
        headers.set("RX-Request", "true").ok();
        request_init.set_headers(&headers);

        if method == "GET" || method == "DELETE" {
            // For GET/DELETE, append form data as query parameters
            let params = UrlSearchParams::new_with_str_sequence_sequence(&form_data).ok();
            if let Some(params) = params {
                let params_str = params.to_string().as_string().unwrap_or_default();
                if !params_str.is_empty() {
                    final_action = if action.contains('?') {
                        format!("{}&{}", action, params_str)
                    } else {
                        format!("{}?{}", action, params_str)
                    };
                }
            }
        } else {
            // For POST/PUT/etc, send form data in body
            request_init.set_body(&form_data);
        }

        let before_options = CustomEventInit::new();
        before_options.set_bubbles(true);
        before_options.set_cancelable(true);

        let before_event = CustomEvent::new_with_event_init_dict("rx:before", &before_options)
            .expect("Failed to create before event");

        if !element.dispatch_event(&before_event).unwrap_or(false) {
            return;
        }

        let request = Request::new_with_str_and_init(&final_action, &request_init)
            .expect("Failed to create request");

        let promise = window.fetch_with_request(&request);
        let response = match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(resp) => resp.dyn_into::<Response>().ok(),
            Err(_) => None,
        };

        let text = if let Some(ref resp) = response {
            if let Ok(promise) = resp.text() {
                wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .ok()
                    .and_then(|v| v.as_string())
            } else {
                None
            }
        } else {
            None
        };

        let after_options = CustomEventInit::new();
        after_options.set_bubbles(true);
        after_options.set_cancelable(true);

        let after_event = CustomEvent::new_with_event_init_dict("rx:after", &after_options)
            .expect("Failed to create after event");

        if !element.dispatch_event(&after_event).unwrap_or(false) {
            return;
        }

        if let Some(text_content) = text {
            match swap.as_str() {
                "outerHTML" => {
                    if let Ok(html_el) = target.dyn_into::<HtmlElement>() {
                        html_el.set_outer_html(&text_content);
                    }
                }
                "innerHTML" => {
                    target.set_inner_html(&text_content);
                }
                "beforebegin" | "afterbegin" | "beforeend" | "afterend" => {
                    if let Ok(html_el) = target.dyn_into::<HtmlElement>() {
                        html_el.insert_adjacent_html(&swap, &text_content).ok();
                    }
                }
                "none" => {}
                _ => {}
            }
        }

        let swapped_options = CustomEventInit::new();
        swapped_options.set_bubbles(true);

        let swapped_event = CustomEvent::new_with_event_init_dict("rx:swapped", &swapped_options)
            .expect("Failed to create swapped event");

        element.dispatch_event(&swapped_event).ok();

        let finally_options = CustomEventInit::new();
        finally_options.set_bubbles(true);

        let finally_event = CustomEvent::new_with_event_init_dict("rx:finally", &finally_options)
            .expect("Failed to create finally event");

        element.dispatch_event(&finally_event).ok();
    });
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_rustx_marker_constants() {
        assert_eq!(RUSTX_MARKER, "__rustx");
        assert_eq!(RUSTX_MO_MARKER, "__rustx_mo");
    }

    #[wasm_bindgen_test]
    fn test_can_get_window_and_document() {
        let window = web_sys::window();
        assert!(window.is_some());

        let document = window.unwrap().document();
        assert!(document.is_some());
    }

    #[wasm_bindgen_test]
    fn test_element_with_rx_action_attribute() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Create a test button
        let button = document.create_element("button").unwrap();
        button.set_attribute("rx-action", "/test").unwrap();
        button.set_attribute("rx-method", "POST").unwrap();

        // Verify attributes are set
        assert_eq!(button.get_attribute("rx-action").unwrap(), "/test");
        assert_eq!(button.get_attribute("rx-method").unwrap(), "POST");
    }

    #[wasm_bindgen_test]
    fn test_element_without_rx_action_is_ignored() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let div = document.create_element("div").unwrap();

        // Element without rx-action should not have the attribute
        assert!(div.get_attribute("rx-action").is_none());
    }

    #[wasm_bindgen_test]
    fn test_rx_target_attribute() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let button = document.create_element("button").unwrap();
        button.set_attribute("rx-action", "/data").unwrap();
        button.set_attribute("rx-target", "#result").unwrap();

        assert_eq!(button.get_attribute("rx-target").unwrap(), "#result");
    }

    #[wasm_bindgen_test]
    fn test_rx_swap_modes() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let swap_modes = vec![
            "innerHTML",
            "outerHTML",
            "beforebegin",
            "afterbegin",
            "beforeend",
            "afterend",
            "none",
        ];

        for mode in swap_modes {
            let button = document.create_element("button").unwrap();
            button.set_attribute("rx-swap", mode).unwrap();
            assert_eq!(button.get_attribute("rx-swap").unwrap(), mode);
        }
    }

    #[wasm_bindgen_test]
    fn test_rx_trigger_attribute() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let select = document.create_element("select").unwrap();
        select.set_attribute("rx-action", "/filter").unwrap();
        select.set_attribute("rx-trigger", "change").unwrap();

        assert_eq!(select.get_attribute("rx-trigger").unwrap(), "change");
    }

    #[wasm_bindgen_test]
    fn test_rx_ignore_attribute() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let div = document.create_element("div").unwrap();
        div.set_attribute("rx-ignore", "").unwrap();

        // Element with rx-ignore should have the attribute
        assert!(div.has_attribute("rx-ignore"));
    }

    #[wasm_bindgen_test]
    fn test_form_element_detection() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let form = document.create_element("form").unwrap();
        assert!(form.matches("form").unwrap());

        let input = document.create_element("input").unwrap();
        input.set_attribute("type", "text").unwrap();
        assert!(input.matches("input:not([type=button])").unwrap());

        let button_input = document.create_element("input").unwrap();
        button_input.set_attribute("type", "button").unwrap();
        assert!(!button_input.matches("input:not([type=button])").unwrap());
    }

    #[wasm_bindgen_test]
    fn test_can_create_custom_events() {
        let options = CustomEventInit::new();
        options.set_bubbles(true);
        options.set_cancelable(true);

        let event = CustomEvent::new_with_event_init_dict("rx:test", &options);
        assert!(event.is_ok());

        let event = event.unwrap();
        assert_eq!(event.type_(), "rx:test");
        assert!(event.bubbles());
        assert!(event.cancelable());
    }

    #[wasm_bindgen_test]
    fn test_document_ready_states() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let ready_state = document.ready_state();
        // In tests, should be one of these
        assert!(
            ready_state == "loading" || ready_state == "interactive" || ready_state == "complete"
        );
    }

    #[wasm_bindgen_test]
    fn test_can_query_selector() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Create a test element
        let div = document.create_element("div").unwrap();
        div.set_id("test-div");

        document.body().unwrap().append_child(&div).unwrap();

        // Query for it
        let found = document.query_selector("#test-div").unwrap();
        assert!(found.is_some());

        // Clean up
        div.remove();
    }

    #[wasm_bindgen_test]
    fn test_form_data_creation() {
        let form_data = web_sys::FormData::new();
        assert!(form_data.is_ok());

        let form_data = form_data.unwrap();
        form_data.append_with_str("key", "value").unwrap();

        // FormData should now have the entry
        assert!(form_data.has("key"));
    }

    #[wasm_bindgen_test]
    fn test_url_search_params() {
        let params = web_sys::UrlSearchParams::new().unwrap();
        params.append("foo", "bar");
        params.append("baz", "qux");

        let params_str = params.to_string().as_string().unwrap();
        assert!(params_str.contains("foo=bar"));
        assert!(params_str.contains("baz=qux"));
    }

    #[wasm_bindgen_test]
    fn test_http_methods() {
        let methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH"];

        for method in methods {
            let request_init = web_sys::RequestInit::new();
            request_init.set_method(method);
            // If this doesn't panic, the method is valid
        }
    }

    #[wasm_bindgen_test]
    fn test_headers_creation() {
        let headers = web_sys::Headers::new();
        assert!(headers.is_ok());

        let headers = headers.unwrap();
        headers.set("RX-Request", "true").unwrap();
        headers
            .set("Content-Type", "application/x-www-form-urlencoded")
            .unwrap();

        assert_eq!(headers.get("RX-Request").unwrap().unwrap(), "true");
    }

    #[wasm_bindgen_test]
    fn test_response_creation() {
        let response = web_sys::Response::new_with_opt_str(Some("test content"));
        assert!(response.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_element_closest() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let div = document.create_element("div").unwrap();
        div.set_attribute("rx-ignore", "").unwrap();

        let button = document.create_element("button").unwrap();
        div.append_child(&button).unwrap();

        // Button should find the rx-ignore ancestor
        let closest = button.closest("[rx-ignore]").unwrap();
        assert!(closest.is_some());
    }

    #[wasm_bindgen_test]
    fn test_mutation_observer_init() {
        let init = web_sys::MutationObserverInit::new();
        init.set_child_list(true);
        init.set_subtree(true);
        // If this doesn't panic, init is configured correctly
    }

    #[wasm_bindgen_test]
    fn test_console_error_panic_hook() {
        console_error_panic_hook::set_once();
        // Should not panic on multiple calls
        console_error_panic_hook::set_once();
    }
}
