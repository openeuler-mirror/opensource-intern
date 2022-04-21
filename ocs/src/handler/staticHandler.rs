use axum::{http::{Uri, Response, header, StatusCode}, response::IntoResponse, body::{boxed, Full}};
use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "site/docs/.vuepress/dist"]
struct Asset;
struct StaticFile<T>(pub T);
impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> axum::response::Response {
        let path = self.0.into();
        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path.as_str()).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from(format!("{} not found", path))))
                .unwrap(),
        }
    }
}



pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}

pub async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse().unwrap()).await
}

pub async fn handler(pkgName: &str) -> impl IntoResponse {
    let path = format!("/{}.html", pkgName);
    StaticFile(path)
}
