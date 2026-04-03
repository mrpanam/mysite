use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <nav class="w-64 min-h-screen bg-gray-800 text-white p-4 flex flex-col gap-2">
            <h2 class="text-xl font-bold mb-4 px-2">"Navigation"</h2>
            <A href="/"  exact=true attr:class="nav-link">"Home"</A>
            <A href="/courses" attr:class="nav-link">"Courses"</A>
            <A href="/students" attr:class="nav-link">"Students"</A>
            <A href="/settings" attr:class="nav-link">"Settings"</A>
        </nav>
    }
}
