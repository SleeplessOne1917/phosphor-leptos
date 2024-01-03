//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn ProjectorScreen(
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
                <path d="M236,72V48a20,20,0,0,0-20-20H40A20,20,0,0,0,20,48V72A20,20,0,0,0,36,91.6V164H32a12,12,0,0,0,0,24h84v23.22a24,24,0,1,0,24,0V188h84a12,12,0,0,0,0-24h-4V91.6A20,20,0,0,0,236,72ZM44,52H212V68H44ZM60,164V92H196v72Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,72V184H48V72Z" opacity="0.2"></path>
    <path d="M232,64V48a16,16,0,0,0-16-16H40A16,16,0,0,0,24,48V64A16,16,0,0,0,40,80v96H32a8,8,0,0,0,0,16h88v17.38a24,24,0,1,0,16,0V192h88a8,8,0,0,0,0-16h-8V80A16,16,0,0,0,232,64ZM128,240a8,8,0,1,1,8-8A8,8,0,0,1,128,240ZM40,48H216V64H40ZM200,176H56V80H200Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M232,64V48a16,16,0,0,0-16-16H40A16,16,0,0,0,24,48V64A16,16,0,0,0,40,80v96H32a8,8,0,0,0,0,16h88v17.38a24,24,0,1,0,16,0V192h88a8,8,0,0,0,0-16h-8V80A16,16,0,0,0,232,64ZM128,240a8,8,0,1,1,8-8A8,8,0,0,1,128,240ZM40,48H216V64H40Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M216,78a14,14,0,0,0,14-14V48a14,14,0,0,0-14-14H40A14,14,0,0,0,26,48V64A14,14,0,0,0,40,78h2V178H32a6,6,0,0,0,0,12h90v20.84a22,22,0,1,0,12,0V190h90a6,6,0,0,0,0-12H214V78ZM138,232a10,10,0,1,1-10-10A10,10,0,0,1,138,232ZM38,64V48a2,2,0,0,1,2-2H216a2,2,0,0,1,2,2V64a2,2,0,0,1-2,2H40A2,2,0,0,1,38,64ZM202,178H54V78H202Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,64V48a16,16,0,0,0-16-16H40A16,16,0,0,0,24,48V64A16,16,0,0,0,40,80v96H32a8,8,0,0,0,0,16h88v17.38a24,24,0,1,0,16,0V192h88a8,8,0,0,0,0-16h-8V80A16,16,0,0,0,232,64ZM128,240a8,8,0,1,1,8-8A8,8,0,0,1,128,240ZM40,48H216V64H40ZM200,176H56V80H200Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M216,76a12,12,0,0,0,12-12V48a12,12,0,0,0-12-12H40A12,12,0,0,0,28,48V64A12,12,0,0,0,40,76h4V180H32a4,4,0,0,0,0,8h92v24.4a20,20,0,1,0,8,0V188h92a4,4,0,0,0,0-8H212V76ZM140,232a12,12,0,1,1-12-12A12,12,0,0,1,140,232ZM36,64V48a4,4,0,0,1,4-4H216a4,4,0,0,1,4,4V64a4,4,0,0,1-4,4H40A4,4,0,0,1,36,64ZM204,180H52V76H204Z"></path>
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
