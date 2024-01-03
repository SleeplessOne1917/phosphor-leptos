//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Tree(
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
                <path d="M201.18,59.63a80,80,0,0,0-146.36,0A76.29,76.29,0,0,0,12,127.79c-.11,41,33.1,75.15,74,76.19a75.84,75.84,0,0,0,30-5.31V232a12,12,0,0,0,24,0V198.67A75.79,75.79,0,0,0,168,204l2,0c40.94-1,74.15-35.22,74-76.19A76.29,76.29,0,0,0,201.18,59.63ZM169.35,180A51.51,51.51,0,0,1,140,171.8V135.42l41.37-20.69a12,12,0,1,0-10.74-21.46L140,108.58V88a12,12,0,0,0-24,0v44.58L85.37,117.27a12,12,0,0,0-10.74,21.46L116,159.42V171.8A51.43,51.43,0,0,1,86.65,180,52,52,0,0,1,66.27,80.76,20,20,0,0,0,76.34,70.34a56,56,0,0,1,103.32,0,20,20,0,0,0,10.07,10.42A52,52,0,0,1,169.35,180Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,127.82a64,64,0,0,1-99.52,53.41,8,8,0,0,0-9,0A64,64,0,1,1,61.25,69.86a8,8,0,0,0,4-4.17,68,68,0,0,1,125.44,0,8,8,0,0,0,4,4.17A64,64,0,0,1,232,127.82Z"
        opacity="0.2"
    ></path>
    <path d="M198.1,62.6a76,76,0,0,0-140.2,0A72.29,72.29,0,0,0,16,127.8C15.89,166.62,47.36,199,86.14,200A71.68,71.68,0,0,0,120,192.49V232a8,8,0,0,0,16,0V192.49A71.45,71.45,0,0,0,168,200l1.86,0c38.78-1,70.25-33.36,70.14-72.18A72.26,72.26,0,0,0,198.1,62.6ZM169.45,184a55.7,55.7,0,0,1-32.52-9.4q-.47-.3-.93-.57V132.94l43.58-21.78a8,8,0,1,0-7.16-14.32L136,115.06V88a8,8,0,0,0-16,0v51.06L83.58,120.84a8,8,0,1,0-7.16,14.32L120,156.94V174c-.31.18-.62.37-.92.57A55.73,55.73,0,0,1,86.55,184a56,56,0,0,1-22-106.86,15.9,15.9,0,0,0,8.05-8.33,60,60,0,0,1,110.7,0,15.9,15.9,0,0,0,8.05,8.33,56,56,0,0,1-22,106.86Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M128,187.88s0,0-.06,0a71.3,71.3,0,0,1-8,4.61V232a8,8,0,0,0,16,0V192.49a71.3,71.3,0,0,1-8-4.61Z"></path>
    <path d="M198.1,62.6a76,76,0,0,0-140.2,0A72.29,72.29,0,0,0,16,127.8C15.89,166.62,47.36,199,86.14,200A71.68,71.68,0,0,0,120,192.49V156.94L76.42,135.16a8,8,0,1,1,7.16-14.32L120,139.06V88a8,8,0,0,1,16,0v27.06l36.42-18.22a8,8,0,1,1,7.16,14.32L136,132.94v59.55A71.45,71.45,0,0,0,168,200l1.86,0c38.78-1,70.25-33.36,70.14-72.18A72.26,72.26,0,0,0,198.1,62.6Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M197.26,64.41a2,2,0,0,1-1-1,74,74,0,0,0-136.52,0,2,2,0,0,1-1,1A70.25,70.25,0,0,0,18,127.8C17.9,165.54,48.49,197,86.2,198a69.4,69.4,0,0,0,35.8-8.8V232a6,6,0,0,0,12,0V189.18A69.54,69.54,0,0,0,168,198l1.81,0c37.7-1,68.29-32.44,68.19-70.18A70.27,70.27,0,0,0,197.26,64.41ZM169.5,186a57.58,57.58,0,0,1-33.69-9.74,14.77,14.77,0,0,0-1.81-1v-43.5l44.68-22.34a6,6,0,1,0-5.36-10.74L134,118.29V88a6,6,0,0,0-12,0v54.29L82.68,122.63a6,6,0,0,0-5.36,10.74L122,155.71v19.5a14.77,14.77,0,0,0-1.81,1A57.51,57.51,0,0,1,86.5,186,58,58,0,0,1,63.76,75.31,14,14,0,0,0,70.81,68a62,62,0,0,1,114.38,0,14,14,0,0,0,7.05,7.29A58,58,0,0,1,169.5,186Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M198.1,62.6a76,76,0,0,0-140.2,0A72.27,72.27,0,0,0,16,127.8C15.89,166.62,47.36,199,86.14,200A71.68,71.68,0,0,0,120,192.49V232a8,8,0,0,0,16,0V192.49A71.45,71.45,0,0,0,168,200l1.86,0c38.78-1,70.25-33.36,70.14-72.18A72.26,72.26,0,0,0,198.1,62.6ZM169.45,184a55.61,55.61,0,0,1-32.52-9.4q-.47-.3-.93-.57V132.94l43.58-21.78a8,8,0,1,0-7.16-14.32L136,115.06V88a8,8,0,0,0-16,0v51.06L83.58,120.84a8,8,0,1,0-7.16,14.32L120,156.94V174q-.47.27-.93.57A55.7,55.7,0,0,1,86.55,184a56,56,0,0,1-22-106.86,15.9,15.9,0,0,0,8.05-8.33,60,60,0,0,1,110.7,0,15.9,15.9,0,0,0,8.05,8.33,56,56,0,0,1-22,106.86Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M196.42,66.23a4,4,0,0,1-2-2.09,72,72,0,0,0-132.82,0,4,4,0,0,1-2,2.09A68.23,68.23,0,0,0,20,127.81c-.1,36.66,29.62,67.24,66.25,68.17A67.74,67.74,0,0,0,124,185.67V232a4,4,0,0,0,8,0V185.67A67.47,67.47,0,0,0,168,196l1.76,0c36.62-.93,66.34-31.51,66.24-68.17A68.23,68.23,0,0,0,196.42,66.23ZM169.55,188A59.43,59.43,0,0,1,134.7,177.9a12.39,12.39,0,0,0-2.7-1.35V130.47l45.79-22.89a4,4,0,1,0-3.58-7.16L132,121.53V88a4,4,0,0,0-8,0v57.53L81.79,124.42a4,4,0,1,0-3.58,7.16L124,154.47v22.08a12.39,12.39,0,0,0-2.7,1.35A59.36,59.36,0,0,1,86.45,188,60,60,0,0,1,62.93,73.49a11.92,11.92,0,0,0,6-6.25,64,64,0,0,1,118.08,0,11.92,11.92,0,0,0,6,6.25A60,60,0,0,1,169.55,188Z"></path>
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
