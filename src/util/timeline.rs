use yew::{function_component, html, Properties, Html};
use std::borrow::Cow;

/// A video or photo, embeddable as media inside a timeline node.
#[derive(PartialEq, Clone)]
pub enum TimelineMedia {
    /// A still image
    Image(String),

    /// A video frame
    Video(String),
}

/// A time renderable by a timeline.
#[derive(PartialEq, Clone)]
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
}

/// A component that renders an expandable image with a title, description, and
/// timestamp.
#[function_component(TimelineNode)]
pub fn timeline_node(props: &TimelineNodeProps) -> Html {
    html! {
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
pub fn timeline(
    props: &TimelineProps,
) -> Html {
    html! {
        <div>
            { props.items.iter().map(|prop| html! { <TimelineNode ..prop.clone() /> }).collect::<Vec<Html>>() }
        </div>
    }
}
