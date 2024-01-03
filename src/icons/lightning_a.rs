//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn LightningA(
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
                <path d="M179.76,110.75a12,12,0,0,0-6.85-8.56L126,81.12l12.9-54.35a12,12,0,0,0-20.48-10.92L23.19,118.7a12,12,0,0,0,3.89,19.1l47,21.08L61.12,213.22a12,12,0,0,0,20.49,10.93L176.8,121.29A12,12,0,0,0,179.76,110.75ZM95,174.31l4.64-19.54a12,12,0,0,0-6.76-13.72l-40.76-18.3L105,65.69l-4.64,19.54A12,12,0,0,0,107.08,99l40.77,18.3Zm147.7,36.32-36-72a12,12,0,0,0-21.47,0l-36,72a12,12,0,1,0,21.46,10.73L177.41,208h37.17l6.68,13.36a12,12,0,1,0,21.47-10.73ZM189.41,184,196,170.83,202.58,184Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M72.8,216,88,152,32,126.86,127.2,24,112,88l56,25.14Z" opacity="0.2"></path>
    <path d="M175.84,111.54a8,8,0,0,0-4.56-5.7l-50-22.43L135,25.85a8,8,0,0,0-13.65-7.28L26.13,121.42a8,8,0,0,0,2.59,12.73l50,22.44L65,214.15a8,8,0,0,0,13.65,7.28l95.2-102.85A8,8,0,0,0,175.84,111.54ZM87.62,188.21l8.16-34.36a8,8,0,0,0-4.5-9.15L45.43,124.12l66.95-72.33-8.16,34.36a8,8,0,0,0,4.5,9.15l45.84,20.58Zm151.53,24.21-36-72a8,8,0,0,0-14.31,0l-36,72a8,8,0,0,0,14.31,7.16L176.94,200h38.11l9.79,19.58A8,8,0,0,0,232,224a8,8,0,0,0,7.15-11.58ZM184.94,184,196,161.89,207.05,184Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M173.87,118.58,78.67,221.43A8,8,0,0,1,65,214.15l13.67-57.56-50-22.44a8,8,0,0,1-2.59-12.73l95.2-102.85A8,8,0,0,1,135,25.85L121.31,83.41l50,22.43a8,8,0,0,1,2.59,12.74Zm61.71,104.57A7.91,7.91,0,0,1,232,224a8,8,0,0,1-7.16-4.42L215.05,200H176.94l-9.79,19.58a8,8,0,0,1-14.31-7.16l36-72a8,8,0,0,1,14.31,0l36,72A8,8,0,0,1,235.58,223.15ZM207.05,184,196,161.89,184.94,184Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M173.88,111.94a6,6,0,0,0-3.42-4.27L119,84.56,133,25.39a6,6,0,0,0-10.24-5.47L27.6,122.78a6,6,0,0,0,1.94,9.55L81,155.44,67,214.61a6,6,0,0,0,3,6.68,6,6,0,0,0,7.22-1.22l95.2-102.85A6,6,0,0,0,173.88,111.94Zm-90,83.21,9.92-41.76a6,6,0,0,0-3.38-6.86L42.08,124.8l74-80-9.92,41.77a6,6,0,0,0,3.38,6.86l48.38,21.73Zm153.44,18.16-36-72a6,6,0,0,0-10.74,0l-36,72a6,6,0,0,0,10.74,5.37L175.71,198h40.58l10.34,20.68A6,6,0,0,0,232,222a5.87,5.87,0,0,0,2.68-.64A6,6,0,0,0,237.36,213.31ZM181.71,186,196,157.42,210.29,186Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M175.84,111.54a8,8,0,0,0-4.56-5.7l-50-22.43L135,25.85a8,8,0,0,0-13.65-7.28L26.13,121.42a8,8,0,0,0,2.59,12.73l50,22.44L65,214.15a8,8,0,0,0,13.65,7.28l95.2-102.85A8,8,0,0,0,175.84,111.54ZM87.62,188.21l8.16-34.36a8,8,0,0,0-4.5-9.15L45.43,124.12l66.95-72.33-8.16,34.36a8,8,0,0,0,4.5,9.15l45.84,20.58Zm151.53,24.21-36-72a8,8,0,0,0-14.31,0l-36,72a8,8,0,0,0,14.31,7.16L176.94,200h38.11l9.79,19.58A8,8,0,0,0,232,224a8,8,0,0,0,7.15-11.58ZM184.94,184,196,161.89,207.05,184Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M171.92,112.34a4,4,0,0,0-2.28-2.85l-53-23.78,14.43-60.79a4,4,0,0,0-6.83-3.64L29.06,124.14a4,4,0,0,0,1.3,6.37l53,23.78L68.91,215.07a4,4,0,0,0,2,4.46,3.94,3.94,0,0,0,1.88.47,4,4,0,0,0,2.94-1.28l95.2-102.86A4,4,0,0,0,171.92,112.34ZM80.21,202.1l11.68-49.18a4,4,0,0,0-2.25-4.57L38.72,125.49,119.79,37.9,108.11,87.08a4,4,0,0,0,2.25,4.57l50.92,22.86Zm155.37,12.11-36-72a4,4,0,0,0-7.16,0l-36,72a4,4,0,1,0,7.16,3.58L174.47,196h43.06l10.89,21.79A4,4,0,0,0,232,220a4.12,4.12,0,0,0,1.79-.42A4,4,0,0,0,235.58,214.21ZM178.47,188,196,152.94,213.53,188Z"></path>
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
