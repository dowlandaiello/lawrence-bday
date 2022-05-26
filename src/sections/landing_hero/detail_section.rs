use yew::{html, function_component, classes};

/// A component that displays a greeting and some text to show the user.
#[function_component(DetailSection)]
pub(crate) fn detail_section() -> Html {
    html! {
        <div class={classes!("w-1/2", "xl:w-2/6", "h-full", "flex", "flex-col", "justify-center", "items-start", "space-y-6", "pl-24")}>
            <div class={classes!("flex", "flex-row")}>
                <h1 class={classes!("text-6xl", "font-black", "text-white")}>{"Happy Birthday, Lawrence"}</h1>
            </div>
            <p class={classes!("text-lg", "text-stone-400")}>{"Thanks for being so awesome and making my year such a mog-fest."}</p>
            <a href="#recap" class={classes!("transition-all", "ease-in", "font-bold", "duration-100", "w-full", "text-lg", "bg-amber-700", "hover:bg-transparent", "hover:ring-2", "hover:ring-amber-800", "text-white", "hover:text-stone-300", "py-2", "px-6", "text-center", "align-middle")}>
                <span class={classes!("material-symbols-sharp", "align-middle", "mr-2")}>
                    {"timeline"}
                </span>
                {"RECAP"}
            </a>
        </div>
    }
}
