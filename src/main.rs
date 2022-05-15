use yew::prelude::*;
use serde::{Serialize, Deserialize};

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "Add" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct IP {
    #[serde(rename = "origin")]
    ip: String,
}

async fn get_ip() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res: IP = client.get("https://httpbin.org/ip").send().await?.json().await?;

    return Ok(res.ip);
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let ip = get_ip().await?;
    println!("{}", ip);

    Ok(())
}

// fn main() {
//     yew::start_app::<Model>();
// }
