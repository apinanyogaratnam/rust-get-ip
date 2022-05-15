use yew::prelude::*;
use std::collections::HashMap;
use serde::Deserialize;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

#[derive(Deserialize)]
struct IP {
    origin: String,
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

async fn get_ip() -> String {
    let response: Result<IP, reqwest::Error> = reqwest::get("http://ip.jsontest.com/")
        .await
        .unwrap()
        .json::<IP>()
        .await;

    return match response {
        Ok(ip) => ip.origin.clone(),
        Err(e) => format!("Error: {}", e),
    };
}

#[tokio::main]
async fn make_request() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

fn main() {
    // yew::start_app::<Model>();
    let ip = get_ip();
}
