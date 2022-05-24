use crate::util::{
    section_header::SectionHeader,
    timeline::{Timeline, TimelineNodeProps, TimelineTimestamp, TimelineMedia},
};
use yew::{classes, function_component, html};
use std::borrow::Cow;
use lazy_static::lazy_static;

lazy_static! {
    static ref MEDIA: ([TimelineMedia; 1], [TimelineMedia; 1], [TimelineMedia; 0]) = (
        [
            TimelineMedia::Video("img/sigma_fall_lawry.mov".to_owned()),
        ],
        [
            TimelineMedia::Image("img/lawry_smile.png".to_owned()),
        ],
        [],
    );

    static ref EVENTS: [TimelineNodeProps; 4] = [
        TimelineNodeProps{
            title: "The Sigma Era".to_owned(),
            description: "All about aesthetics. We were all finding our way in a new world. New handshakes, new discord servers, and new friends. You never ceased to amaze us.".to_owned(),
            timestamp: TimelineTimestamp::Range(1632240000, 1635490800),
            media: (&MEDIA.0[..]).into(),
        },
        TimelineNodeProps {
            title: "The Good Times".to_owned(),
            description: r#"Things were pretty good, but you made sure that good times didn't create weak men. Every day, we grew stronger, became bigger and better people, and did what we needed to do to adapt. Every day was a new experience, but all along the way, you reminded us, "what would gigachad do?""#.to_owned(),
            timestamp: TimelineTimestamp::Range(1635490800, 1639814400),
            media: (&MEDIA.1[..]).into(),
        },
        TimelineNodeProps {
            title: "The Dark Ages".to_owned(),
            description: "Winter quarter was rough for everyone. We went months without seeing each other, and became trapped in the things we had overcome over the last quarter. But yet, we prevailed. You held us together like glue, reminding us that every next day should be better than the last. You introduced us to self-improvement, and gave us hope in a time of darkness.".to_owned(),
            timestamp: TimelineTimestamp::Range(1639814400, 1616914800),
            media: (&MEDIA.2[..]).into(),
        },
        TimelineNodeProps {
            title: "The Golden Age".to_owned(),
            description: "".to_owned(),
            timestamp: TimelineTimestamp::Discrete(1616914800),
            media: (&MEDIA.2[..]).into(),
        },
    ];
}

/// A component that renders a timeline of things that happened this year.
#[function_component(RecapSection)]
pub(crate) fn recap_section() -> Html {
    html! {
        <div id="recap" class={classes!("w-screen", "pl-24", "pr-24", "pt-16", "pb-16", "dark:bg-stone-800", "dark:text-white", "flex", "flex-col", "justify-center", "items-center")}>
            <SectionHeader title="Recap" subtitle="You showed us that we're all gonna make it." />
            <Timeline items={ <&[TimelineNodeProps] as Into<Cow<'static, [TimelineNodeProps]>>>::into(&EVENTS[..]) } />
        </div>
    }
}
