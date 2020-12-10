use web_view::*;

fn main() {
    web_view::builder()
        .title("Rustlinks")
        .content(Content::Html(include_str!("index.html")))
        .size(1000, 700)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
