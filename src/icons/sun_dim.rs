//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn SunDim(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M116,36V32a12,12,0,0,1,24,0v4a12,12,0,0,1-24,0Zm80,92a68,68,0,1,1-68-68A68.07,68.07,0,0,1,196,128Zm-24,0a44,44,0,1,0-44,44A44.05,44.05,0,0,0,172,128ZM51.51,68.49a12,12,0,1,0,17-17l-4-4a12,12,0,0,0-17,17Zm0,119-4,4a12,12,0,0,0,17,17l4-4a12,12,0,1,0-17-17ZM196,72a12,12,0,0,0,8.49-3.51l4-4a12,12,0,0,0-17-17l-4,4A12,12,0,0,0,196,72Zm8.49,115.51a12,12,0,0,0-17,17l4,4a12,12,0,0,0,17-17ZM48,128a12,12,0,0,0-12-12H32a12,12,0,0,0,0,24h4A12,12,0,0,0,48,128Zm80,80a12,12,0,0,0-12,12v4a12,12,0,0,0,24,0v-4A12,12,0,0,0,128,208Zm96-92h-4a12,12,0,0,0,0,24h4a12,12,0,0,0,0-24Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M184,128a56,56,0,1,1-56-56A56,56,0,0,1,184,128Z" opacity="0.2"></path>
    <path d="M120,40V32a8,8,0,0,1,16,0v8a8,8,0,0,1-16,0Zm72,88a64,64,0,1,1-64-64A64.07,64.07,0,0,1,192,128Zm-16,0a48,48,0,1,0-48,48A48.05,48.05,0,0,0,176,128ZM58.34,69.66A8,8,0,0,0,69.66,58.34l-8-8A8,8,0,0,0,50.34,61.66Zm0,116.68-8,8a8,8,0,0,0,11.32,11.32l8-8a8,8,0,0,0-11.32-11.32ZM192,72a8,8,0,0,0,5.66-2.34l8-8a8,8,0,0,0-11.32-11.32l-8,8A8,8,0,0,0,192,72Zm5.66,114.34a8,8,0,0,0-11.32,11.32l8,8a8,8,0,0,0,11.32-11.32ZM40,120H32a8,8,0,0,0,0,16h8a8,8,0,0,0,0-16Zm88,88a8,8,0,0,0-8,8v8a8,8,0,0,0,16,0v-8A8,8,0,0,0,128,208Zm96-88h-8a8,8,0,0,0,0,16h8a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M120,40V32a8,8,0,0,1,16,0v8a8,8,0,0,1-16,0Zm8,24a64,64,0,1,0,64,64A64.07,64.07,0,0,0,128,64ZM58.34,69.66A8,8,0,0,0,69.66,58.34l-8-8A8,8,0,0,0,50.34,61.66Zm0,116.68-8,8a8,8,0,0,0,11.32,11.32l8-8a8,8,0,0,0-11.32-11.32ZM192,72a8,8,0,0,0,5.66-2.34l8-8a8,8,0,0,0-11.32-11.32l-8,8A8,8,0,0,0,192,72Zm5.66,114.34a8,8,0,0,0-11.32,11.32l8,8a8,8,0,0,0,11.32-11.32ZM40,120H32a8,8,0,0,0,0,16h8a8,8,0,0,0,0-16Zm88,88a8,8,0,0,0-8,8v8a8,8,0,0,0,16,0v-8A8,8,0,0,0,128,208Zm96-88h-8a8,8,0,0,0,0,16h8a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M122,40V32a6,6,0,0,1,12,0v8a6,6,0,0,1-12,0Zm68,88a62,62,0,1,1-62-62A62.07,62.07,0,0,1,190,128Zm-12,0a50,50,0,1,0-50,50A50.06,50.06,0,0,0,178,128ZM59.76,68.24a6,6,0,1,0,8.48-8.48l-8-8a6,6,0,0,0-8.48,8.48Zm0,119.52-8,8a6,6,0,1,0,8.48,8.48l8-8a6,6,0,1,0-8.48-8.48Zm136-136-8,8a6,6,0,1,0,8.48,8.48l8-8a6,6,0,0,0-8.48-8.48Zm.48,136a6,6,0,0,0-8.48,8.48l8,8a6,6,0,0,0,8.48-8.48ZM40,122H32a6,6,0,0,0,0,12h8a6,6,0,0,0,0-12Zm88,88a6,6,0,0,0-6,6v8a6,6,0,0,0,12,0v-8A6,6,0,0,0,128,210Zm96-88h-8a6,6,0,0,0,0,12h8a6,6,0,0,0,0-12Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M120,40V32a8,8,0,0,1,16,0v8a8,8,0,0,1-16,0Zm72,88a64,64,0,1,1-64-64A64.07,64.07,0,0,1,192,128Zm-16,0a48,48,0,1,0-48,48A48.05,48.05,0,0,0,176,128ZM58.34,69.66A8,8,0,0,0,69.66,58.34l-8-8A8,8,0,0,0,50.34,61.66Zm0,116.68-8,8a8,8,0,0,0,11.32,11.32l8-8a8,8,0,0,0-11.32-11.32ZM192,72a8,8,0,0,0,5.66-2.34l8-8a8,8,0,0,0-11.32-11.32l-8,8A8,8,0,0,0,192,72Zm5.66,114.34a8,8,0,0,0-11.32,11.32l8,8a8,8,0,0,0,11.32-11.32ZM40,120H32a8,8,0,0,0,0,16h8a8,8,0,0,0,0-16Zm88,88a8,8,0,0,0-8,8v8a8,8,0,0,0,16,0v-8A8,8,0,0,0,128,208Zm96-88h-8a8,8,0,0,0,0,16h8a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M124,40V32a4,4,0,0,1,8,0v8a4,4,0,0,1-8,0Zm64,88a60,60,0,1,1-60-60A60.07,60.07,0,0,1,188,128Zm-8,0a52,52,0,1,0-52,52A52.06,52.06,0,0,0,180,128ZM61.17,66.83a4,4,0,0,0,5.66-5.66l-8-8a4,4,0,0,0-5.66,5.66Zm0,122.34-8,8a4,4,0,0,0,5.66,5.66l8-8a4,4,0,0,0-5.66-5.66Zm136-136-8,8a4,4,0,0,0,5.66,5.66l8-8a4,4,0,1,0-5.66-5.66Zm-2.34,136a4,4,0,0,0-5.66,5.66l8,8a4,4,0,0,0,5.66-5.66ZM40,124H32a4,4,0,0,0,0,8h8a4,4,0,0,0,0-8Zm88,88a4,4,0,0,0-4,4v8a4,4,0,0,0,8,0v-8A4,4,0,0,0,128,212Zm96-88h-8a4,4,0,0,0,0,8h8a4,4,0,0,0,0-8Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().unwrap_or(TextProp::from(""))
            class=move || class.get().unwrap_or(TextProp::from(""))
        >
            {body}
        </svg>
    }
}
