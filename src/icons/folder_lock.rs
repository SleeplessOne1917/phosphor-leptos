//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn FolderLock(
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
                <path d="M224,152h-4v-4a32,32,0,0,0-64,0v4h-4a12,12,0,0,0-12,12v44a12,12,0,0,0,12,12h72a12,12,0,0,0,12-12V164A12,12,0,0,0,224,152Zm-44-4a8,8,0,0,1,16,0v4H180Zm32,48H164V176h48Zm4-128H133.39l-26-29.29a20,20,0,0,0-15-6.71H40A20,20,0,0,0,20,52V200.62A19.41,19.41,0,0,0,39.38,220h65.18a12,12,0,0,0,0-24H44V92H212a12,12,0,0,0,24,0V88A20,20,0,0,0,216,68ZM44,68V56H90.61l10.67,12Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,168v40H152V168ZM92.69,48H40a8,8,0,0,0-8,8V80h96L98.34,50.34A8,8,0,0,0,92.69,48Z"
        opacity="0.2"
    ></path>
    <path d="M224,160h-8v-4a28,28,0,0,0-56,0v4h-8a8,8,0,0,0-8,8v40a8,8,0,0,0,8,8h72a8,8,0,0,0,8-8V168A8,8,0,0,0,224,160Zm-48-4a12,12,0,0,1,24,0v4H176Zm40,44H160V176h56Zm0-128H131.31L104,44.69A15.86,15.86,0,0,0,92.69,40H40A16,16,0,0,0,24,56V200.62A15.4,15.4,0,0,0,39.38,216h73.18a8,8,0,0,0,0-16H40V88H216v16a8,8,0,0,0,16,0V88A16,16,0,0,0,216,72ZM92.69,56l16,16H40V56Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M216,72H131.31L104,44.69A15.86,15.86,0,0,0,92.69,40H40A16,16,0,0,0,24,56V200.62A15.4,15.4,0,0,0,39.38,216h73.18a8,8,0,0,0,0-16H40V88H216v16a8,8,0,0,0,16,0V88A16,16,0,0,0,216,72ZM40,72V56H92.69l16,16Zm184,88h-8v-4a28,28,0,0,0-56,0v4h-8a8,8,0,0,0-8,8v40a8,8,0,0,0,8,8h72a8,8,0,0,0,8-8V168A8,8,0,0,0,224,160Zm-24,0H176v-4a12,12,0,0,1,24,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M224,162H214v-6a26,26,0,0,0-52,0v6H152a6,6,0,0,0-6,6v40a6,6,0,0,0,6,6h72a6,6,0,0,0,6-6V168A6,6,0,0,0,224,162Zm-50-6a14,14,0,0,1,28,0v6H174Zm44,46H158V174h60ZM216,74H130.49l-27.9-27.9a13.94,13.94,0,0,0-9.9-4.1H40A14,14,0,0,0,26,56V200.62A13.39,13.39,0,0,0,39.38,214h73.18a6,6,0,0,0,0-12H39.38A1.4,1.4,0,0,1,38,200.62V86H216a2,2,0,0,1,2,2v16a6,6,0,0,0,12,0V88A14,14,0,0,0,216,74ZM40,54H92.69a2,2,0,0,1,1.41.59L113.51,74H38V56A2,2,0,0,1,40,54Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,160h-8v-4a28,28,0,0,0-56,0v4h-8a8,8,0,0,0-8,8v40a8,8,0,0,0,8,8h72a8,8,0,0,0,8-8V168A8,8,0,0,0,224,160Zm-48-4a12,12,0,0,1,24,0v4H176Zm40,44H160V176h56Zm0-128H131.31L104,44.69A15.86,15.86,0,0,0,92.69,40H40A16,16,0,0,0,24,56V200.62A15.4,15.4,0,0,0,39.38,216h73.18a8,8,0,0,0,0-16H40V88H216v16a8,8,0,0,0,16,0V88A16,16,0,0,0,216,72ZM92.69,56l16,16H40V56Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M224,164H212v-8a24,24,0,0,0-48,0v8H152a4,4,0,0,0-4,4v40a4,4,0,0,0,4,4h72a4,4,0,0,0,4-4V168A4,4,0,0,0,224,164Zm-52-8a16,16,0,0,1,32,0v8H172Zm48,48H156V172h64ZM216,76H129.66L101.17,47.51A12,12,0,0,0,92.69,44H40A12,12,0,0,0,28,56V200.62A11.4,11.4,0,0,0,39.38,212h73.18a4,4,0,0,0,0-8H39.38A3.39,3.39,0,0,1,36,200.62V84H216a4,4,0,0,1,4,4v16a4,4,0,0,0,8,0V88A12,12,0,0,0,216,76ZM40,52H92.69a4,4,0,0,1,2.82,1.17L118.34,76H36V56A4,4,0,0,1,40,52Z"></path>
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
