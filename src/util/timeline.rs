use super::media_advancer::MediaAdvancer;
use chrono::{Datelike, TimeZone, Utc};
use std::borrow::Cow;
use yew::{
    classes, function_component, html, use_state,
    virtual_dom::{VList, VNode},
    Callback, Html, Properties,
};

/// A video or photo, embeddable as media inside a timeline node.
#[derive(PartialEq, Clone)]
pub enum TimelineMedia {
    /// A still image
    Image(String),

    /// A video frame
    Video(String),
}

/// A time renderable by a timeline.
#[derive(PartialEq, Clone, Copy)]
pub enum TimelineTimestamp {
    /// A timestamp with a start and end
    Range(usize, usize),

    /// A timestamp with one time
    Discrete(usize),
}

/// Props for a timeline node.
/// Specifies the contents of the node, its image, its title, and its
/// timestamp.
#[derive(Properties, PartialEq, Clone)]
pub struct TimelineNodeProps {
    pub title: String,
    pub description: String,
    pub timestamp: TimelineTimestamp,
    pub media: Cow<'static, [TimelineMedia]>,
    pub i: usize,
}

// Used for human readability
const MONTH_NAMES: [&str; 12] = [
    "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
];

// Gets a human readable representation of a timestamp.
fn format_timestamp(timestamp: TimelineTimestamp) -> String {
    match timestamp {
        TimelineTimestamp::Range(start, end) => format!(
            "{} - {}",
            format_timestamp(TimelineTimestamp::Discrete(start)),
            format_timestamp(TimelineTimestamp::Discrete(end))
        ),
        TimelineTimestamp::Discrete(t) => {
            let parsed = Utc.timestamp(t as i64, 0);
            format!(
                "{} {}",
                MONTH_NAMES[parsed.month0() as usize],
                parsed.iso_week().year()
            )
        }
    }
}

/// A component that renders an expandable image with a title, description, and
/// timestamp.
#[function_component(TimelineNode)]
pub fn timeline_node(props: &TimelineNodeProps) -> Html {
    let mut container_classes = classes!(
        "w-1/2",
        "flex",
        "flex-row",
        "items-center",
        "space-x-4",
        "border-r",
        "border-white/20"
    );
    let expanded = use_state(|| false);

    // The user can switch between the possible media items. Start on the first one
    let cur_media_index = use_state(|| 0 as usize);

    let mut content_classes = classes!(
        "transition",
        if *expanded { "h-full" } else { "h-0" },
        if *expanded { "w-full" } else { "w-0" },
        "overflow-clip",
        "flex",
        "flex-nowrap",
        "mt-8"
    );

    // Put odd items on the right side
    if props.i % 2 != 0 {
        container_classes.push("self-end");
        container_classes.push("flex-row-reverse");
        container_classes.push("space-x-reverse");
        container_classes.push("border-r-0");
        container_classes.push("border-l");
    }

    // Render the active media according to its type
    let cur_media = if *cur_media_index >= props.media.len() {
        VNode::from(VList::new())
    } else {
        match props.media[*cur_media_index].clone() {
            TimelineMedia::Video(path) => {
                content_classes.push("space-y-6");
                content_classes.push("flex-col");
                content_classes.push("items-center");
                content_classes.push("justify-start");

                html! {
                    <video class={classes!("transition", "m-auto", "rounded-mid")} autoplay={true} loop={true} controls={true}>
                        <source src={path} type="video/mp4" />
                    </video>
                }
            }
            TimelineMedia::Image(path) => {
                content_classes.push("space-x-6");
                content_classes.push("flex-row");
                content_classes.push("items-start");
                content_classes.push("justify-start");

                html! {
                    <img class={classes!("transition", "w-full", "rounded-md")} src={path} />
                }
            }
        }
    };

    let toggle_expanded = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };

    let select_media = {
        let cur_media_index = cur_media_index.clone();
        Callback::from(move |i: usize| {
            cur_media_index.set(i);
        })
    };

    html! {
        <div class={container_classes}>
            <div class={classes!("p-8", "bg-stone-900", "rounded-md", if *expanded { "w-3/4" } else { "w-auto" })}>
                <div class={classes!("flex", "flex-row", "items-center", "justify-between", "space-x-20")}>
                    <div class={classes!("flex", "flex-row", "items-center", "justify-start", "space-x-2")}>
                        <span class={classes!("transition", "material-symbols-sharp", "cursor-pointer", "hover:opacity-50")} onclick={toggle_expanded}>{ if *expanded { "expand_more" } else { "expand_less" } }</span>
                        <p class={classes!("text-xl", "font-bold")}>{ props.title.as_str() }</p>
                    </div>
                    <p class={classes!("font-mono", "opacity-50")}>{ format_timestamp(props.timestamp) }</p>
                </div>
                <div class={content_classes}>
                    <div class={classes!("w-full", "h-full", "space-y-4")}>
                        { cur_media }
                        { if props.media.len() > 0 { html! { <MediaAdvancer n_items={props.media.len()} on_select_item={select_media} /> } } else { VNode::from(VList::new()) } }
                    </div>
                    <p class={classes!("text-lg", "text-stone-400")}>{ props.description.as_str() }</p>
                </div>
            </div>
            <hr class={classes!("grow", "opacity-20")} />
        </div>
    }
}

/// Props for a timeline.
#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    pub items: Cow<'static, [TimelineNodeProps]>,
}

/// A component that renders a sequence of images with titles, descriptions,
/// and timestamps linearly.
#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    html! {
        <div class={classes!("w-full", "space-y-4", "flex", "flex-col", "justify-center", "items-start")}>
            { props.items.iter().map(|prop| html! { <TimelineNode ..prop.clone() /> }).collect::<Vec<Html>>() }
        </div>
    }
}
