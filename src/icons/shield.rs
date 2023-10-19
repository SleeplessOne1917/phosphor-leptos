/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Shield(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M224,56v58.77c0,89.61-75.82,119.34-91,124.39a15.53,15.53,0,0,1-10,0c-15.2-5.05-91-34.78-91-124.39V56A16,16,0,0,1,48,40H208A16,16,0,0,1,224,56Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,56v58.77c0,84.18-71.31,112.07-85.54,116.8a7.54,7.54,0,0,1-4.92,0C111.31,226.86,40,199,40,114.79V56a8,8,0,0,1,8-8H208A8,8,0,0,1,216,56Z" opacity="0.2"/><path d="M208,40H48A16,16,0,0,0,32,56v58.77c0,89.61,75.82,119.34,91,124.39a15.53,15.53,0,0,0,10,0c15.2-5.05,91-34.78,91-124.39V56A16,16,0,0,0,208,40Zm0,74.79c0,78.42-66.35,104.62-80,109.18-13.53-4.51-80-30.69-80-109.18V56l160,0Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M208,44H48A12,12,0,0,0,36,56v58.77c0,86.87,73.54,115.7,88.28,120.6a11.65,11.65,0,0,0,7.44,0c14.74-4.9,88.28-33.73,88.28-120.6V56A12,12,0,0,0,208,44Zm4,70.79c0,81.38-69,108.41-82.8,113a3.53,3.53,0,0,1-2.4,0C113,223.2,44,196.17,44,114.79V56a4,4,0,0,1,4-4H208a4,4,0,0,1,4,4Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M208,36H48A20,20,0,0,0,28,56V114.8c0,92.36,78.1,123,93.76,128.18a19.6,19.6,0,0,0,12.48,0C149.9,237.78,228,207.16,228,114.8V56A20,20,0,0,0,208,36Zm-4,78.8c0,73.56-60.53,99.53-76,105-15.47-5.42-76-31.39-76-104.95V60H204Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M208,42H48A14,14,0,0,0,34,56v58.77c0,88.24,74.68,117.52,89.65,122.49a13.5,13.5,0,0,0,8.7,0c15-5,89.65-34.25,89.65-122.49V56A14,14,0,0,0,208,42Zm2,72.79c0,80-67.84,106.59-81.44,111.1a1.55,1.55,0,0,1-1.12,0C113.84,221.38,46,194.79,46,114.79V56a2,2,0,0,1,2-2H208a2,2,0,0,1,2,2Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,40H48A16,16,0,0,0,32,56v58.77c0,89.61,75.82,119.34,91,124.39a15.53,15.53,0,0,0,10,0c15.2-5.05,91-34.78,91-124.39V56A16,16,0,0,0,208,40Zm0,74.79c0,78.42-66.35,104.62-80,109.18-13.53-4.51-80-30.69-80-109.18V56l160,0Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg 
            xmlns="http://www.w3.org/2000/svg" 
            width=size.clone()
            height=size
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}