//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Wheelchair(
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
                <path d="M255.14,187.54a12,12,0,0,0-15.6-6.68l-9.75,3.9-27.06-54.13A12,12,0,0,0,192,124H116V108h52a12,12,0,0,0,0-24H116V81.94a36,36,0,1,0-24,0v4.75a76,76,0,1,0,92.21,97.06,12,12,0,1,0-22.8-7.5A52,52,0,1,1,92,112v24a12,12,0,0,0,12,12h80.58l28.68,57.37a12,12,0,0,0,15.19,5.77l20-8A12,12,0,0,0,255.14,187.54ZM104,36A12,12,0,1,1,92,48,12,12,0,0,1,104,36Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M128,48a24,24,0,1,1-24-24A24,24,0,0,1,128,48Z" opacity="0.2"></path>
    <path d="M255.59,189.47a8,8,0,0,0-10.12-5.06l-17.42,5.81-28.9-57.8A8,8,0,0,0,192,128H112V104h56a8,8,0,0,0,0-16H112V79a32,32,0,1,0-16,0V89.81A72,72,0,0,0,112,232c33.52,0,63.69-22.71,71.75-54a8,8,0,1,0-15.5-4C162.09,198,137.91,216,112,216A56,56,0,0,1,96,106.34V136a8,8,0,0,0,8,8h83.05l29.79,59.58a8,8,0,0,0,9.69,4l24-8A8,8,0,0,0,255.59,189.47ZM88,48a16,16,0,1,1,16,16A16,16,0,0,1,88,48Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M250.53,199.59l-24,8a8,8,0,0,1-9.69-4L187.05,144H104a8,8,0,0,1-8-8V106.34A56,56,0,0,0,112,216c25.91,0,50.09-18.05,56.25-42a8,8,0,1,1,15.5,4c-8.06,31.3-38.23,54-71.75,54A72,72,0,0,1,96,89.81v-19a28,28,0,1,1,16,0V88h56a8,8,0,0,1,0,16H112v24h80a8,8,0,0,1,7.15,4.42l28.9,57.8,17.42-5.81a8,8,0,0,1,5.06,15.18Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M253.69,190.1a6,6,0,0,0-7.59-3.79L227,192.66l-29.68-59.34A6,6,0,0,0,192,130H110V102.05c.66,0,1.33,0,2,0h56a6,6,0,0,0,0-12H112c-.67,0-1.33,0-2,0V77.4a30,30,0,1,0-12,0v14A70,70,0,0,0,112,230c32.62,0,62-22.08,69.81-52.5a6,6,0,0,0-11.62-3c-6.49,25.21-31,43.5-58.19,43.5A58,58,0,0,1,98,103.72V136a6,6,0,0,0,6,6h84.29l30.34,60.68a6,6,0,0,0,7.27,3l24-8A6,6,0,0,0,253.69,190.1ZM86,48a18,18,0,1,1,18,18A18,18,0,0,1,86,48Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M255.59,189.47a8,8,0,0,0-10.12-5.06l-17.42,5.81-28.9-57.8A8,8,0,0,0,192,128H112V104h56a8,8,0,0,0,0-16H112V79a32,32,0,1,0-16,0V89.81A72,72,0,0,0,112,232c33.52,0,63.69-22.71,71.75-54a8,8,0,1,0-15.5-4C162.09,198,137.91,216,112,216A56,56,0,0,1,96,106.34V136a8,8,0,0,0,8,8h83.05l29.79,59.58a8,8,0,0,0,9.69,4l24-8A8,8,0,0,0,255.59,189.47ZM88,48a16,16,0,1,1,16,16A16,16,0,0,1,88,48Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M251.79,190.73a4,4,0,0,0-5.06-2.52l-20.7,6.9-30.45-60.9A4,4,0,0,0,192,132H108V100.15c1.32-.09,2.65-.15,4-.15h56a4,4,0,0,0,0-8H112c-1.34,0-2.68,0-4,.13V75.71a28,28,0,1,0-8,0V93.08A68,68,0,0,0,112,228c31.72,0,60.27-21.45,67.87-51a4,4,0,0,0-7.74-2c-6.71,26.08-32,45-60.13,45a60,60,0,0,1-12-118.79V136a4,4,0,0,0,4,4h85.53l30.89,61.79A4,4,0,0,0,224,204a3.92,3.92,0,0,0,1.26-.21l24-8A4,4,0,0,0,251.79,190.73ZM84,48a20,20,0,1,1,20,20A20,20,0,0,1,84,48Z"></path>
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
