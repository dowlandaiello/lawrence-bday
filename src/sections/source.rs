use yew::{html, function_component, classes};

/// Source code disclosure
#[function_component(Source)]
pub fn source() -> Html {
    html! {
        <div class={classes!("w-screen", "pl-24", "pr-24", "pt-4", "pb-4", "bg-stone-800", "text-white", "flex", "flex-row", "justify-start", "items-center", "space-x-2")}>
            <img class={classes!("h-8", "opacity-50")} src="img/gh_logo.png" />
            <p class={classes!("opacity-50")}>{"Check out the "}<a target="_blank" class={classes!("font-mono", "underline")}href="https://github.com/dowlandaiello/lawrence-bday">{"source code"}</a></p>
        </div>
    }
}
