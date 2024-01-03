//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn CloudSun(
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
                <path d="M156,68a80.39,80.39,0,0,0-18.46,2.15,59.87,59.87,0,0,0-6-7.42l7.57-10.82a12,12,0,0,0-19.66-13.77L111.87,49A59.85,59.85,0,0,0,89.61,44l-2.3-13a12,12,0,0,0-23.63,4.17l2.3,13A60,60,0,0,0,46.77,60.47L35.91,52.86A12,12,0,0,0,22.14,72.52L33,80.11A59.45,59.45,0,0,0,28,102.36l-13,2.3a12,12,0,0,0,2.07,23.82,12.59,12.59,0,0,0,2.1-.18l13-2.3a59.29,59.29,0,0,0,3.44,7.25A56,56,0,0,0,76,228h80a80,80,0,0,0,0-160ZM88,68a36,36,0,0,1,26.45,11.61,80.37,80.37,0,0,0-32.06,36.75A56.5,56.5,0,0,0,76,116a55.84,55.84,0,0,0-20.33,3.83A36,36,0,0,1,88,68Zm68,136H76a32,32,0,0,1,0-64h.28c-.11,1.1-.2,2.2-.26,3.3a12,12,0,0,0,24,1.4,55.78,55.78,0,0,1,1.74-11l.15-.55A56.06,56.06,0,1,1,156,204Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M131.84,84.41v0a68.22,68.22,0,0,0-41.65,46v-.11a44.08,44.08,0,0,0-38.54,5h0a48,48,0,1,1,80.19-50.94Z"
        opacity="0.2"
    ></path>
    <path d="M156,72a76.2,76.2,0,0,0-20.26,2.73,55.63,55.63,0,0,0-9.41-11.54l9.51-13.57a8,8,0,1,0-13.11-9.18L113.22,54A55.9,55.9,0,0,0,88,48c-.58,0-1.16,0-1.74,0L83.37,31.71a8,8,0,1,0-15.75,2.77L70.5,50.82A56.1,56.1,0,0,0,47.23,65.67L33.61,56.14a8,8,0,1,0-9.17,13.11L38,78.77A55.55,55.55,0,0,0,32,104c0,.57,0,1.15,0,1.72L15.71,108.6a8,8,0,0,0,1.38,15.88,8.24,8.24,0,0,0,1.39-.12l16.32-2.88a55.74,55.74,0,0,0,5.86,12.42A52,52,0,0,0,76,224h80a76,76,0,0,0,0-152ZM48,104a40,40,0,0,1,72.54-23.24,76.26,76.26,0,0,0-35.62,40,52.14,52.14,0,0,0-31,4.17A40,40,0,0,1,48,104ZM156,208H76a36,36,0,1,1,4.78-71.69c-.37,2.37-.63,4.79-.77,7.23a8,8,0,0,0,16,.92,58.91,58.91,0,0,1,1.88-11.81c0-.16.09-.32.12-.48A60.06,60.06,0,1,1,156,208Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M156,72a76.2,76.2,0,0,0-20.26,2.73,55.63,55.63,0,0,0-9.41-11.54l9.51-13.57a8,8,0,1,0-13.11-9.18L113.22,54A55.9,55.9,0,0,0,88,48c-.59,0-1.16,0-1.74,0L83.37,31.71a8,8,0,1,0-15.75,2.77L70.5,50.82A56.1,56.1,0,0,0,47.23,65.67L33.61,56.14a8,8,0,1,0-9.17,13.11L38,78.77A55.55,55.55,0,0,0,32,104c0,.57,0,1.15,0,1.72L15.71,108.6a8,8,0,0,0,1.38,15.88,8.24,8.24,0,0,0,1.39-.12l16.32-2.88a55.74,55.74,0,0,0,5.86,12.42A52,52,0,0,0,76,224h80a76,76,0,0,0,0-152ZM84.92,120.76a52.14,52.14,0,0,0-31,4.17,40,40,0,0,1,66.62-44.17A76.26,76.26,0,0,0,84.92,120.76Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M156,74a74.15,74.15,0,0,0-21.18,3.09,54.08,54.08,0,0,0-11.14-13.61l10.52-15a6,6,0,1,0-9.83-6.89l-10.52,15A53.9,53.9,0,0,0,88,50c-1.15,0-2.28,0-3.41.12L81.4,32.05a6,6,0,1,0-11.81,2.09L72.77,52.2A54,54,0,0,0,47.52,68.32L32.47,57.78a6,6,0,0,0-6.89,9.83l15,10.52A53.7,53.7,0,0,0,34,104c0,1.13,0,2.26.12,3.39l-18.07,3.18a6,6,0,0,0,1,11.91,6.38,6.38,0,0,0,1.05-.09L36.2,119.2a53.51,53.51,0,0,0,7.08,15A50,50,0,0,0,76,222h80a74,74,0,0,0,0-148ZM46,104a42,42,0,0,1,77.48-22.49A74.29,74.29,0,0,0,86.2,123,50.36,50.36,0,0,0,76,122a49.65,49.65,0,0,0-22.79,5.52A42,42,0,0,1,46,104ZM156,210H76a38,38,0,1,1,7.08-75.34,75.84,75.84,0,0,0-1.07,9,6,6,0,0,0,12,.7,61.54,61.54,0,0,1,2-12.24c0-.15.08-.29.11-.43A62.06,62.06,0,1,1,156,210Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M156,72a76.2,76.2,0,0,0-20.26,2.73,55.63,55.63,0,0,0-9.41-11.54l9.51-13.57a8,8,0,1,0-13.11-9.18L113.22,54A55.9,55.9,0,0,0,88,48c-.58,0-1.16,0-1.74,0L83.37,31.71a8,8,0,1,0-15.75,2.77L70.5,50.82A56.1,56.1,0,0,0,47.23,65.67L33.61,56.14a8,8,0,1,0-9.17,13.11L38,78.77A55.55,55.55,0,0,0,32,104c0,.57,0,1.15,0,1.72L15.71,108.6a8,8,0,0,0,1.38,15.88,8.24,8.24,0,0,0,1.39-.12l16.32-2.88a55.74,55.74,0,0,0,5.86,12.42A52,52,0,0,0,76,224h80a76,76,0,0,0,0-152ZM48,104a40,40,0,0,1,72.54-23.24,76.26,76.26,0,0,0-35.62,40,52.14,52.14,0,0,0-31,4.17A40,40,0,0,1,48,104ZM156,208H76a36,36,0,1,1,4.78-71.69c-.37,2.37-.63,4.79-.77,7.23a8,8,0,0,0,16,.92,58.91,58.91,0,0,1,1.88-11.81c0-.16.09-.32.12-.48A60.06,60.06,0,1,1,156,208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M156,76a71.85,71.85,0,0,0-22.14,3.48A51.78,51.78,0,0,0,121,63.83l11.56-16.51A4,4,0,0,0,126,42.73L114.45,59.24A52,52,0,0,0,88,52c-1.71,0-3.4.09-5.06.25L79.44,32.4a4,4,0,0,0-7.88,1.39l3.5,19.84A52.19,52.19,0,0,0,47.85,71L31.32,59.42A4,4,0,1,0,26.73,66L43.26,77.54A51.63,51.63,0,0,0,36,104c0,1.69.09,3.37.25,5l-19.85,3.5a4,4,0,0,0,.69,7.94,4.23,4.23,0,0,0,.7-.06l19.85-3.5A52.07,52.07,0,0,0,46,134.6,48,48,0,0,0,76,220h80a72,72,0,0,0,0-144ZM44,104a44,44,0,0,1,82.33-21.61,72.23,72.23,0,0,0-38.82,43A48.28,48.28,0,0,0,76,124a47.76,47.76,0,0,0-23.4,6.11A44,44,0,0,1,44,104ZM156,212H76a40,40,0,1,1,9.43-78.88A71.63,71.63,0,0,0,84,143.77a4,4,0,0,0,8,.46,64.3,64.3,0,0,1,2-12.67c0-.12.07-.24.09-.36A64.06,64.06,0,1,1,156,212Z"></path>
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
