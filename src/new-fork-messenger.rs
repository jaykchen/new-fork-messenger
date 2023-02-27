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
    if let EventPayload::ForkEvent(e) = payload {
        let forkee = e.forkee;
        send_message_to_channel("ik8", "general", forkee.name);
        send_message_to_channel("ik8", "general", forkee.url.to_string());
        send_message_to_channel(
            "ik8",
            "general",
            forkee.full_name.expect("no full_name obtained"),
        );

        // let forkee_name = forkee.owner.unwrap().login;

        // let text = format!(
        //     "{} forked your {}({})!\n{}",
        //     forkee_name, repo.full_name, repo.visibility, repo.html_url
        // );

        // send_message_to_channel("jaykchen", "ik8", text.clone());

        // if stargazers_count % 10 == 0 {
        //     send_message_to_channel("jaykchen", "ik8", text)
        // }
    }
}
