use yew::prelude::*;
use std::collections::HashMap;

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

fn get_ip() {
    match reqwest::get("https://httpbin.org/ip") {
        Ok(mut response) => {
            if response.status() == reqwest::status::Ok {
                match response.text() {
                    Ok(text) => println!("Response: {}", text),
                    Err(_) => println!("Error getting text from response")
                }
            } else {
                println!("Error getting response: {}", response.status());
            }
        }
        Err(_) => println!("Error getting response")
    }
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
    get_ip();
}
