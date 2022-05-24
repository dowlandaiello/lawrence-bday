mod sections;
mod util;

use yew::{function_component, html};
use sections::{landing_hero::MainHero, recap::RecapSection};
use util::nav::NavBar;
use console_log::init_with_level;
use log::Level;

/// The names and anchors to parts on the page for navigation purposes.
const PAGES: &'static [(&'static str, &'static str)] = &[("HOME", "#based"), ("RECAP", "#recap")];

/// The root of the application.
#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <NavBar pages={PAGES}/>
            <MainHero />
            <RecapSection />
        </>
    }
}

fn main() {
    init_with_level(Level::Warn).expect("Could not init logger.");

    yew::start_app::<App>();
}
