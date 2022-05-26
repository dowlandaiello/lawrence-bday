use yew::{Properties, function_component, use_state, html, classes, Callback};

/// The number of items to be cycled through, and what to do when the selector
/// is advanced.
#[derive(Properties, Clone, PartialEq)]
pub struct MediaAdvancerProps {
    pub n_items: usize,
    pub on_select_item: Callback<usize>,
}

/// A component that renders a previous and next item selector our of a number
/// of maximum items.
#[function_component(MediaAdvancer)]
pub fn media_advancer(props: &MediaAdvancerProps) -> Html {
    let i = use_state(|| 0);

    let n_items = props.n_items;

    let change_pos = |incr: isize| {
        let i = i.clone();
        let cb = props.on_select_item.clone();

        Callback::from(move |_| {
            cb.emit(((*i + incr) % (n_items as isize)) as usize);
            i.set((*i + incr) % (n_items as isize));
        })
    };

    let btn_style = classes!("transition", "material-symbols-sharp", "text-amber-600", "hover:text-amber-800", "cursor-pointer", "text-sm");

    html! {
        <div class={classes!("flex", "flex-row", "items-center", "justify-between", "w-full")}>
            <span class={btn_style.clone()} onclick={change_pos(-1)}>{"arrow_back_ios"}</span>
            <p class={classes!("font-mono", "opacity-50")}>{ format!("{} / {}", *i + 1, props.n_items) }</p>
            <span class={btn_style} onclick={change_pos(1)}>{"arrow_forward_ios"}</span>
        </div>
    }
}
