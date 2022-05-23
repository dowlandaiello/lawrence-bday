use yew::{Component, Context, Html};

/// The root. No props needed.
struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        }
    }
}

fn main() {
    yew::start_app<App>();
}
