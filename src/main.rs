extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::format::Json;
use yew::services::console::{ConsoleService};
use yew::services::storage::{Area, StorageService};

type Card = u16;
type Context = ();

//TODO: Refactor into enum! :)
#[derive(Debug, Deserialize, Serialize)]
struct History{
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<Card>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    cards: Vec<Card>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct EventPile {
    stack: Vec<Card>,
    history: Vec<History>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct DrawPile {
    stack: Vec<Card>,
    list: Vec<Card>,
    history: Vec<History>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct List {
    list: Vec<Card>,
    history: Vec<History>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Stacks {
    city_events: EventPile,
    road_events: EventPile,
    random_item_designs: DrawPile,
    random_scenarios: DrawPile,
    personal_goals: DrawPile,
    prosperity: u8,
}

struct Model {
    storage: StorageService,
    console: ConsoleService,
    stacks: Stacks,
}

enum Msg {

}

const KEY: &str = "state";

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let mut console = ConsoleService::new();
        let mut storage = StorageService::new(Area::Local);
        let res = storage.restore(KEY);
        match res {
            Json(Ok(stacks)) => {
                Model {
                    console,
                    storage,
                    stacks,
                }
            },
            Json(Err(e)) => {
                console.log(&format!("Error deserializing localStorage: {}", e));
                Model {
                    console,
                    storage,
                    stacks: Stacks::default(),
                }
            },
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
//        match msg {
//            Msg::Load => {
//                // Update your model on events
//                true
//            }
//        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <pre>{serde_json::to_string_pretty(&self.stacks).unwrap()}</pre>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
