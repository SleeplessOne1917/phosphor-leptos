//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Hoodie(
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
                <path d="M246,121.34,189.69,36.91A20,20,0,0,0,173.05,28H83a20,20,0,0,0-16.64,8.91L10,121.34a12,12,0,0,0-1.73,9.28l18.39,82.11c0,.18.09.36.14.53A20,20,0,0,0,46.11,228H76a20,20,0,0,0,20-20v-8h64v8a20,20,0,0,0,20,20h29.89a20,20,0,0,0,19.29-14.74c0-.17.1-.35.14-.53l18.39-82.11A12,12,0,0,0,246,121.34ZM128,74.4,86,52h84ZM80,176V76l16,8.53V136a12,12,0,0,0,24,0V97.33l2.35,1.26a12,12,0,0,0,11.3,0L136,97.33V128a12,12,0,0,0,24,0V84.53L176,76V176ZM49.32,204,32.83,130.39,56,95.63V180a20,20,0,0,0,16,19.6V204Zm157.36,0H184v-4.4A20,20,0,0,0,200,180V95.63l23.17,34.76Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,128l-22.39,82.1a8,8,0,0,1-7.72,5.9H176a8,8,0,0,1-8-8V184H88v24a8,8,0,0,1-8,8H54.11a8,8,0,0,1-7.72-5.9L24,128,72,56l56,32,56-32Z"
        opacity="0.2"
    ></path>
    <path d="M238.66,123.56l-56.3-84.44A16,16,0,0,0,169.05,32H87a16,16,0,0,0-13.31,7.12l-56.3,84.44a8,8,0,0,0-1.06,6.54l22.39,82.11A16.05,16.05,0,0,0,54.11,224H80a16,16,0,0,0,16-16V192h64v16a16,16,0,0,0,16,16h25.89a16.05,16.05,0,0,0,15.44-11.79l22.39-82.11A8,8,0,0,0,238.66,123.56ZM80,176V69.79L104,83.5V136a8,8,0,0,0,16,0V92.64L124,95A8,8,0,0,0,132,95l4-2.31V128a8,8,0,0,0,16,0V83.5l24-13.71V176ZM169.05,48l3.54,5.31L128,78.79,83.41,53.31,87,48ZM80,208H54.11L32.68,129.41,64,82.42V176a16,16,0,0,0,16,16Zm121.89,0H176V192a16,16,0,0,0,16-16V82.42l31.32,47Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M238.66,123.56l-56.3-84.44A16,16,0,0,0,169.05,32H87a16,16,0,0,0-13.31,7.12l-56.3,84.44a8,8,0,0,0-1.06,6.54l22.39,82.11A16,16,0,0,0,54.11,224H80a16,16,0,0,0,16-16V192h64v16a16,16,0,0,0,16,16h25.89a16,16,0,0,0,15.44-11.79l22.39-82.11A8,8,0,0,0,238.66,123.56ZM80,208H54.11L32.68,129.41,64,82.42V176a16,16,0,0,0,16,16Zm40-72a8,8,0,0,1-16,0V94.86a8,8,0,0,1,16,0Zm32-8a8,8,0,0,1-16,0V94.86a8,8,0,0,1,16,0ZM128,78.79,83.41,53.31,87,48h82.1l3.54,5.31ZM201.89,208H176V192a16,16,0,0,0,16-16V82.42l31.32,47Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M237,124.67,180.7,40.23A14,14,0,0,0,169.05,34H87A14,14,0,0,0,75.3,40.23L19,124.67a6,6,0,0,0-.8,4.91l22.39,82.1A14,14,0,0,0,54.11,222H80a14,14,0,0,0,14-14V190h68v18a14,14,0,0,0,14,14h25.89a14,14,0,0,0,13.51-10.32l22.39-82.1A6,6,0,0,0,237,124.67ZM80,178a2,2,0,0,1-2-2V66.34l28,16V136a6,6,0,0,0,12,0V89.2l7,4a6,6,0,0,0,6,0l7-4V128a6,6,0,0,0,12,0V82.34l28-16V176a2,2,0,0,1-2,2ZM87,46h82.1a2,2,0,0,1,1.67.89L175.44,54,128,81.09,80.56,54l4.72-7.09A2,2,0,0,1,87,46ZM82,208a2,2,0,0,1-2,2H54.11a2,2,0,0,1-1.93-1.47L30.51,129.06,66,75.82V176a14,14,0,0,0,14,14h2Zm121.82.53a2,2,0,0,1-1.93,1.47H176a2,2,0,0,1-2-2V190h2a14,14,0,0,0,14-14V75.82l35.49,53.24Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M238.66,123.56l-56.3-84.44A16,16,0,0,0,169.05,32H87a16,16,0,0,0-13.31,7.12l-56.3,84.44a8,8,0,0,0-1.06,6.54l22.39,82.11A16.05,16.05,0,0,0,54.11,224H80a16,16,0,0,0,16-16V192h64v16a16,16,0,0,0,16,16h25.89a16.05,16.05,0,0,0,15.44-11.79l22.39-82.11A8,8,0,0,0,238.66,123.56ZM80,176V69.79L104,83.5V136a8,8,0,0,0,16,0V92.64L124,95A8,8,0,0,0,132,95l4-2.31V128a8,8,0,0,0,16,0V83.5l24-13.71V176ZM169.05,48l3.54,5.31L128,78.79,83.41,53.31,87,48ZM80,208H54.11L32.68,129.41,64,82.42V176a16,16,0,0,0,16,16Zm121.89,0H176V192a16,16,0,0,0,16-16V82.42l31.32,47Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M235.33,125.78,179,41.34a12,12,0,0,0-10-5.34H87a12,12,0,0,0-10,5.34L20.67,125.78a4,4,0,0,0-.53,3.27l22.39,82.11A12,12,0,0,0,54.11,220H80a12,12,0,0,0,12-12V188h72v20a12,12,0,0,0,12,12h25.89a12,12,0,0,0,11.58-8.84l22.39-82.11A4,4,0,0,0,235.33,125.78ZM80,180a4,4,0,0,1-4-4V62.89l32,18.29V136a4,4,0,0,0,8,0V85.75l10,5.72a4,4,0,0,0,4,0l10-5.72V128a4,4,0,0,0,8,0V81.18l32-18.29V176a4,4,0,0,1-4,4ZM83.62,45.78A4,4,0,0,1,87,44h82.1a4,4,0,0,1,3.33,1.78l5.91,8.87L128,83.39,77.71,54.65ZM84,208a4,4,0,0,1-4,4H54.11a4,4,0,0,1-3.86-2.95L28.34,128.7,68,69.21V176a12,12,0,0,0,12,12h4Zm121.75,1.05a4,4,0,0,1-3.86,2.95H176a4,4,0,0,1-4-4V188h4a12,12,0,0,0,12-12V69.21l39.66,59.49Z"></path>
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
