use leptos::*;
use crate::security::route_guard::RouteGuard;
use crate::state::auth::AuthState;

#[component]
pub fn CalendarPage() -> impl IntoView {
    let auth = use_context::<AuthState>().expect("AuthState");
    let view_mode = create_rw_signal("day".to_string());
    let current_date = create_rw_signal(chrono::Local::now().format("%Y-%m-%d").to_string());
    let store_filter = create_rw_signal(auth.store_id.get_untracked().unwrap_or_default());
    let stores = create_rw_signal(Vec::<crate::api::types::Store>::new());
    let status_filter = create_rw_signal(vec!["available".to_string(), "reserved".to_string(), "on-rent".to_string()]);
    let calendar_data = create_rw_signal(Option::<crate::api::types::CalendarResponse>::None);
    let loading = create_rw_signal(false);

    let load_calendar = move || {
        let date = current_date.get();
        let store = store_filter.get();
        let view = view_mode.get();
        let statuses = status_filter.get().join(",");
        loading.set(true);

        spawn_local(async move {
            let path = format!("/calendar?store_id={}&date={}&view={}&asset_status={}", store, date, view, statuses);
            if let Ok(json) = crate::api::client::api_get(&path).await {
                if let Ok(data) = serde_wasm_bindgen::from_value(json) {
                    calendar_data.set(Some(data));
                }
            }
            loading.set(false);
        });
    };

    // Load on mount
    create_effect(move |_| {
        load_calendar();
        spawn_local(async move {
            if let Ok(json) = crate::api::client::api_get("/stores").await {
                if let Ok(data) = serde_wasm_bindgen::from_value::<serde_json::Value>(json) {
                    if let Some(arr) = data.get("stores") {
                        if let Ok(list) = serde_json::from_value(arr.clone()) {
                            stores.set(list);
                        }
                    }
                }
            }
        });
    });

    view! {
        <RouteGuard required_role="MerchantStaff">
            <h1>"Availability Calendar"</h1>

            <div class="filters">
                <div class="form-group" style="margin-bottom: 0;">
                    <label>"Store"</label>
                    <select on:change=move |ev| { store_filter.set(event_target_value(&ev)); load_calendar(); }>
                        <For
                            each=move || stores.get()
                            key=|s| s.id.clone()
                            children=move |s| view! { <option value=s.id.clone()>{s.name.clone()}</option> }
                        />
                    </select>
                </div>

                <div class="form-group" style="margin-bottom: 0;">
                    <label>"Date"</label>
                    <input type="date" prop:value=move || current_date.get()
                        on:change=move |ev| { current_date.set(event_target_value(&ev)); load_calendar(); } />
                </div>
            </div>

            <div class="view-toggle">
                <button class=move || format!("btn {}", if view_mode.get() == "day" { "active" } else { "" })
                    on:click=move |_| { view_mode.set("day".into()); load_calendar(); }>"Day"</button>
                <button class=move || format!("btn {}", if view_mode.get() == "week" { "active" } else { "" })
                    on:click=move |_| { view_mode.set("week".into()); load_calendar(); }>"Week"</button>
            </div>

            <div class="filters">
                <label style="font-weight: 500; font-size: 0.875rem;">"Filter: "</label>
                {["available", "reserved", "on-rent", "in-repair"].iter().map(|s| {
                    let status = s.to_string();
                    let sf = status_filter.clone();
                    view! {
                        <label style="display: flex; align-items: center; gap: 0.25rem; font-size: 0.875rem;">
                            <input type="checkbox" checked=true
                                on:change=move |_| {
                                    sf.update(|v| {
                                        if v.contains(&status) { v.retain(|x| x != &status); }
                                        else { v.push(status.clone()); }
                                    });
                                }
                            />
                            {*s}
                        </label>
                    }
                }).collect_view()}
            </div>

            <div class="card">
                <Show when=move || loading.get()>
                    <p>"Loading calendar..."</p>
                </Show>

                <Show when=move || calendar_data.get().is_some()>
                    {move || {
                        let data = calendar_data.get().unwrap();
                        let time_slots = crate::utils::time::generate_time_slots(7, 19, 15);
                        view! {
                            <p style="font-size: 0.875rem; color: #6b7280; margin-bottom: 0.5rem;">
                                "Business hours: " {data.business_hours.start.clone()} " - " {data.business_hours.end.clone()}
                                " | 15-minute increments"
                            </p>
                            <div class="calendar-grid" style="grid-template-columns: 80px 1fr;">
                                {time_slots.iter().map(|slot| {
                                    let slot_time = format!("{}T{}:00", data.date, slot);
                                    let has_reservations = data.slots.iter()
                                        .any(|s| s.time == slot_time && !s.reservations.is_empty());
                                    view! {
                                        <div class="calendar-slot" style="font-weight: 500; background: #f9fafb;">
                                            {crate::utils::format::format_time_12h(slot)}
                                        </div>
                                        <div class=move || format!("calendar-slot {}", if has_reservations { "occupied" } else { "" })>
                                            {data.slots.iter()
                                                .filter(|s| s.time == slot_time)
                                                .flat_map(|s| s.reservations.iter())
                                                .map(|r| view! {
                                                    <span style="font-size: 0.7rem; background: #93c5fd; padding: 0.1rem 0.25rem; border-radius: 0.2rem; margin-right: 0.25rem;">
                                                        {r.asset_name.clone()} " (" {r.user_display_name.clone()} ")"
                                                    </span>
                                                }).collect_view()
                                            }
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }
                    }}
                </Show>
            </div>
        </RouteGuard>
    }
}
