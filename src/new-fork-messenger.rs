use anyhow::{Error, Result};
use github_flows::{listen_to_event, octocrab::models::Repository, EventPayload};
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event("jaykchen", "vitesse-lite", vec!["fork"], handler).await;

    Ok(())
}

async fn handler(payload: EventPayload) {
    if let EventPayload::UnknownEvent(e) = payload {
        let sender = e.get("sender").expect("sender not obtained");
        let forkee_name = sender.get("login").expect("forkee_name not obtained");
        let html_url = sender.get("html_url").expect("html_url not obtained");

        let repo = e.get("repository").expect("repo not obtained");
        let full_name = repo.get("full_name").expect("full_name not obtained");
        let visibility = repo.get("visibility").expect("visibility not obtained");

        let text = format!(
            "{} forked your {}({})!\n{}",
            forkee_name, full_name, visibility, html_url
        );

        send_message_to_channel("ik8", "general", text);
    }
}
