use yew::{function_component, html, Html, Properties, classes};

/// The list of pages that the nav bar should render.
#[derive(Properties, PartialEq)]
pub struct NavBarProps {
    pub(crate) pages: &'static [(&'static str, &'static str)],
}

/// Displays buttons to go to different parts of the site.
#[function_component(NavBar)]
pub(crate) fn nav_bar(props: &NavBarProps) -> Html {
    // Build a pretty anchor button that navigates to the indicated link for
    // each page
    let anchors: Vec<Html> = props
        .pages
        .iter()
        .map(|(name, anchor)| html! {
            <a class={classes!("transition", "font-bold", "font-mono", "text-white", "opacity-100", "hover:opacity-70", "border-b-2", "border-transparent", "hover:border-amber-700")} href={ anchor.to_owned() }>{ name }</a>
        })
        .collect();

    html! {
        <div class={classes!("fixed", "w-full", "flex", "flex-row", "justify-between", "top-0", "pl-24", "pr-24", "pt-12", "z-30", "text-2xl")}>
            <a class={classes!("transition", "material-symbols-sharp", "text-amber-600", "hover:text-amber-800")} href="#">{ "home" }</a>
            <div class={classes!("flex", "flex-row", "space-x-8")}>
                { anchors }
            </div>
        </div>
    }
}
