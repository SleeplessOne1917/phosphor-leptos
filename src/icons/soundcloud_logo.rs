//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn SoundcloudLogo(
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
                <path d="M32,120v48a12,12,0,0,1-24,0V120a12,12,0,0,1,24,0ZM60,84A12,12,0,0,0,48,96v96a12,12,0,0,0,24,0V96A12,12,0,0,0,60,84Zm40-40A12,12,0,0,0,88,56V192a12,12,0,0,0,24,0V56A12,12,0,0,0,100,44Zm122.34,59.33A84,84,0,0,0,140,36a12,12,0,0,0,0,24,59.78,59.78,0,0,1,59.7,53.93,12,12,0,0,0,9.66,10.58A28,28,0,0,1,204,180H140a12,12,0,0,0,0,24h64a52,52,0,0,0,18.34-100.67Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M248,152a40,40,0,0,1-40,40H144V48a72,72,0,0,1,71.64,64.73A40,40,0,0,1,248,152Z"
        opacity="0.2"
    ></path>
    <path d="M24,120v48a8,8,0,0,1-16,0V120a8,8,0,0,1,16,0ZM48,88a8,8,0,0,0-8,8v96a8,8,0,0,0,16,0V96A8,8,0,0,0,48,88Zm32-8a8,8,0,0,0-8,8V192a8,8,0,0,0,16,0V88A8,8,0,0,0,80,80Zm32-32a8,8,0,0,0-8,8V192a8,8,0,0,0,16,0V56A8,8,0,0,0,112,48Zm110.84,58.34A80,80,0,0,0,144,40a8,8,0,0,0,0,16,63.76,63.76,0,0,1,63.68,57.53,8,8,0,0,0,6.44,7A32,32,0,0,1,208,184H144a8,8,0,0,0,0,16h64a48,48,0,0,0,14.84-93.66Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M24,120v48a8,8,0,0,1-16,0V120a8,8,0,0,1,16,0ZM48,88a8,8,0,0,0-8,8v96a8,8,0,0,0,16,0V96A8,8,0,0,0,48,88Zm32-8a8,8,0,0,0-8,8V192a8,8,0,0,0,16,0V88A8,8,0,0,0,80,80Zm32-32a8,8,0,0,0-8,8V192a8,8,0,0,0,16,0V56A8,8,0,0,0,112,48Zm110.84,58.34A80,80,0,0,0,144,40a8,8,0,0,0-8,8V192a8,8,0,0,0,8,8h64a48,48,0,0,0,14.84-93.66Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M22,120v48a6,6,0,0,1-12,0V120a6,6,0,0,1,12,0ZM48,90a6,6,0,0,0-6,6v96a6,6,0,0,0,12,0V96A6,6,0,0,0,48,90Zm32-8a6,6,0,0,0-6,6V192a6,6,0,0,0,12,0V88A6,6,0,0,0,80,82Zm32-32a6,6,0,0,0-6,6V192a6,6,0,0,0,12,0V56A6,6,0,0,0,112,50Zm109.06,57.88A78,78,0,0,0,144,42a6,6,0,0,0,0,12,65.75,65.75,0,0,1,65.67,59.33,6,6,0,0,0,4.83,5.29A34,34,0,0,1,208,186H144a6,6,0,0,0,0,12h64a46,46,0,0,0,13.06-90.12Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M24,120v48a8,8,0,0,1-16,0V120a8,8,0,0,1,16,0ZM48,88a8,8,0,0,0-8,8v96a8,8,0,0,0,16,0V96A8,8,0,0,0,48,88Zm32-8a8,8,0,0,0-8,8V192a8,8,0,0,0,16,0V88A8,8,0,0,0,80,80Zm32-32a8,8,0,0,0-8,8V192a8,8,0,0,0,16,0V56A8,8,0,0,0,112,48Zm110.84,58.34A80,80,0,0,0,144,40a8,8,0,0,0,0,16,63.76,63.76,0,0,1,63.68,57.53,8,8,0,0,0,6.44,7A32,32,0,0,1,208,184H144a8,8,0,0,0,0,16h64a48,48,0,0,0,14.84-93.66Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M20,120v48a4,4,0,0,1-8,0V120a4,4,0,0,1,8,0ZM48,92a4,4,0,0,0-4,4v96a4,4,0,0,0,8,0V96A4,4,0,0,0,48,92Zm32-8a4,4,0,0,0-4,4V192a4,4,0,0,0,8,0V88A4,4,0,0,0,80,84Zm32-32a4,4,0,0,0-4,4V192a4,4,0,0,0,8,0V56A4,4,0,0,0,112,52Zm107.27,57.46A76,76,0,0,0,144,44a4,4,0,0,0,0,8,67.75,67.75,0,0,1,67.66,61.13,4,4,0,0,0,3.22,3.53A36,36,0,0,1,208,188H144a4,4,0,0,0,0,8h64a44,44,0,0,0,11.27-86.54Z"></path>
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
