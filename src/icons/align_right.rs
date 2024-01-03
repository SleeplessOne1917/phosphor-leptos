//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn AlignRight(
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
                <path d="M228,40V216a12,12,0,0,1-24,0V40a12,12,0,0,1,24,0ZM188,64v36a20,20,0,0,1-20,20H80a20,20,0,0,1-20-20V64A20,20,0,0,1,80,44h88A20,20,0,0,1,188,64Zm-24,4H84V96h80Zm24,88v36a20,20,0,0,1-20,20H40a20,20,0,0,1-20-20V156a20,20,0,0,1,20-20H168A20,20,0,0,1,188,156Zm-24,4H44v28H164Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M184,64v40a8,8,0,0,1-8,8H80a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8h96A8,8,0,0,1,184,64Zm-8,80H40a8,8,0,0,0-8,8v40a8,8,0,0,0,8,8H176a8,8,0,0,0,8-8V152A8,8,0,0,0,176,144Z"
        opacity="0.2"
    ></path>
    <path d="M224,40V216a8,8,0,0,1-16,0V40a8,8,0,0,1,16,0ZM192,64v40a16,16,0,0,1-16,16H80a16,16,0,0,1-16-16V64A16,16,0,0,1,80,48h96A16,16,0,0,1,192,64Zm-16,0H80v40h96Zm16,88v40a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V152a16,16,0,0,1,16-16H176A16,16,0,0,1,192,152Zm-16,0H40v40H176Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M224,40V216a8,8,0,0,1-16,0V40a8,8,0,0,1,16,0Zm-48,8H80A16,16,0,0,0,64,64v40a16,16,0,0,0,16,16h96a16,16,0,0,0,16-16V64A16,16,0,0,0,176,48Zm0,88H40a16,16,0,0,0-16,16v40a16,16,0,0,0,16,16H176a16,16,0,0,0,16-16V152A16,16,0,0,0,176,136Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M222,40V216a6,6,0,0,1-12,0V40a6,6,0,0,1,12,0ZM190,64v40a14,14,0,0,1-14,14H80a14,14,0,0,1-14-14V64A14,14,0,0,1,80,50h96A14,14,0,0,1,190,64Zm-12,0a2,2,0,0,0-2-2H80a2,2,0,0,0-2,2v40a2,2,0,0,0,2,2h96a2,2,0,0,0,2-2Zm12,88v40a14,14,0,0,1-14,14H40a14,14,0,0,1-14-14V152a14,14,0,0,1,14-14H176A14,14,0,0,1,190,152Zm-12,0a2,2,0,0,0-2-2H40a2,2,0,0,0-2,2v40a2,2,0,0,0,2,2H176a2,2,0,0,0,2-2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,40V216a8,8,0,0,1-16,0V40a8,8,0,0,1,16,0ZM192,64v40a16,16,0,0,1-16,16H80a16,16,0,0,1-16-16V64A16,16,0,0,1,80,48h96A16,16,0,0,1,192,64Zm-16,0H80v40h96Zm16,88v40a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V152a16,16,0,0,1,16-16H176A16,16,0,0,1,192,152Zm-16,0H40v40H176Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M220,40V216a4,4,0,0,1-8,0V40a4,4,0,0,1,8,0ZM188,64v40a12,12,0,0,1-12,12H80a12,12,0,0,1-12-12V64A12,12,0,0,1,80,52h96A12,12,0,0,1,188,64Zm-8,0a4,4,0,0,0-4-4H80a4,4,0,0,0-4,4v40a4,4,0,0,0,4,4h96a4,4,0,0,0,4-4Zm8,88v40a12,12,0,0,1-12,12H40a12,12,0,0,1-12-12V152a12,12,0,0,1,12-12H176A12,12,0,0,1,188,152Zm-8,0a4,4,0,0,0-4-4H40a4,4,0,0,0-4,4v40a4,4,0,0,0,4,4H176a4,4,0,0,0,4-4Z"></path>
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
