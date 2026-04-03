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
            <table class="w-full border border-gray-300">
                <thead>
                    <tr>
                        <th class="border border-gray-300 px-4 py-2 bg-gray-100">"Key"</th>
                        <th class="border border-gray-300 px-4 py-2 bg-gray-100">"Name"</th>
                        <th class="border border-gray-300 px-4 py-2 bg-gray-100">"Expertise"</th>
                        <th class="border border-gray-300 px-4 py-2 bg-gray-100">"Created At"</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-gray-300">
                    <Suspense>
                        {move || match teachers.get() {
                            Some(Ok(teachers)) => {
                                if teachers.is_empty() {
                                    view! { <tr><td colspan="4" class="px-4 py-2 text-center">"No teachers found"</td></tr> }.into_any()
                                } else {
                                    teachers
                                        .into_iter()
                                        .map(|teacher| {
                                            let created_at = teacher.formatted_created_at();

                                            view! {
                                                <tr>
                                                    <td class="px-4 py-2">{teacher.id_key()}</td>
                                                    <td class="px-4 py-2">{teacher.name}</td>
                                                    <td class="px-4 py-2">{teacher.expertise}</td>
                                                    <td class="px-4 py-2">{created_at}</td>
                                                </tr>
                                            }
                                        })
                                        .collect_view()
                                        .into_any()
                                }
                            }
                            Some(Err(e)) => view! { <tr><td colspan="4" class="px-4 py-2 text-center text-red-500">{format!("Error: {}", e)}</td></tr> }.into_any(),
                            None => view! { <tr><td colspan="4" class="px-4 py-2 text-center text-gray-500">"Loading..."</td></tr> }.into_any(),
                        }}
                    </Suspense>
                </tbody>
            </table>
            <div class="mt-8 p-4 bg-gray-50 rounded border border-gray-200">
                <h2 class="text-xl font-semibold mb-2">"Git SSH Setup"</h2>
                <ol class="list-decimal list-inside space-y-2 text-sm text-gray-700">
                    <li>
                        <code class="font-mono bg-gray-200 px-1 rounded">"ssh-keygen -t ed25519 -C \"your-email@example.com\""</code>
                    </li>
                    <li>
                        <code class="font-mono bg-gray-200 px-1 rounded">"cat ~/.ssh/id_ed25519.pub"</code>
                    </li>
                    <li>"Add the key to GitHub: "<a href="https://github.com/settings/keys" class="text-blue-600 underline" target="_blank">"https://github.com/settings/keys"</a></li>
                    <li>
                        <code class="font-mono bg-gray-200 px-1 rounded">"git remote set-url origin git@github.com:mrpanam/mysite.git"</code>
                    </li>
                    <li>
                        <code class="font-mono bg-gray-200 px-1 rounded">"git push"</code>
                    </li>
                </ol>
                <p class="mt-2 text-xs text-gray-500">"You can have as many SSH keys as you want on your GitHub account."</p>
            </div>
        </div>
    }
}
