use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        // Đưa Router ra ngoài cùng
        <Router>
            <div class="flex flex-col h-screen bg-gray-50 max-w-md mx-auto border-x shadow-xl">
                <header class="bg-blue-700 text-white p-4">"SME POS"</header>
                <main class="flex-1 overflow-y-auto">
                    <Routes>
                        <Route path="/" view=Home />
                        <Route path="/cart" view=Cart />
                        <Route path="/settings" view=Settings />
                    </Routes>
                </main>

                <nav class="h-16 bg-white border-t flex justify-around items-center">
                    <A href="/" class="flex flex-col items-center">
                        "🏠"
                    </A>
                    <A href="/cart" class="flex flex-col items-center">
                        "🛒"
                    </A>
                    <A href="/settings" class="flex flex-col items-center">
                        "⚙️"
                    </A>
                </nav>
            </div>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView { view! { <div>"Màn hình bán hàng"</div> } }

#[component]
fn Cart() -> impl IntoView { view! { <div>"Giỏ hàng"</div> } }

#[component]
fn Settings() -> impl IntoView { view! { <div>"Cài đặt hệ thống"</div> } }

