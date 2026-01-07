use leptos::prelude::*;
use rs_design::{VirtualTable, VirtualRow, VirtualColumn, ColumnAlign, Input, Select};

#[derive(Clone, Copy, Debug, PartialEq)]
enum LogLevel {
    All,
    Error,
    Warn,
    Info,
}

#[component]
pub fn LogsViewerDemo() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let level_value = RwSignal::new("All".to_string());
    let level_filter = RwSignal::new(LogLevel::All);
    
    // Gerar logs
    let all_logs: Vec<(String, String, String, String)> = (0..100_000)
        .map(|i| {
            let level = if i % 100 == 0 { 
                "ERROR" 
            } else if i % 10 == 0 { 
                "WARN" 
            } else { 
                "INFO" 
            };
            
            let service = match i % 5 {
                0 => "api.gateway",
                1 => "auth.service",
                2 => "payment.processor",
                3 => "notification.hub",
                _ => "database.pool",
            };
            
            let messages = [
                "Request processed successfully",
                "Database query executed",
                "Cache miss - fetching from origin",
                "Rate limit exceeded for client",
            ];
            
            (
                format!("2024-12-30 {:02}:{:02}:{:02}.{:03}", 
                    (i / 3600) % 24, (i / 60) % 60, i % 60, i % 1000),
                level.to_string(),
                service.to_string(),
                format!("{} - correlation_id={}", messages[i % messages.len()], i),
            )
        })
        .collect();
    
    let filtered_logs = move || {
        let query = search.get().to_lowercase();
        let level = level_filter.get();
        
        if query.len() > 0 && query.len() < 3 {
            return Vec::new();
        }
        
        all_logs
            .iter()
            .filter(|(_, log_level, _, message)| {
                let level_match = match level {
                    LogLevel::All => true,
                    LogLevel::Error => log_level == "ERROR",
                    LogLevel::Warn => log_level == "WARN",
                    LogLevel::Info => log_level == "INFO",
                };
                
                let search_match = query.is_empty() || 
                    message.to_lowercase().contains(&query);
                
                level_match && search_match
            })
            .enumerate()
            .map(|(idx, (timestamp, level, service, message))| {
                VirtualRow {
                    index: idx,
                    data: vec![
                        timestamp.clone(),
                        level.clone(),
                        service.clone(),
                        message.clone(),
                    ],
                }
            })
            .collect::<Vec<_>>()
    };
    
    let rows = Signal::derive(filtered_logs);
    
    let columns = vec![
        VirtualColumn {
            key: "Timestamp".to_string(),
            width: Some(200),
            flex: None,
            align: ColumnAlign::Left,
            resizable: false,
        },
        VirtualColumn {
            key: "Level".to_string(),
            width: Some(80),
            flex: None,
            align: ColumnAlign::Center,
            resizable: false,
        },
        VirtualColumn {
            key: "Service".to_string(),
            width: Some(150),
            flex: None,
            align: ColumnAlign::Left,
            resizable: false,
        },
        VirtualColumn {
            key: "Message".to_string(),
            width: None,
            flex: Some(1.0),
            align: ColumnAlign::Left,
            resizable: false,
        },
    ];

    view! {
        <div class="space-y-4">
            <div class="flex items-center justify-between">
                <div>
                    <h2 class="text-2xl font-bold">"Production Logs Viewer"</h2>
                    <p class="text-sm text-muted-foreground">
                        "Real-time monitoring • 100k entries"
                    </p>
                </div>
                <div class="flex items-center gap-4">
                    <div class="text-right">
                        <div class="text-2xl font-bold">
                            {move || rows.get().len().to_string()}
                        </div>
                        <div class="text-xs text-muted-foreground">"entries"</div>
                    </div>
                    <div class="text-right">
                        <div class="text-2xl font-bold text-red-500">
                            {move || {
                                rows.get()
                                    .iter()
                                    .filter(|r| r.data.get(1).map(|l| l == "ERROR").unwrap_or(false))
                                    .count()
                                    .to_string()
                            }}
                        </div>
                        <div class="text-xs text-muted-foreground">"errors"</div>
                    </div>
                </div>
            </div>
            
            <div class="grid grid-cols-[1fr_auto] items-center gap-3 p-4 border border-border rounded-lg bg-muted/30">
                <div class="flex-1">
                    <Input
                        value=search
                        placeholder="Search logs... (min 3 characters)"
                        class="w-full"
                    />
                    {move || {
                        let query = search.get();
                        if query.len() > 0 && query.len() < 3 {
                            view! {
                                <p class="text-xs text-yellow-600 mt-1">
                                    "Type at least 3 characters"
                                </p>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }}
                </div>
                
                <Select
                    value=level_value
                    on_change=Callback::new(move |value: String| {
                        level_filter.set(match value.as_str() {
                            "ERROR" => LogLevel::Error,
                            "WARN" => LogLevel::Warn,
                            "INFO" => LogLevel::Info,
                            _ => LogLevel::All,
                        });
                    })
                    class="min-w-[120px]"
                >
                    <option value="All">"All Levels"</option>
                    <option value="ERROR">"ERROR Only"</option>
                    <option value="WARN">"WARN Only"</option>
                    <option value="INFO">"INFO Only"</option>
                </Select>
            </div>
            
            <VirtualTable
                rows=rows
                columns=columns
                row_height=36.0
                viewport_height=600.0
                overscan=10
                empty_message="Type at least 3 characters or adjust filter.".to_string()
            />
            
            <div class="text-xs text-muted-foreground text-center py-2">
                "Rendering " 
                {move || rows.get().len().to_string()} 
                " logs • O(1) DOM • Canon Input + Select + VirtualTable"
            </div>
        </div>
    }
}
