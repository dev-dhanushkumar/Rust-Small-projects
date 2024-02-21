use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    //first value of tuble read signal and second value write signal
    let (Conversation, set_convarsation)  = create_signal(cx, Conversation::new());
    let send = create_action(cx, move |new_message: &String| {
	let user_message = Message {
		text: new_message.clone(),
		user: true,
	};	
	set_conversation.update(move |c| {
		c.message.push(user_message);
	});

	converse(cx,  conversation.get())
    });
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rusty-llama.css"/>

        // sets the document title
        <Title text="Rusty Llama"/>
	<ChatArea conversation/>
	<TypeArea send />
    }
}















/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
