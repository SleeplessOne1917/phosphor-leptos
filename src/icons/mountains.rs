//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Mountains(
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
                <path d="M164,84a32,32,0,1,0-32-32A32,32,0,0,0,164,84Zm0-40a8,8,0,1,1-8,8A8,8,0,0,1,164,44Zm90.32,149.88L199.76,101.8A19.85,19.85,0,0,0,182.55,92h0a19.83,19.83,0,0,0-17.2,9.8l-18.7,31.55-37.42-63.5a20,20,0,0,0-34.46,0L1.66,193.91A12,12,0,0,0,12,212H244a12,12,0,0,0,10.32-18.12ZM92,87.87,108.57,116H75.43ZM33,188l28.28-48h61.44l13.58,23.06,0,.07L151,188Zm145.86,0L160.56,157l22-37.1L222.94,188Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M94.9,75.94,125.65,128H50.35L81.1,75.94A8,8,0,0,1,94.9,75.94ZM164,72a20,20,0,1,0-20-20A20,20,0,0,0,164,72Zm29.44,35.92a8,8,0,0,0-13.77,0l-33.06,55.79L168,200h80Z"
        opacity="0.2"
    ></path>
    <path d="M164,80a28,28,0,1,0-28-28A28,28,0,0,0,164,80Zm0-40a12,12,0,1,1-12,12A12,12,0,0,1,164,40Zm90.88,155.92-54.56-92.08A15.87,15.87,0,0,0,186.55,96h0a15.85,15.85,0,0,0-13.76,7.84L146.63,148l-44.84-76.1a16,16,0,0,0-27.58,0L1.11,195.94A8,8,0,0,0,8,208H248a8,8,0,0,0,6.88-12.08ZM88,80l23.57,40H64.43ZM22,192l33-56h66l18.74,31.8,0,0L154,192Zm150.57,0-16.66-28.28L186.55,112,234,192Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M136,52a28,28,0,1,1,28,28A28,28,0,0,1,136,52ZM255,204a8,8,0,0,1-6.95,4H8a8,8,0,0,1-6.89-12.06L74.21,71.88a16,16,0,0,1,27.58,0L146.63,148l26.16-44.14A15.85,15.85,0,0,1,186.55,96h0a15.87,15.87,0,0,1,13.77,7.84l54.56,92.08A8,8,0,0,1,255,204ZM64.43,120h47.14L88,80Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M164,78a26,26,0,1,0-26-26A26,26,0,0,0,164,78Zm0-40a14,14,0,1,1-14,14A14,14,0,0,1,164,38Zm89.16,158.94L198.6,104.86a13.9,13.9,0,0,0-12-6.86h0a13.88,13.88,0,0,0-12,6.86l-27.88,47.05-46.56-79a14,14,0,0,0-24.13,0L2.83,197A6,6,0,0,0,8,206H248a6,6,0,0,0,5.16-9.06ZM86.27,79a2,2,0,0,1,3.46,0l25.34,43H60.93ZM18.5,194l35.36-60h68.29l19.3,32.77,0,0,16,27.2Zm152.93,0-17.85-30.29L184.83,111a2,2,0,0,1,1.72-1,1.93,1.93,0,0,1,1.72,1l49.2,83Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M164,80a28,28,0,1,0-28-28A28,28,0,0,0,164,80Zm0-40a12,12,0,1,1-12,12A12,12,0,0,1,164,40Zm90.88,155.92-54.56-92.08A15.87,15.87,0,0,0,186.55,96h0a15.85,15.85,0,0,0-13.76,7.84L146.63,148l-44.84-76.1a16,16,0,0,0-27.58,0L1.11,195.94A8,8,0,0,0,8,208H248a8,8,0,0,0,6.88-12.08ZM88,80l23.57,40H64.43ZM22,192l33-56h66l18.74,31.8,0,0L154,192Zm150.57,0-16.66-28.28L186.55,112,234,192Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M164,76a24,24,0,1,0-24-24A24,24,0,0,0,164,76Zm0-40a16,16,0,1,1-16,16A16,16,0,0,1,164,36Zm87.44,162-54.56-92.08A11.91,11.91,0,0,0,186.55,100h0a11.89,11.89,0,0,0-10.32,5.88l-29.61,50L98.34,73.91a12,12,0,0,0-20.68,0L4.55,198A4,4,0,0,0,8,204H248a4,4,0,0,0,3.44-6ZM84.55,78a4,4,0,0,1,6.9,0l27.12,46H57.43ZM15,196l37.71-64h70.58L161,196Zm155.29,0-19-32.29L183.11,110a4,4,0,0,1,6.88,0l51,86Z"></path>
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
