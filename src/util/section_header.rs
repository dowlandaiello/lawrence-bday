use yew::{html, function_component, Properties, classes};

/// Properties for a section header.
/// Takes a title and subtitle.
#[derive(Properties, PartialEq)]
pub struct SectionHeaderProps<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
}

/// A component that renders a title and subtitle.
#[function_component(SectionHeader)]
pub(crate) fn section_header(props: &SectionHeaderProps<'static>) -> Html {
    html! {
        <div class={classes!("flex", "flex-col", "justify-center", "items-center", "space-y-2")}>
            <h1 class={classes!("text-3xl", "font-bold", "dark:text-stone-300")}>{props.title}</h1>
            <p class={classes!("dark:text-stone-400")}>{props.subtitle}</p>
        </div>
    }
}
