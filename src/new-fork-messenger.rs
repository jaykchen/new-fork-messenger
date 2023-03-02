use github_flows::{get_octo, listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event("jaykchen", "vitesse-lite", vec!["fork"], handler).await;

    Ok(())
}

async fn handler(payload: EventPayload) {
    let octocrab = get_octo(None);

    let repo = octocrab
        .repos("jaykchen", "vitesse-lite")
        .get()
        .await
        .expect("repo not obtained");

    let full_name = repo.full_name.expect("full_name not obtained");
    let visibility = repo.visibility.expect("visibility not obtained");

    if let EventPayload::ForkEvent(e) = payload {
        let forkee = e.forkee;
        let forkee_name = forkee.owner.expect("no owner field").login;
        let html_url = forkee.html_url.expect("no html_url field").to_string();

        let text = format!(
            "{} forked your {}({})!\n{}",
            forkee_name, full_name, visibility, html_url
        );
        send_message_to_channel("ik8", "general", text);
    }
}
