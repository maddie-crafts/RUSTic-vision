use leptos::*;
use leptos::logging::log;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Event};
use gloo::file::callbacks::FileReader as GlooFileReader;
use wasm_bindgen::JsValue;

use crate::ascii;

#[component]
pub fn App() -> impl IntoView {
    let ascii_output = create_rw_signal::<Vec<String>>(vec![]);
    let download_ascii = move |_| {
        let ascii = ascii_output.get().join("\n");
        let blob = web_sys::Blob::new_with_str_sequence(&js_sys::Array::of1(&JsValue::from_str(&ascii))).unwrap();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        
        let document = web_sys::window().unwrap().document().unwrap();
        let a = document.create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
        a.set_href(&url);
        a.set_download("ascii.txt");
        a.click();
        web_sys::Url::revoke_object_url(&url).ok();
    };
    let reader_handle = create_rw_signal::<Option<GlooFileReader>>(None);

    let on_file_change = move |ev: leptos::ev::Event| {
        let target: HtmlInputElement = event_target::<Event>(&ev).unchecked_into();
        let file_list = target.files();
        if let Some(files) = file_list {
            if let Some(file) = files.get(0) {
                let filename = file.name();
                log!("📂 File selected: {filename}");

                let blob = gloo::file::Blob::from(file);
                let reader = gloo::file::callbacks::read_as_bytes(&blob, move |res| {
                    match res {
                        Ok(bytes) => {
                            // Convert bytes into an image
                            match image::load_from_memory(&bytes) {
                                Ok(img) => {
                                    let ascii = ascii::convert(img, 80, 2.0);
                                    ascii_output.set(ascii);
                                }
                                Err(e) => {
                                    log!("❌ Failed to decode image: {e:?}");
                                }
                            }
                        }
                        Err(err) => log!("❌ Error reading file: {err:?}"),
                    }
                });

                reader_handle.set(Some(reader));
            }
        }
    };

    view! {
        <main class="p-4 max-w-2xl mx-auto">
        <h1 class="text-3xl font-bold mb-4">{"Upload Image -> ASCII"}</h1>
            <input
                type="file"
                accept="image/*"
                class="mb-4"
                on:change=on_file_change
            />
            <pre class="bg-gray-900 text-green-400 text-sm whitespace-pre-wrap font-mono p-2">
                {move || ascii_output.get().join("\n")}
            </pre>
        <button on:click=download_ascii class="bg-blue-500 text-white px-4 py-2 rounded mb-4">
            "Download ASCII"
        </button>
        </main>
    }
}