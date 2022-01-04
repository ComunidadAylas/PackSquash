use actix_web::{body::Body, web, App, HttpResponse, HttpServer};
use rust_embed::RustEmbed;
use std::borrow::Cow;
use web_view::{Content, WebViewBuilder};

#[derive(RustEmbed)]
#[folder = "front/dist"]
struct EmbeddedAssets;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let server = HttpServer::new(|| {
		App::new().service(web::resource("/{_:.*}").route(web::get().to(
			|path: web::Path<String>| {
				match EmbeddedAssets::get(&path.0) {
					Some(asset) => HttpResponse::Ok()
						.content_type(
							mime_guess::from_path(path.0)
								.first_or_octet_stream()
								.as_ref()
						)
						.body(asset_data_to_body(asset.data)),
					None => {
						// Required by Vue.js routing:
						// https://cli.vuejs.org/guide/deployment.html#routing-with-history-pushstate
						HttpResponse::Ok().body(asset_data_to_body(
							EmbeddedAssets::get("index.html").unwrap().data
						))
					}
				}
			}
		)))
	})
	.workers(1)
	.bind(("127.0.0.1", 8080))?
	.run();

	WebViewBuilder::new()
		.title("PackSquash")
		.size(480, 360)
		.content(Content::Url("http://127.0.0.1:8080"))
		.user_data(())
		.invoke_handler(|_webview, _arg| Ok(()))
		.run()
		.unwrap();

	server.await
}

fn asset_data_to_body(data: Cow<'static, [u8]>) -> Body {
	match data {
		Cow::Owned(data) => data.into(),
		Cow::Borrowed(data) => data.into()
	}
}
