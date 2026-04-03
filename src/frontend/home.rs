use crate::backend::db::get_all_teachers;
use crate::backend::model::Teacher;
use leptos::prelude::*;
use surrealdb_types::RecordIdKey;

#[component]
pub fn HomePage() -> impl IntoView {
    let teachers = Resource::new(|| (), |_| get_all_teachers());

    view! {
        <div class="p-4">
            <h1 class="text-2xl font-bold mb-4">"Teachers"</h1>
            <table class="min-w-full border-collapse border border-gray-300">
                <thead>
                    <tr>
                        <th class="border border-gray-300 px-4 py-2 bg-gray-100">"Key"</th>
                        <th class="border border-gray-300 px-4 py-2 bg-gray-100">"Name"</th>
                        <th class="border border-gray-300 px-4 py-2 bg-gray-100">"Expertise"</th>
                        <th class="border border-gray-300 px-4 py-2 bg-gray-100">"Created At"</th>
                    </tr>
                </thead>
                <tbody>
                    <Suspense>
                        {move || match teachers.get() {
                            Some(Ok(teachers)) => {
                                if teachers.is_empty() {
                                    view! { <tr><td colspan="4" class="border border-gray-300 px-4 py-2 text-center">"No teachers found"</td></tr> }.into_any()
                                } else {
                                    teachers
                                        .into_iter()
                                        .map(|teacher| {

                                            view! {
                                                <tr>
                                                    <td class="border border-gray-300 px-4 py-2">{teacher.id_key()}</td>
                                                    <td class="border border-gray-300 px-4 py-2">{teacher.name}</td>
                                                    <td class="border border-gray-300 px-4 py-2">{teacher.expertise}</td>
                                                    <td class="border border-gray-300 px-4 py-2">{teacher.created_at.to_string()}</td>
                                                </tr>
                                            }
                                        })
                                        .collect_view()
                                        .into_any()
                                }
                            }
                            Some(Err(e)) => view! { <tr><td colspan="4" class="border border-gray-300 px-4 py-2 text-center text-red-500">{format!("Error: {}", e)}</td></tr> }.into_any(),
                            None => view! { <tr><td colspan="4" class="border border-gray-300 px-4 py-2 text-center text-gray-500">"Loading..."</td></tr> }.into_any(),
                        }}
                    </Suspense>
                </tbody>
            </table>
        </div>
    }
}
