use anyhow::{Error, Result};
use github_flows::{listen_to_event, octocrab::models::Repository, EventPayload};
use serde_json::Value;
use slack_flows::send_message_to_channel;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event("jaykchen", "vitesse-lite", vec!["fork"], handler).await;

    Ok(())
}

async fn handler(payload: EventPayload) {
    let mut forkee_name: String = "no forkee_name found".to_string();
    let mut full_name: &str = "no full_name found";
    let mut visibility: &str = "no visibility found";
    let mut html_url: String = "no html_url found".to_string();

    // if let value: Value = serde_json::from_str(payload.ForkEventPayload.clone()) {
    //     let repo = value.repo.expect("repo doesn't exist on CreateEvent payload");
    //     full_name = repo.name;
    //     visibility = if e.public { "public" } else { "private" };
    // };

    if let EventPayload::ForkEvent(e) = payload {
        let forkee = e.forkee;
        forkee_name = forkee.owner.expect("no owner field").login;
        html_url = forkee.html_url.expect("no html_url field").to_string();
    }
    let text = format!(
        "{} forked your {}({})!\n{}",
        forkee_name, full_name, visibility, html_url
    );

    send_message_to_channel("ik8", "general", text);
}
