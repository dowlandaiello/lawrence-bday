use yew::{Html, html, Component, Context, classes};
use gloo_timers::callback::{Interval, Timeout};

/// The path containing all of the GIGACHADS
const N_CHADS: u8 = 4;
const CHADS: [&'static str; N_CHADS as usize] = [
    "char_chad.png",
    "jamchad.png",
    "posechad.png",
    "chad-side.png",
];

/// Switch chads every 2 sec
const CHAD_INTERVAL: u32 = 2000;

/// A component that renders a flashing slideshow of gigachads.
pub struct ChadSlideshow {
    i: u8,

    // Whether the transition for the chad has played
    loaded: bool,
    queue_i: u8,

    // Not used, the Drop implementation for Interval will automatically
    // de-schedule this, but if we don't have a handle on it, it won't get
    // dropped ever
    _changer_handle: Interval,
}

/// Ways the slideshow can be dispatched.
pub enum SlideshowMsg {
    /// Fades out the current slide
    QueueNextSlide,

    /// Shows the next slide
    AdvanceSlide,

    /// Fades in the slide
    ShowSlide,
}

impl Component for ChadSlideshow {
    type Message = SlideshowMsg;
    type Properties = ();

    // Schedules the slide changer
    fn create(ctx: &Context<Self>) -> Self {
        // A reference to the self actor that allows sending messages
        let scope = ctx.link().clone();
        scope.send_message(SlideshowMsg::ShowSlide);

        Self {
            i: 0,
            queue_i: 0,
            loaded: false,

            // Every n seconds, move to the next slide
            _changer_handle: Interval::new(CHAD_INTERVAL, move || {
                scope.send_message(SlideshowMsg::QueueNextSlide);
            })
        }
    }

    // Handles operations on the slideshow
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SlideshowMsg::QueueNextSlide => {
                // Move to the next slide
                self.loaded = false;
                self.queue_i = (self.i + 1) % N_CHADS;

                true
            },
            SlideshowMsg::AdvanceSlide => {
                self.i = self.queue_i;

                true
            },
            SlideshowMsg::ShowSlide => {
                self.loaded = true;

                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let scope = ctx.link().clone();

        // The chad should always be visible, but there should be at least
        // one frame where it is zero opacity
        if !self.loaded && self.i == self.queue_i {
            Timeout::new(100, move || scope.send_message(SlideshowMsg::ShowSlide)).forget();
        // The chad must be advanced
        } else if self.queue_i != self.i {
            Timeout::new(100, move || scope.send_message(SlideshowMsg::AdvanceSlide)).forget();
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Chads are located in the img folder
        let chad_url = format!("img/{}", CHADS[self.i as usize]);
        let mut classes = classes!(
            "transition",
            "ease-out",
            "duration-800",
            "h-5/6",
            "xl:h-full",
            "-mr-20",
            "mt-auto",
            "-scale-x-100",
            "scale-y-100",
            "opacity-0",
            "drop-shadow-md",
        );

        // Fade in the chad
        if self.loaded {
            classes.push("opacity-95");
            classes.push("-scale-x-120");
            classes.push("scale-y-120");
        }

        html! {
            <div class={classes!("h-full", "flex", "justify-end", "align-end", "overflow-clip")}>
                <img class={classes} src={ chad_url } />
            </div>
        }
    }
}
