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
    item_renderer: ItemRenderer,
    stacks: Stacks,
}

enum Msg {}

const KEY: &str = "state";

struct ItemSheet {
    max_item: u16,
    url: &'static str,
}
static ITEM_URLS: [ItemSheet; 6] = [
    ItemSheet{
        max_item: 20,
        url: "https://lh3.googleusercontent.com/u/0/d/1P0bd7vtA_SVwC7Qm9dJ_YmZkQPNDLOCk=s3200-k-iv2",
    },
    ItemSheet {
        max_item: 40,
        url: "https://lh3.googleusercontent.com/u/0/d/1uHYherEvc9bv3Jpl2TpA2DlRPgGT8l3z=s3200-k-iv2",
    },
    ItemSheet {
        max_item: 60,
        url: "https://lh3.googleusercontent.com/u/0/d/172NPm8x9T8zPE2Vd672_10rZ3ieFHHOX=s3200-k-iv2",
    },
    ItemSheet {
        max_item: 90,
        url: "https://lh3.googleusercontent.com/u/0/d/1XAabPK_Hs8gBXXJVpaBaSsAv6_NioEyu=s3200-k-iv2",
    },
    ItemSheet {
        max_item: 133,
        url: "https://lh3.googleusercontent.com/u/0/d/1KW0TZOs7SDVl5frTM9y-ISR-4dZFIex4=s3200-k-iv2",
    },
    ItemSheet {
        max_item: 150,
        url: "https://lh3.googleusercontent.com/u/0/d/1BHfEwqmC_dax5dV4RFFP5MlfJ76eS7KZ=s3200-k-iv2",
    }
];

static ITEM_COUNTS: [Card; 150] = [2,2,2,2,2,2,2,2,2,2,2,4,4,4,2,2,2,2,2,4,2,2,2,2,2,2,4,2,2,2,2,2,2,4,2,2,2,2,2,2,4,2,2,2,2,2,2,4,2,2,2,2,2,2,4,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,2,1,1,1,2,2,2,1,1,1,1,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];

struct ItemInSheet {
    url: &'static str,
    number_in_picture: u16,
    sheet_size: u16,
}

struct ItemRenderer(Vec<ItemInSheet>);

impl ItemRenderer {
    fn new() -> ItemRenderer {
        let mut acc: Card = 0;
        let mut sheet_idx: usize = 0;
        let mut sheet_size = ITEM_URLS[sheet_idx].max_item;
        ItemRenderer(ITEM_COUNTS.iter().enumerate().map(|(idx, num)| {
            let mut num = *num;
            if idx >= 71 && idx <= 95 {
                // there are two random items but only one of them is "red", i.e. in this picture
                num -= 1;
            }

            acc += num;
            let mut number_in_picture = acc;
            if idx+1 > ITEM_URLS[sheet_idx].max_item.into() {
                sheet_idx += 1;
                sheet_size = ITEM_URLS[sheet_idx].max_item - ITEM_URLS[sheet_idx-1].max_item;
                number_in_picture = 0;
                acc = 0;
            }

            ItemInSheet {
                url: ITEM_URLS[sheet_idx].url,
                number_in_picture,
                sheet_size,
            }
        }).collect())
    }
}

impl<Context> Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        let mut console = ConsoleService::new();
        let mut storage = StorageService::new(Area::Local);
        let res = storage.restore(KEY);
        match res {
            Json(Ok(stacks)) => {
                Model {
                    console,
                    storage,
                    item_renderer: ItemRenderer::new(),
                    stacks,
                }
            },
            Json(Err(e)) => {
                console.log(&format!("Error deserializing localStorage: {}", e));
                Model {
                    console,
                    storage,
                    item_renderer: ItemRenderer::new(),
                    stacks: Stacks::default(),
                }
            },
        }
    }

    fn update(&mut self, _: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
//        match msg {
//            Msg::Load => {
//                // Update your model on events
//                true
//            }
//        }
        true
    }
}

struct CardInfo {
    card: Card,
    url: &'static str,
    cols: u16,
    offset: u16,
    height: u16,
    width: u16,
}

impl Model {

    fn render_card<Context>(&self, card: CardInfo) -> Html<Context, Self>
        where Context: 'static,{
        let n = card.card - card.offset;
        let row = n / card.cols;
        let col = n % card.cols;
        let style = format!("background: url({url}) no-repeat scroll top -{totalHeight}px left -{totalWidth}px;\
width: {actualWidth}px;\
maxWidth: {actualWidth}px;\
height: {actualHeight}px;\
margin-left: 10px;\
color: white;\
padding: 0 0 3px 14px;\
display: inline-block",
            url=card.url,
            totalHeight=(row * card.height),
            totalWidth=(col * card.width),
            actualWidth=card.width-14,
            actualHeight=card.height-3);

        html! {
            <div style=style,>{card.card}</div>
        }
    }

    fn render_item<Context>(&self, card: Card) -> Html<Context, Self>
        where Context: 'static,{
        //TODO: The -1 is very ugly
        let itemInSheet = &self.item_renderer.0[(card-1) as usize];
        self.render_card(CardInfo{
            card,
            url: itemInSheet.url,
            cols: 10,
            offset: card - itemInSheet.number_in_picture,
            width: 292,
            height: 458,
        })
    }

    fn render_item_list<Context>(&self, title: &str, list: &[Card]) -> Html<Context, Self>
        where Context: 'static,
    {
        html! {
            <>
                <h2>{title}</h2>
                { for list.iter().map(|card| self.render_item(*card)) }
            </>
        }
    }
}

impl<Context> Renderable<Context, Model> for Model where Context: 'static {

    fn view(&self) -> Html<Context, Self> {
        let stacks = &self.stacks;
        html! {
            <>
                {self.render_item_list("Random Item Designs", stacks.random_item_designs.list.as_slice())}
                <pre>{serde_json::to_string_pretty(&self.stacks).unwrap()}</pre>
            </>
        }
    }

}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
