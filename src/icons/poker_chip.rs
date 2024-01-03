//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn PokerChip(
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
                <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,144a36,36,0,1,1,36-36A36,36,0,0,1,128,164Zm33.06-86A59.51,59.51,0,0,0,140,69.21V44.87a83.55,83.55,0,0,1,38.28,15.88ZM116,69.21A59.51,59.51,0,0,0,94.94,78L77.72,60.75A83.55,83.55,0,0,1,116,44.87ZM78,94.94A59.51,59.51,0,0,0,69.21,116H44.87A83.59,83.59,0,0,1,60.75,77.72ZM69.21,140A59.51,59.51,0,0,0,78,161.06L60.75,178.28A83.59,83.59,0,0,1,44.87,140Zm25.73,38A59.51,59.51,0,0,0,116,186.79v24.34a83.55,83.55,0,0,1-38.28-15.88ZM140,186.79A59.51,59.51,0,0,0,161.06,178l17.22,17.22A83.55,83.55,0,0,1,140,211.13Zm38-25.73A59.51,59.51,0,0,0,186.79,140h24.34a83.59,83.59,0,0,1-15.88,38.28ZM186.79,116A59.51,59.51,0,0,0,178,94.94l17.22-17.22A83.59,83.59,0,0,1,211.13,116Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M128,32a96,96,0,1,0,96,96A96,96,0,0,0,128,32Zm0,152a56,56,0,1,1,56-56A56,56,0,0,1,128,184Z"
        opacity="0.2"
    ></path>
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,152a48,48,0,1,1,48-48A48.05,48.05,0,0,1,128,176Zm39.21-98.53a63.66,63.66,0,0,0-31.21-13V40.37a87.6,87.6,0,0,1,48.28,20ZM120,64.52a63.66,63.66,0,0,0-31.21,13L71.72,60.4a87.6,87.6,0,0,1,48.28-20ZM77.47,88.79a63.66,63.66,0,0,0-13,31.21H40.37a87.6,87.6,0,0,1,20-48.28ZM64.52,136a63.66,63.66,0,0,0,13,31.21L60.4,184.28a87.6,87.6,0,0,1-20-48.28Zm24.27,42.53A63.66,63.66,0,0,0,120,191.48v24.15a87.6,87.6,0,0,1-48.28-20ZM136,191.48a63.66,63.66,0,0,0,31.21-12.95l17.07,17.07a87.6,87.6,0,0,1-48.28,20Zm42.53-24.27A63.66,63.66,0,0,0,191.48,136h24.15a87.6,87.6,0,0,1-20,48.28ZM191.48,120a63.66,63.66,0,0,0-12.95-31.21L195.6,71.72a87.6,87.6,0,0,1,20,48.28Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24ZM60.4,71.72,77.47,88.79a63.66,63.66,0,0,0-13,31.21H40.37A87.6,87.6,0,0,1,60.4,71.72ZM40.37,136H64.52a63.66,63.66,0,0,0,13,31.21L60.4,184.28A87.6,87.6,0,0,1,40.37,136ZM120,215.63a87.6,87.6,0,0,1-48.28-20l17.07-17.07A63.66,63.66,0,0,0,120,191.48Zm0-151.11a63.66,63.66,0,0,0-31.21,13L71.72,60.4a87.6,87.6,0,0,1,48.28-20ZM215.63,120H191.48a63.66,63.66,0,0,0-12.95-31.21L195.6,71.72A87.6,87.6,0,0,1,215.63,120ZM136,40.37a87.6,87.6,0,0,1,48.28,20L167.21,77.47a63.66,63.66,0,0,0-31.21-13Zm0,175.26V191.48a63.66,63.66,0,0,0,31.21-12.95l17.07,17.07A87.6,87.6,0,0,1,136,215.63Zm59.6-31.35-17.07-17.07A63.66,63.66,0,0,0,191.48,136h24.15A87.6,87.6,0,0,1,195.6,184.28Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,152a50,50,0,1,1,50-50A50.06,50.06,0,0,1,128,178Zm39.37-97.86A61.68,61.68,0,0,0,134,66.3V38.2a89.64,89.64,0,0,1,53.22,22.09ZM122,66.3A61.68,61.68,0,0,0,88.63,80.14L68.78,60.29A89.64,89.64,0,0,1,122,38.2ZM80.14,88.63A61.68,61.68,0,0,0,66.3,122H38.2A89.61,89.61,0,0,1,60.29,68.78ZM66.3,134a61.68,61.68,0,0,0,13.84,33.37L60.29,187.22A89.61,89.61,0,0,1,38.2,134Zm22.33,41.86A61.68,61.68,0,0,0,122,189.7v28.1a89.64,89.64,0,0,1-53.22-22.09ZM134,189.7a61.68,61.68,0,0,0,33.37-13.84l19.85,19.85A89.64,89.64,0,0,1,134,217.8Zm41.86-22.33A61.68,61.68,0,0,0,189.7,134h28.1a89.61,89.61,0,0,1-22.09,53.22ZM189.7,122a61.68,61.68,0,0,0-13.84-33.37l19.85-19.85A89.61,89.61,0,0,1,217.8,122Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,152a48,48,0,1,1,48-48A48.05,48.05,0,0,1,128,176Zm39.21-98.53a63.66,63.66,0,0,0-31.21-13V40.37a87.6,87.6,0,0,1,48.28,20ZM120,64.52a63.66,63.66,0,0,0-31.21,13L71.72,60.4a87.6,87.6,0,0,1,48.28-20ZM77.47,88.79a63.66,63.66,0,0,0-13,31.21H40.37a87.6,87.6,0,0,1,20-48.28ZM64.52,136a63.66,63.66,0,0,0,13,31.21L60.4,184.28a87.6,87.6,0,0,1-20-48.28Zm24.27,42.53A63.66,63.66,0,0,0,120,191.48v24.15a87.6,87.6,0,0,1-48.28-20ZM136,191.48a63.66,63.66,0,0,0,31.21-12.95l17.07,17.07a87.6,87.6,0,0,1-48.28,20Zm42.53-24.27A63.66,63.66,0,0,0,191.48,136h24.15a87.6,87.6,0,0,1-20,48.28ZM191.48,120a63.66,63.66,0,0,0-12.95-31.21L195.6,71.72a87.6,87.6,0,0,1,20,48.28Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,152a52,52,0,1,1,52-52A52.06,52.06,0,0,1,128,180Zm39.47-97.13A59.7,59.7,0,0,0,132,68.15V36.09a91.64,91.64,0,0,1,58.13,24.12ZM124,68.15A59.7,59.7,0,0,0,88.53,82.87L65.87,60.21A91.64,91.64,0,0,1,124,36.09ZM82.87,88.53A59.7,59.7,0,0,0,68.15,124H36.09A91.64,91.64,0,0,1,60.21,65.87ZM68.15,132a59.7,59.7,0,0,0,14.72,35.47L60.21,190.13A91.64,91.64,0,0,1,36.09,132Zm20.38,41.13A59.7,59.7,0,0,0,124,187.85v32.06a91.64,91.64,0,0,1-58.13-24.12ZM132,187.85a59.7,59.7,0,0,0,35.47-14.72l22.66,22.66A91.64,91.64,0,0,1,132,219.91Zm41.13-20.38A59.7,59.7,0,0,0,187.85,132h32.06a91.64,91.64,0,0,1-24.12,58.13ZM187.85,124a59.7,59.7,0,0,0-14.72-35.47l22.66-22.66A91.64,91.64,0,0,1,219.91,124Z"></path>
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
