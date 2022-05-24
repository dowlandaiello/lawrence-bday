use yew::{function_component, html, classes};

mod detail_section;
mod chads;

use detail_section::DetailSection;
use chads::ChadSlideshow;

/// A component that renders a hero saying happy birthday!
#[function_component(MainHero)]
pub(crate) fn main_hero() -> Html {
    html! {
        <div id="based" class={classes!("h-screen", "w-screen", "dark:bg-stone-900", "grid", "grid-cols-1", "grid-rows-1")}>
            <div class={classes!("col-start-1", "row-start-1", "col-span-1", "row-span-1", "z-20", "2xl:ml-80")}>
                <DetailSection />
            </div>
            <div class={classes!("col-start-1", "row-start-1", "col-span-1", "row-span-1", "opacity-40", "grid", "grid-cols-1", "grid-rows-1")}>
                <div class={classes!("bg-repeat-space", "bg-origin-content", "bg-[length:8em]", "col-start-1", "row-start-1", "col-span-1", "row-span-1", "bg-trident-pattern", "z-0", "invert", "opacity-10")}>
                </div>
                <div class={classes!("col-start-1", "row-start-1", "col-span-1", "row-span-1", "z-10")}>
                    <ChadSlideshow />
                </div>
            </div>
            <div class={classes!("col-start-1", "row-start-1", "col-span-1", "row-span-1", "w-1/2", "ml-auto", "z-20", "bg-gradient-to-r", "from-transparent", "to-black", "opacity-40")} />
        </div>
    }
}
