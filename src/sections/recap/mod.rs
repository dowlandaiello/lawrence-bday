use crate::util::{
    section_header::SectionHeader,
    timeline::{Timeline, TimelineNodeProps, TimelineTimestamp, TimelineMedia},
};
use yew::{classes, function_component, html};
use std::borrow::Cow;
use lazy_static::lazy_static;

lazy_static! {
    static ref MEDIA: ([TimelineMedia; 5], [TimelineMedia; 3], [TimelineMedia; 1], [TimelineMedia; 4]) = (
        [
            TimelineMedia::Video("img/sigma_fall_lawry.mp4".to_owned()),
            TimelineMedia::Image("img/sigma_era/PXL_20211209_200632479.jpg".to_owned()),
            TimelineMedia::Image("img/sigma_era/PXL_20211206_230042816.jpg".to_owned()),
            TimelineMedia::Image("img/sigma_era/PXL_20211118_192817976.jpg".to_owned()),
            TimelineMedia::Video("img/sigma_era/PXL_20211020_211154067.mp4".to_owned()),
        ],
        [
            TimelineMedia::Image("img/lawry_smile.png".to_owned()),
            TimelineMedia::Video("img/lawry_flex.mp4".to_owned()),
            TimelineMedia::Image("img/good_times/lawry_ass.jpg".to_owned()),
        ],
        [
            TimelineMedia::Video("img/dark_ages/saul.mp4".to_owned()),
        ],
        [
            TimelineMedia::Image("img/golden_age/lawry_sus.jpg".to_owned()),
            TimelineMedia::Image("img/golden_age/veins.jpg".to_owned()),
            TimelineMedia::Image("img/golden_age/lawry_veins.jpg".to_owned()),
            TimelineMedia::Image("img/golden_age/lawry_vision.jpeg".to_owned()),
        ],
    );

    static ref EVENTS: [TimelineNodeProps; 4] = [
        TimelineNodeProps{
            title: "The Sigma Era".to_owned(),
            description: "It was all about aesthetics. We were all finding our way in a new world. New handshakes, new discord servers, and new friends. And for many of us, you showed us something new: the capabilities of man if he disciplines himself fully.".to_owned(),
            timestamp: TimelineTimestamp::Range(1632240000, 1635490800),
            media: (&MEDIA.0[..]).into(),
            i: 0,
        },
        TimelineNodeProps {
            title: "The Good Times".to_owned(),
            description: r#"Things were pretty good, but you made sure that good times didn't create weak men. Every day, we grew stronger, became bigger and better people, and did what we needed to do to adapt. Every day was a new experience, but all along the way, you reminded us, "what would gigachad do?""#.to_owned(),
            timestamp: TimelineTimestamp::Range(1635490800, 1639814400),
            media: (&MEDIA.1[..]).into(),
            i: 1,
        },
        TimelineNodeProps {
            title: "The Dark Ages".to_owned(),
            description: "Winter quarter was rough for everyone. We went months without seeing each other, and became trapped in the things we had overcome over the last quarter. But yet, we prevailed. You held us together like glue, reminding us that every next day should be better than the last. You introduced us to self-improvement, and gave us hope in a time of darkness.".to_owned(),
            timestamp: TimelineTimestamp::Range(1639814400, 1648299214),
            media: (&MEDIA.2[..]).into(),
            i: 2,
        },
        TimelineNodeProps {
            title: "The Golden Age".to_owned(),
            description: "We've learned so much from you, Lawrence. Things are better than ever, and I'm loving life more than I ever have before. Things are on the up and up, and every day I wake up better than before. Thank you.".to_owned(),
            timestamp: TimelineTimestamp::Discrete(1648299214),
            media: (&MEDIA.3[..]).into(),
            i: 3,
        },
    ];
}

/// A component that renders a timeline of things that happened this year.
#[function_component(RecapSection)]
pub(crate) fn recap_section() -> Html {
    html! {
        <div id="recap" class={classes!("w-screen", "pl-24", "pr-24", "pt-16", "pb-16", "bg-stone-800", "text-white", "flex", "flex-col", "justify-center", "items-center")}>
            <SectionHeader title="Recap" subtitle="You showed us that we're all gonna make it." />
            <Timeline items={ <&[TimelineNodeProps] as Into<Cow<'static, [TimelineNodeProps]>>>::into(&EVENTS[..]) } />
            <p class={classes!("text-stone-400", "mt-4")}>{"As far as our dreams can take us"}</p>
        </div>
    }
}
