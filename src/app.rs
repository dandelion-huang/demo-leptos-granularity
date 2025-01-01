use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
	components::{Route, Router, Routes},
	StaticSegment,
};
use rand::Rng;

pub fn shell(options: LeptosOptions) -> impl IntoView {
	view! {
		<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="utf-8" />
				<meta name="viewport" content="width=device-width, initial-scale=1" />
				<AutoReload options=options.clone() />
				<HydrationScripts options />
				<MetaTags />
			</head>
			<body>
				<App />
			</body>
		</html>
	}
}

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	view! {
		// injects a stylesheet into the document <head>
		// id=leptos means cargo-leptos will hot-reload this stylesheet
		<Stylesheet id="leptos" href="/pkg/demo-leptos-granularity.css" />

		// sets the document title
		<Title text="Demo Leptos Granularity" />

		// content for this welcome page
		<Router>
			<main>
				<Routes fallback=|| "Page not found.".into_view()>
					<Route path=StaticSegment("") view=HomePage />
				</Routes>
			</main>
		</Router>
	}
}

const COLORS: [&str; 9] = [
	"lime", "amber", "cyan", "teal", "rose", "fuchsia", "red", "yellow", "indigo",
];

const COLOR_CODES: [&str; 5] = ["200", "300", "400", "500", "600"];

fn random_bg_color() -> String {
	let mut rng = rand::thread_rng();

	let from_color = COLORS[rng.gen_range(0..COLORS.len())];
	let from_code = COLOR_CODES[rng.gen_range(0..COLOR_CODES.len())];

	let to_color = COLORS[rng.gen_range(0..COLORS.len())];
	let to_code = COLOR_CODES[rng.gen_range(0..COLOR_CODES.len())];

	format!(
		"bg-gradient-to-br from-{}-{} to-{}-{}",
		from_color, from_code, to_color, to_code
	)
}

#[component]
fn BgContainer(
	children: Children,
	count: ReadSignal<i32>,
	#[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
	view! {
		<div class=format!(
			"{} {}",
			class.unwrap_or("relative p-16 text-center"),
			random_bg_color(),
		)>
			<p class="absolute right-0 left-0 top-1 mx-auto">"Count: " {count}</p>
			{children()}
		</div>
	}
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
	let (count, set_count) = signal(0);

	view! {
		<div class="flex flex-col gap-16 justify-center items-center bg-black h-dvh">
			<BgContainer count=count>
				<BgContainer count=count>
					<BgContainer count=count>{}</BgContainer>
				</BgContainer>
			</BgContainer>

			<div>
				<button
					class="py-2 px-4 bg-white rounded-md"
					on:click=move |_| set_count.update(|count| *count += 1)
				>
					"Click me"
				</button>
			</div>
		</div>
	}
}
