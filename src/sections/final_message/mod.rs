use yew::{html, function_component, classes};
use crate::util::section_header::SectionHeader;

/// The last word.
#[function_component(FinalMessage)]
pub fn final_message() -> Html {
    html! {
        <div id="end" class={classes!("w-screen", "pl-24", "pr-24", "pt-16", "pb-16", "bg-stone-900", "text-white", "flex", "flex-col", "justify-center", "items-center")}>
            <SectionHeader title={"Final Message"} subtitle={"Straight from the man himself. o7"} />
            <video loop={true} controls={true}>
                <source src="img/golden_age/zyzz.mp4" type="video/mp4" />
            </video>
        </div>
    }
}
