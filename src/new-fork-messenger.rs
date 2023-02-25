use anyhow::{Error, Result};
use github_flows::{listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event("jaykchen", "vitesse-lite", vec!["fork"], handler).await;
    // let octo = get_octo(None);
    // let repo = octo.repos("jaykchen", "vitesse-lite").get().await?;

    // let full_name = repo.full_name.unwrap();
    // let stargazers_count = repo.stargazers_count.unwrap();
    // let text = format!(
    //     "Congratulations on your repository {} with {} stars.",
    //     full_name, stargazers_count
    // );
    // send_message_to_channel("jaykchen", "ik8", text);

    Ok(())
}

async fn handler(payload: EventPayload) {
    if let EventPayload::ForkEvent(e) = payload {
        let text = format!("{:?}", e);
        send_message_to_channel("ik8", "general", text);

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
