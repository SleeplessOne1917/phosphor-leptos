//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Shrimp(
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
                <path d="M244,60a32,32,0,0,0-32-32H128a4,4,0,0,1-4-4,12,12,0,0,0-24,0,28,28,0,0,0,28,28h84a8,8,0,0,1,0,16H96.9C50.43,68,12.34,105.4,12,151.37A84,84,0,0,0,96,236h56a12,12,0,0,0,0-24H124V196h44a12,12,0,0,0,0-24H112a8,8,0,0,1,0-16h40a76.1,76.1,0,0,0,75.58-68.07A32,32,0,0,0,244,60ZM96.9,92H100v42.34a32.2,32.2,0,0,0-12.65,9.27l-44-20A61.19,61.19,0,0,1,96.9,92ZM36,151.54c0-1.61.11-3.21.25-4.79l43.87,19.94a30.18,30.18,0,0,0,.66,4.29L51.31,192A59.54,59.54,0,0,1,36,151.54ZM96,212a59.8,59.8,0,0,1-24.23-5.09l22.66-16.18a32.47,32.47,0,0,0,5.57,2.93V212Zm56-80H124V92h78.6A52.08,52.08,0,0,1,152,132Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M112,80h0v72a20,20,0,0,0,0,40v32H96A72,72,0,0,1,96,80Z" opacity="0.2"></path>
    <path d="M136,116a12,12,0,1,1,12,12A12,12,0,0,1,136,116Zm87.8-30.62A80.09,80.09,0,0,1,144,160H112a12,12,0,0,0,0,24h56a8,8,0,0,1,0,16H120v16h32a8,8,0,0,1,0,16H96A80,80,0,0,1,96,72H212a12,12,0,0,0,0-24H128a24,24,0,0,1-24-24,8,8,0,0,1,16,0,8,8,0,0,0,8,8h84a28,28,0,0,1,11.8,53.38Zm-173,111.91,33.22-23.73c0-.51,0-1,0-1.56a28,28,0,0,1,1-7.48L33,140.87a63.74,63.74,0,0,0,17.84,56.42Zm39-8.2L64.12,207.46A63.6,63.6,0,0,0,96,216h8V198.83A28.13,28.13,0,0,1,89.84,189.09ZM104,145.17V88H96a64.07,64.07,0,0,0-58.22,37.48l55.87,25.39A28,28,0,0,1,104,145.17ZM207.5,88H120v56h24A64.09,64.09,0,0,0,207.5,88Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M136,116a12,12,0,1,1,12,12A12,12,0,0,1,136,116ZM240,60a28,28,0,0,1-16.2,25.38A80.09,80.09,0,0,1,144,160H112a12,12,0,0,0,0,24h56a8,8,0,0,1,0,16H120v16h32a8,8,0,0,1,0,16H96A80,80,0,0,1,96,72H212a12,12,0,0,0,0-24H128a24,24,0,0,1-24-24,8,8,0,0,1,16,0,8,8,0,0,0,8,8h84A28,28,0,0,1,240,60ZM85.72,182.2a8,8,0,0,0-11.16-1.86l-15.36,11a8,8,0,0,0,9.3,13l15.36-11A8,8,0,0,0,85.72,182.2Zm-1.5-35.62L45.55,129a8,8,0,1,0-6.62,14.56L77.6,161.15a8,8,0,0,0,10.59-4A8,8,0,0,0,84.22,146.58ZM207.5,88H120v56h24A64.09,64.09,0,0,0,207.5,88Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M138,116a10,10,0,1,1,10,10A10,10,0,0,1,138,116Zm83.9-32A78.1,78.1,0,0,1,144,158H112a14,14,0,0,0,0,28h56a6,6,0,0,1,0,12H118v20h34a6,6,0,0,1,0,12H96A78,78,0,0,1,96,74H212a14,14,0,0,0,0-28H128a22,22,0,0,1-22-22,6,6,0,0,1,12,0,10,10,0,0,0,10,10h84a26,26,0,0,1,9.9,50ZM50.65,199.88l35.48-25.34A23.74,23.74,0,0,1,86,172a25.92,25.92,0,0,1,1.46-8.57L31.51,138a65.8,65.8,0,0,0,19.14,61.88ZM90.3,186.3,60.49,207.59A65.56,65.56,0,0,0,96,218h10V197.29A26.05,26.05,0,0,1,90.3,186.3ZM106,146.71V86H96a66.1,66.1,0,0,0-60.86,40.47L94,153.24A25.86,25.86,0,0,1,106,146.71ZM209.73,86H118v60h26A66.09,66.09,0,0,0,209.73,86Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M136,116a12,12,0,1,1,12,12A12,12,0,0,1,136,116Zm87.8-30.62A80.09,80.09,0,0,1,144,160H112a12,12,0,0,0,0,24h56a8,8,0,0,1,0,16H120v16h32a8,8,0,0,1,0,16H96A80,80,0,0,1,96,72H212a12,12,0,0,0,0-24H128a24,24,0,0,1-24-24,8,8,0,0,1,16,0,8,8,0,0,0,8,8h84a28,28,0,0,1,11.8,53.38Zm-173,111.91,33.22-23.73c0-.51,0-1,0-1.56a28,28,0,0,1,1-7.48L33,140.87a63.74,63.74,0,0,0,17.84,56.42Zm39-8.2L64.12,207.46A63.6,63.6,0,0,0,96,216h8V198.83A28.13,28.13,0,0,1,89.84,189.09ZM104,145.17V88H96a64.07,64.07,0,0,0-58.22,37.48l55.87,25.39A28,28,0,0,1,104,145.17ZM207.5,88H120v56h24A64.09,64.09,0,0,0,207.5,88Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M140,116a8,8,0,1,1,8,8A8,8,0,0,1,140,116Zm79.93-33.35A76.07,76.07,0,0,1,144,156H112a16,16,0,0,0,0,32h56a4,4,0,0,1,0,8H116v24h36a4,4,0,0,1,0,8H96A76,76,0,0,1,96,76H212a16,16,0,0,0,0-32H128a20,20,0,0,1-20-20,4,4,0,0,1,8,0,12,12,0,0,0,12,12h84a24,24,0,0,1,7.93,46.65ZM50.47,202.46l37.78-27A24.75,24.75,0,0,1,88,172a23.85,23.85,0,0,1,2-9.6l-59.9-27.23a68,68,0,0,0,20.36,67.29Zm40.43-19L57,207.64A67.59,67.59,0,0,0,96,220h12V195.66A24.07,24.07,0,0,1,90.9,183.42ZM108,148.34V84H96a68.1,68.1,0,0,0-63.42,43.51l61.89,28.13A23.94,23.94,0,0,1,108,148.34ZM211.88,84H116v64h28A68.08,68.08,0,0,0,211.88,84Z"></path>
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
