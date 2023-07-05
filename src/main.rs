#![allow(non_snake_case)]
use dioxus::html::button;
use dioxus::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

fn main() {
    dioxus_desktop::launch(App);
}

const TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiU1REX0NBTExTSUdOLTEiLCJ2ZXJzaW9uIjoidjIiLCJyZXNldF9kYXRlIjoiMjAyMy0wNi0yNCIsImlhdCI6MTY4ODUzNTExNywic3ViIjoiYWdlbnQtdG9rZW4ifQ.BEy9KyGmarkYvev_sgZCZ8gCmqH-gCLkvNc3-t78wNRD4NBojp0xpg88qRt96IOGiSi8kpMDryprrx33f5REj5p-_RDknl3D3tF0sK3JKRg5pgbSkqTuILXTQXQASTM0FzDCoxuARS34iV17RTgrKeaOUZ4iUlCIJINN3DnxqKbZqX1LfoRgTum4UpRzWTydxzJ5Yawog40-jM7s29tZOvcFJgXLqDF0uQ46ucGL55R909VPs13GKan0UgGwpPBFCWnM-kaVgJU50OXRkv4OART6-xGo-FVBHoFDtAnJ1_ifu2E1PkQP0qxw4uQ02St8EZpn-FG4oaA9L2_b2Z5_rg";

#[derive(Debug, Default, Deserialize, Serialize)]
struct AgentData {
    accountId: String,
    credits: i64,
    headquarters: String,
    startingFaction: String,
    symbol: String,
}

fn App(cx: Scope) -> Element {
    let token_value = use_state(cx, || TOKEN.to_string());

    let login_click_future = use_future(cx, (token_value,), |(token_value,)| async move {
        let agent_data_url = "https://api.spacetraders.io/v2/my/agent";

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Bearer {token_value}").as_str()).unwrap(),
        );

        // cx.spawn(async move {
        let res = reqwest::Client::new()
            .get(agent_data_url)
            .headers(headers)
            .send()
            .await;
        match res {
            Ok(r) => {
                if r.status() == 200 {
                    let data: Value = r.json().await.unwrap();
                    let data = data.get("data").unwrap();
                    let data: AgentData = serde_json::from_value(data.clone()).unwrap();
                    println!("ok, {:#?}", data);
                    return data;
                }
                AgentData::default()
            }

            Err(e) => {
                println!("err, {:#?}", e);
                AgentData::default()
            }
        }
    });

    match login_click_future.value() {
        Some(agent_data) if agent_data.accountId.is_empty() => {
            println!("the accountid is empty");
            None
        }
        Some(agent_data) => cx.render(rsx! {
                    h1 { "logged in!" }
        AgentDataComponent { agent_data: agent_data }
                }),
        _ => cx.render(rsx! {
            h1 { "not logged in!" }
        }),
    }
}

#[derive(Props)]
struct AgentDataProps<'a> {
    agent_data: &'a AgentData,
}

fn AgentDataComponent<'a>(cx: Scope<'a, AgentDataProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "agent-data",
            table {
                tr {
                    td { "accountId" }
                    td { "{cx.props.agent_data.accountId}" }
                }
                tr {
                    td { "credits" }
                    td { "{cx.props.agent_data.credits}" }
                }
                tr {
                    td { "hq" }
                    td { "{cx.props.agent_data.headquarters}" }
                }
                tr {
                    td { "startingFaction" }
                    td { "{cx.props.agent_data.startingFaction}" }
                }
                tr {
                    td { "symbol" }
                    td { "{cx.props.agent_data.symbol}" }
                }
            }
        }
    ))
}
