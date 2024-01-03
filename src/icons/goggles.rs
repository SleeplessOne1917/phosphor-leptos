//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Goggles(
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
                <path d="M180,60H76A76.08,76.08,0,0,0,0,136v4a24,24,0,0,0,13.74,21.69c3.15,8.71,10.51,16.75,21.52,23.27,11.52,6.81,25.6,11,36.74,11a44.06,44.06,0,0,0,42.32-32h27.36A44.06,44.06,0,0,0,184,196c18.53,0,50.62-12.81,58.31-34.33A24,24,0,0,0,256,140v-4A76.08,76.08,0,0,0,180,60ZM76,84H180a52.07,52.07,0,0,1,51.13,42.6A65,65,0,0,0,220.74,119c-11.52-6.81-25.6-11-36.74-11a44.06,44.06,0,0,0-42.32,32H114.32A44.06,44.06,0,0,0,72,108c-13.4,0-33.9,6.71-47.13,18.56A52.08,52.08,0,0,1,76,84Zm-4,88c-14.13,0-36-12.15-36-20,0-2.74,3.55-7.61,11.48-12.3,9-5.32,18.8-7.7,24.52-7.7a20,20,0,0,1,0,40Zm136.52-7.7c-9,5.32-18.8,7.7-24.52,7.7a20,20,0,0,1,0-40c14.13,0,36,12.15,36,20C220,154.74,216.45,159.61,208.52,164.3Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,152c0,17.67-30.33,32-48,32a32,32,0,0,1,0-64C201.67,120,232,134.33,232,152ZM72,120c-17.67,0-48,14.33-48,32s30.33,32,48,32a32,32,0,0,0,0-64Z"
        opacity="0.2"
    ></path>
    <path d="M256,136a72.08,72.08,0,0,0-72-72H72A72.08,72.08,0,0,0,0,136a24.06,24.06,0,0,0,17,23c6.06,20.37,37.63,33,55,33a40.07,40.07,0,0,0,39.2-32h33.6A40.07,40.07,0,0,0,184,192c17.33,0,48.9-12.66,55-33A24.06,24.06,0,0,0,256,136ZM45.45,167.74C37,162.76,32,156.88,32,152s4.91-10.61,13.13-15.55l37.21,37.2A23.74,23.74,0,0,1,72,176C64.47,176,53.8,172.68,45.45,167.74Zm48.2-5.4L61,129.7A42.72,42.72,0,0,1,72,128a24,24,0,0,1,24,24A23.74,23.74,0,0,1,93.65,162.34ZM160,152a23.88,23.88,0,0,1,5.46-15.22L201,172.32c-6,2.3-12.15,3.68-17,3.68A24,24,0,0,1,160,152Zm55.63,12.31-35.92-35.92A24.19,24.19,0,0,1,184,128c7.53,0,18.2,3.32,26.55,8.26S224,147.12,224,152C224,155.79,221,160.2,215.63,164.31Zm22.05-22.69C229.34,123.25,200.34,112,184,112a40.07,40.07,0,0,0-39.2,32H111.2A40.07,40.07,0,0,0,72,112c-16.34,0-45.34,11.25-53.68,29.62A8,8,0,0,1,16,136,56.06,56.06,0,0,1,72,80H184a56.06,56.06,0,0,1,56,56A8,8,0,0,1,237.68,141.62Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M184,64H72A72.08,72.08,0,0,0,0,136a24.06,24.06,0,0,0,17,23c6.06,20.37,37.63,33,55,33a40.07,40.07,0,0,0,39.2-32h33.6A40.07,40.07,0,0,0,184,192c17.33,0,48.9-12.66,55-33a24.06,24.06,0,0,0,17-23A72.08,72.08,0,0,0,184,64ZM89,169a8,8,0,0,1-11.31,0L53.14,144.45a8,8,0,0,1,11.31-11.31L89,157.65A8,8,0,0,1,89,169Zm119.52-.49a8,8,0,0,1-11.31,0l-25.41-25.4a8,8,0,0,1,11.32-11.32l25.4,25.41A8,8,0,0,1,208.48,168.48Zm29.2-26.86C229.34,123.25,200.34,112,184,112a40.07,40.07,0,0,0-39.2,32H111.2A40.07,40.07,0,0,0,72,112c-16.34,0-45.34,11.25-53.68,29.62A8,8,0,0,1,16,136,56.06,56.06,0,0,1,72,80H184a56.06,56.06,0,0,1,56,56A8,8,0,0,1,237.68,141.62Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M254,136a70.08,70.08,0,0,0-70-70H72A70.08,70.08,0,0,0,2,136a22,22,0,0,0,16.68,21.35c2.06,8.24,8.85,16.06,19.64,22.44S61.87,190,72,190a38.05,38.05,0,0,0,37.52-32h37A38.05,38.05,0,0,0,184,190c10.13,0,23-3.91,33.68-10.21s17.58-14.2,19.64-22.44A22,22,0,0,0,254,136ZM44.43,169.46C35.26,164,30,157.67,30,152c0-6.64,6.77-13.19,15.45-18.06l40.19,40.18A25.81,25.81,0,0,1,72,178C64.14,178,53.06,174.57,44.43,169.46Zm49.69-3.82L57.24,128.75A48.84,48.84,0,0,1,72,126a26,26,0,0,1,22.12,39.64ZM158,152a25.89,25.89,0,0,1,7.39-18.13L204.55,173c-7.07,3.07-14.63,5-20.55,5A26,26,0,0,1,158,152Zm57.46,15-39.65-39.64A25.84,25.84,0,0,1,184,126c7.86,0,18.94,3.43,27.57,8.54C220.74,140,226,146.33,226,152,226,157.32,221.65,162.58,215.46,167Zm21.31-22.18c-2.57-7.56-9.12-14.68-19.09-20.58C207,117.91,194.13,114,184,114a38.05,38.05,0,0,0-37.52,32h-37A38.05,38.05,0,0,0,72,114c-10.13,0-23,3.91-33.68,10.21-10,5.9-16.52,13-19.09,20.58A10,10,0,0,1,14,136,58.07,58.07,0,0,1,72,78H184a58.07,58.07,0,0,1,58,58A10,10,0,0,1,236.77,144.79Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M256,136a72.08,72.08,0,0,0-72-72H72A72.08,72.08,0,0,0,0,136a24.06,24.06,0,0,0,17,23c6.06,20.37,37.63,33,55,33a40.07,40.07,0,0,0,39.2-32h33.6A40.07,40.07,0,0,0,184,192c17.33,0,48.9-12.66,55-33A24.06,24.06,0,0,0,256,136ZM45.45,167.74C37,162.76,32,156.88,32,152s4.91-10.61,13.13-15.55l37.21,37.2A23.74,23.74,0,0,1,72,176C64.47,176,53.8,172.68,45.45,167.74Zm48.2-5.4L61,129.7A42.66,42.66,0,0,1,72,128a24,24,0,0,1,24,24A23.74,23.74,0,0,1,93.65,162.34ZM160,152a23.88,23.88,0,0,1,5.46-15.22L201,172.32c-6,2.3-12.15,3.68-17,3.68A24,24,0,0,1,160,152Zm55.63,12.31-35.92-35.92A24.19,24.19,0,0,1,184,128c7.53,0,18.2,3.32,26.55,8.26S224,147.12,224,152C224,155.79,221,160.2,215.63,164.31Zm22.05-22.69C229.34,123.25,200.34,112,184,112a40.07,40.07,0,0,0-39.2,32H111.2A40.07,40.07,0,0,0,72,112c-16.34,0-45.34,11.25-53.68,29.62A8,8,0,0,1,16,136,56.06,56.06,0,0,1,72,80H184a56.06,56.06,0,0,1,56,56A8,8,0,0,1,237.68,141.62Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M252,136a68.07,68.07,0,0,0-68-68H72A68.07,68.07,0,0,0,4,136a20,20,0,0,0,16.35,19.65C23.91,174.84,54.81,188,72,188a36,36,0,0,0,35.77-32h40.46A36,36,0,0,0,184,188c17.19,0,48.09-13.16,51.65-32.35A20,20,0,0,0,252,136ZM28,152c0-7.75,7.92-15.25,17.81-20.53l43,42.95A27.89,27.89,0,0,1,72,180C55.45,180,28,166.44,28,152Zm66.42,16.76L53.58,127.92C60,125.46,66.58,124,72,124a28,28,0,0,1,22.42,44.76ZM156,152a27.94,27.94,0,0,1,9.42-20.92L208,173.65c-8,3.91-17,6.35-24,6.35A28,28,0,0,1,156,152Zm59.2,17.55-42.95-43A27.86,27.86,0,0,1,184,124c16.55,0,44,13.56,44,28C228,158.39,222.62,164.6,215.2,169.55Zm20.26-22.06C231.15,128.77,200.93,116,184,116a36,36,0,0,0-35.77,32H107.77A36,36,0,0,0,72,116c-16.93,0-47.15,12.77-51.46,31.49A12,12,0,0,1,12,136,60.07,60.07,0,0,1,72,76H184a60.07,60.07,0,0,1,60,60A12,12,0,0,1,235.46,147.49Z"></path>
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
