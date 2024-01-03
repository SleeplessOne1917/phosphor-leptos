//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn ScribbleLoop(
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
                <path d="M248.9,152c-1.33-1.47-23.75-25.85-60.21-40.69-2.15-18.28-9.1-34.9-20.1-47.71C153.12,45.52,130.79,36,104,36,50.44,36,15.69,83.79,14.24,85.82a12,12,0,0,0,19.53,14C34.05,99.38,62.65,60,104,60c19.85,0,35.45,6.45,46.38,19.18a61.35,61.35,0,0,1,12.4,24A143.6,143.6,0,0,0,132.61,100c-27,0-49.79,7.13-65.85,20.63C52.3,132.79,44,149.78,44,167.25,44,193.46,63.44,220,100.61,220c51.93,0,82.34-40.28,87.87-82.43a156.46,156.46,0,0,1,42.62,30.48A12,12,0,1,0,248.9,152ZM148.37,173.74C140.09,183.9,125.09,196,100.61,196,78.08,196,68,181.56,68,167.25,68,146.42,88.22,124,132.61,124a119.85,119.85,0,0,1,32.64,4.62C164.24,145.44,158.21,161.66,148.37,173.74Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M100.6,208c-64,0-64-96,32-96a134.44,134.44,0,0,1,44.73,7.83h0C179,162.36,151.88,208,100.6,208Z"
        opacity="0.2"
    ></path>
    <path d="M245.93,154.63c-1.32-1.46-24.09-26.22-61-40.56-1.72-18.42-8.46-35.17-19.41-47.92C150.87,49,129.58,40,104,40,52.48,40,18.89,86.18,17.49,88.15a8,8,0,0,0,13,9.31C30.8,97.05,60.81,56,104,56c20.77,0,37.86,7.11,49.41,20.57,7.42,8.64,12.44,19.69,14.67,32A140.87,140.87,0,0,0,132.6,104c-26.06,0-47.93,6.81-63.26,19.69C55.78,135.09,48,151,48,167.25A47.59,47.59,0,0,0,61.87,201.3c9.66,9.62,23.06,14.7,38.73,14.7,51.81,0,81.18-42.13,84.49-84.42a161.43,161.43,0,0,1,49,33.79,8,8,0,1,0,11.86-10.74Zm-94.46,21.64C142.64,187.09,126.66,200,100.6,200,75.32,200,64,183.55,64,167.25,64,144.49,85.47,120,132.6,120a124.34,124.34,0,0,1,36.78,5.68C168.93,144.44,162.46,162.78,151.47,176.27Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M128,128a81.4,81.4,0,0,1,25.69,4.28C151.56,154.87,137.33,176,112,176c-15.8,0-24.06-10.85-24.06-21.58,0-6.59,3-12.75,8.56-17.35C103.62,131.14,114.52,128,128,128Zm96-80V208a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32H208A16,16,0,0,1,224,48Zm-18.11,98.59a120.21,120.21,0,0,0-36.08-25.21c-.9-14.35-5.75-27.54-13.89-37.55C145.38,70.86,130.19,64,112,64,76.44,64,50.68,97.76,49.6,99.2a8,8,0,0,0,12.79,9.62C62.61,108.53,84.51,80,112,80c13.4,0,24,4.68,31.5,13.92a47.54,47.54,0,0,1,9.48,21.4A96.75,96.75,0,0,0,128,112c-17.27,0-31.71,4.42-41.74,12.78C77,132.47,71.94,143,71.94,154.42,71.94,172.64,86,192,112,192a54,54,0,0,0,43.53-21.23A70,70,0,0,0,169,138.89a106.24,106.24,0,0,1,25.13,18.52,8,8,0,1,0,11.78-10.82Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M244.45,156c-1.32-1.46-24.27-26.42-61.37-40.5-1.5-18.49-8.13-35.31-19-48C149.74,50.8,129,42,104,42,53.49,42,20.5,87.38,19.12,89.31a6,6,0,1,0,9.76,7C29.18,95.87,59.75,54,104,54c21.37,0,39,7.35,50.93,21.27,8.26,9.62,13.64,22.14,15.62,36.06a139,139,0,0,0-38-5.33c-25.58,0-47,6.65-62,19.22-13.1,11-20.62,26.34-20.62,42a45.65,45.65,0,0,0,13.28,32.64C72.56,209.12,85.47,214,100.6,214c51.73,0,80.55-43.09,82.68-85.38,32.05,13.49,52,35.09,52.27,35.4a6,6,0,0,0,8.9-8ZM153,177.53C143.92,188.69,127.44,202,100.6,202,82,202,62,191.12,62,167.25,62,143.53,84.09,118,132.6,118a126.74,126.74,0,0,1,38.8,6.22C171.26,143.94,164.58,163.34,153,177.53Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M245.93,154.63c-1.32-1.46-24.09-26.22-61-40.56-1.72-18.42-8.46-35.17-19.41-47.92C150.87,49,129.58,40,104,40,52.48,40,18.89,86.18,17.49,88.15a8,8,0,0,0,13,9.31C30.8,97.05,60.81,56,104,56c20.77,0,37.86,7.11,49.41,20.57,7.42,8.64,12.44,19.69,14.67,32A140.87,140.87,0,0,0,132.6,104c-26.06,0-47.93,6.81-63.26,19.69C55.78,135.09,48,151,48,167.25A47.59,47.59,0,0,0,61.87,201.3c9.66,9.62,23.06,14.7,38.73,14.7,51.81,0,81.18-42.13,84.49-84.42a161.43,161.43,0,0,1,49,33.79,8,8,0,1,0,11.86-10.74Zm-94.46,21.64C142.64,187.09,126.66,200,100.6,200,75.32,200,64,183.55,64,167.25,64,144.49,85.47,120,132.6,120a124.34,124.34,0,0,1,36.78,5.68C168.93,144.44,162.46,162.78,151.47,176.27Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M243,157.32c-1.32-1.46-24.47-26.63-61.79-40.43-1.26-18.56-7.78-35.45-18.66-48.13C148.62,52.56,128.38,44,104,44,54.51,44,22.1,88.58,20.74,90.48a4,4,0,0,0,6.51,4.65C27.56,94.7,58.68,52,104,52c22,0,40.11,7.6,52.45,22,9.11,10.61,14.81,24.62,16.46,40.13A137.84,137.84,0,0,0,132.6,108c-25.1,0-46.09,6.48-60.69,18.75C59.26,137.39,52,152.15,52,167.25a43.64,43.64,0,0,0,12.69,31.22C73.59,207.32,86,212,100.6,212c51.63,0,79.87-44.08,80.78-86.32,34.07,13.58,55.36,36.67,55.65,37a4,4,0,1,0,5.94-5.36Zm-88.4,21.47c-9.37,11.5-26.34,25.21-54,25.21C72.71,204,60,185,60,167.25,60,142.57,82.72,116,132.6,116a129.23,129.23,0,0,1,40.8,6.77v.81C173.4,144,166.54,164.1,154.57,178.79Z"></path>
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
