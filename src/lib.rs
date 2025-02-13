use worker::*;
use serde_json::{Value};

async fn get_email_post() -> Result<String> {
	console_log!("start get_email_post");

	// Create template
	let post_template = r#"
			{
				"personalizations": [
					{
						"to": [
							{
								"email": "test@example.com",
								"name": "Test Recipient"
							}],
					},
				],
				"from": {
					"email": "sender@example.com",
					"name": "Workers - MailChannels integration",
				},
				"subject": "Look! No servers",
				"content": [
					{
						"type": "text/plain",
						"value": "And no email service accounts and all for free too!",
					},
				],
			}"#;
	console_log!("get_email_post before from_str {post_template}");

	let mut v: Value = serde_json::from_str(post_template)?;
	console_log!("get_email_post after from_str");

	// Fill template
	console_log!("fill template");

	v["personalizations"][0]["email"] = serde_json::to_value("fun@example.com")?;
	v["personalizations"][0]["name"] = serde_json::to_value(&"milot")?;
	v["from"]["email"] = serde_json::to_value(&"neat@example.com")?;

	return Ok(v.to_string());
}


#[event(fetch)]
async fn fetch(
    _req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();
	let _post = get_email_post().await?;
    Response::ok("Hello World!")
}