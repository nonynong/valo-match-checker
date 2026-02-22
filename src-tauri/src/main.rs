#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    LogicalPosition, Manager, Runtime,
};
use tokio::time::{interval, Duration};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ApiResponse {
    status: String,
    data: ApiData,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ApiData {
    status: u16,
    segments: Vec<MatchSegment>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct MatchSegment {
    team1: String,
    team2: String,
    score1: String,
    score2: String,
    #[serde(default)]
    current_map: String,
    #[serde(default)]
    match_event: String,
    #[serde(default)]
    match_series: String,
    #[serde(default)]
    time_until_match: String,
    #[serde(default)]
    flag1: String,
    #[serde(default)]
    flag2: String,
    #[serde(default)]
    team1_logo: String,
    #[serde(default)]
    team2_logo: String,
    #[serde(default)]
    team1_round_ct: String,
    #[serde(default)]
    team1_round_t: String,
    #[serde(default)]
    team2_round_ct: String,
    #[serde(default)]
    team2_round_t: String,
    #[serde(default)]
    map_number: String,
    #[serde(default)]
    unix_timestamp: String,
    #[serde(default)]
    match_page: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PolymarketOdds {
    team1_odds: Option<f64>,
    team2_odds: Option<f64>,
    market_url: Option<String>,
}

// Test mode flag - set to true to use mock data
const USE_TEST_DATA: bool = false; // Change to false to use real API

fn get_test_matches() -> Vec<MatchSegment> {
    vec![
        MatchSegment {
            team1: "Sentinels".to_string(),
            team2: "100 Thieves".to_string(),
            score1: "13".to_string(),
            score2: "9".to_string(),
            current_map: "Ascent".to_string(),
            match_event: "VCT 2025: Americas Stage 1".to_string(),
            match_series: "Regular Season".to_string(),
            time_until_match: "LIVE".to_string(),
            flag1: "flag_us".to_string(),
            flag2: "flag_us".to_string(),
            team1_logo: "https://owcdn.net/img/62e7a0e8f1c0b.png".to_string(),
            team2_logo: "https://owcdn.net/img/62e7a0e8f1c0b.png".to_string(),
            team1_round_ct: "7".to_string(),
            team1_round_t: "6".to_string(),
            team2_round_ct: "5".to_string(),
            team2_round_t: "4".to_string(),
            map_number: "1".to_string(),
            unix_timestamp: "1713996000".to_string(),
            match_page: "https://www.vlr.gg/12345".to_string(),
        },
        MatchSegment {
            team1: "Fnatic".to_string(),
            team2: "Team Liquid".to_string(),
            score1: "7".to_string(),
            score2: "5".to_string(),
            current_map: "Bind".to_string(),
            match_event: "VCT 2025: EMEA Stage 1".to_string(),
            match_series: "Regular Season".to_string(),
            time_until_match: "LIVE".to_string(),
            flag1: String::new(),
            flag2: String::new(),
            team1_logo: String::new(),
            team2_logo: String::new(),
            team1_round_ct: String::new(),
            team1_round_t: String::new(),
            team2_round_ct: String::new(),
            team2_round_t: String::new(),
            map_number: "2".to_string(),
            unix_timestamp: String::new(),
            match_page: String::new(),
        },
        MatchSegment {
            team1: "Paper Rex".to_string(),
            team2: "DRX".to_string(),
            score1: "10".to_string(),
            score2: "8".to_string(),
            current_map: "Icebox".to_string(),
            match_event: "VCT 2025: Pacific Stage 1".to_string(),
            match_series: "Regular Season".to_string(),
            time_until_match: "LIVE".to_string(),
            flag1: String::new(),
            flag2: String::new(),
            team1_logo: String::new(),
            team2_logo: String::new(),
            team1_round_ct: String::new(),
            team1_round_t: String::new(),
            team2_round_ct: String::new(),
            team2_round_t: String::new(),
            map_number: "1".to_string(),
            unix_timestamp: String::new(),
            match_page: String::new(),
        },
        MatchSegment {
            team1: "LOUD".to_string(),
            team2: "KRÜ Esports".to_string(),
            score1: "6".to_string(),
            score2: "6".to_string(),
            current_map: "Lotus".to_string(),
            match_event: "VCT 2025: Americas Stage 1".to_string(),
            match_series: "Regular Season".to_string(),
            time_until_match: "LIVE".to_string(),
            flag1: String::new(),
            flag2: String::new(),
            team1_logo: String::new(),
            team2_logo: String::new(),
            team1_round_ct: String::new(),
            team1_round_t: String::new(),
            team2_round_ct: String::new(),
            team2_round_t: String::new(),
            map_number: "3".to_string(),
            unix_timestamp: String::new(),
            match_page: String::new(),
        },
        MatchSegment {
            team1: "G2 Esports".to_string(),
            team2: "KOI".to_string(),
            score1: "12".to_string(),
            score2: "11".to_string(),
            current_map: "Split".to_string(),
            match_event: "VCT 2025: EMEA Stage 1".to_string(),
            match_series: "Regular Season".to_string(),
            time_until_match: "LIVE".to_string(),
            flag1: String::new(),
            flag2: String::new(),
            team1_logo: String::new(),
            team2_logo: String::new(),
            team1_round_ct: String::new(),
            team1_round_t: String::new(),
            team2_round_ct: String::new(),
            team2_round_t: String::new(),
            map_number: "2".to_string(),
            unix_timestamp: String::new(),
            match_page: String::new(),
        },
    ]
}

async fn fetch_live_matches() -> Result<Vec<MatchSegment>, Box<dyn std::error::Error>> {
    // Use test data if flag is set
    if USE_TEST_DATA {
        return Ok(get_test_matches());
    }

    // Otherwise fetch from real API
    let url = "https://vlrggapi.vercel.app/v2/match?q=live_score";
    let response = reqwest::get(url).await?;
    let api_response: ApiResponse = response.json().await?;

    Ok(api_response.data.segments)
}

fn format_match_text(segment: &MatchSegment) -> String {
    let score = format!("{} - {}", segment.score1, segment.score2);
    let teams = format!("{} vs {}", segment.team1, segment.team2);
    let map = if segment.current_map.is_empty() {
        "Unknown Map".to_string()
    } else {
        segment.current_map.clone()
    };
    let status = if segment.time_until_match == "LIVE" {
        "LIVE".to_string()
    } else if !segment.time_until_match.is_empty() {
        segment.time_until_match.clone()
    } else {
        "".to_string()
    };

    if status == "LIVE" {
        format!("{} | {} | {}", teams, score, map)
    } else if !status.is_empty() {
        format!("{} | {} | {}", teams, score, status)
    } else {
        format!("{} | {}", teams, score)
    }
}

// Convert team name to Polymarket slug format (lowercase, remove spaces, common abbreviations)
fn team_to_slug(team: &str) -> String {
    let team_lower = team.to_lowercase();
    // Common team abbreviations mapping
    let abbreviations: HashMap<&str, &str> = [
        ("100 thieves", "100t"),
        ("100t", "100t"),
        ("mibr", "mibr"),
        ("nrg", "nrg"),
        ("g2 esports", "g2"),
        ("g2", "g2"),
        ("sentinels", "sentinels"),
        ("fnatic", "fnatic"),
        ("team liquid", "tl"),
        ("paper rex", "prx"),
        ("loud", "loud"),
        ("kru esports", "kru"),
        ("kru", "kru"),
        ("koi", "koi"),
        ("drx", "drx"),
    ]
    .iter()
    .cloned()
    .collect();
    
    if let Some(abbr) = abbreviations.get(team_lower.as_str()) {
        return abbr.to_string();
    }
    
    // Otherwise, convert to slug: lowercase, remove spaces and special chars
    team_lower
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

// Format date for Polymarket URL (YYYY-MM-DD)
fn format_date_for_polymarket() -> String {
    use chrono::Local;
    Local::now().format("%Y-%m-%d").to_string()
}

// Fetch Polymarket odds for a match
async fn fetch_polymarket_odds(team1: &str, team2: &str) -> Result<PolymarketOdds, Box<dyn std::error::Error>> {
    let team1_slug = team_to_slug(team1);
    let team2_slug = team_to_slug(team2);
    let date = format_date_for_polymarket();
    
    // Try searching for the market using team names
    let search_query = format!("{} vs {}", team1, team2);
    let search_url = format!(
        "https://gamma-api.polymarket.com/public-search?q={}&limit=10",
        urlencoding::encode(&search_query)
    );
    
    let client = reqwest::Client::new();
    let response = client
        .get(&search_url)
        .header("User-Agent", "Valorant-Menubar-App")
        .send()
        .await?;
    
    if !response.status().is_success() {
        // If search fails, return empty odds with URL
        let market_url = format!("https://polymarket.com/sports/valorant/games/week/1/val-{}-{}-{}", 
            team1_slug, team2_slug, date);
        return Ok(PolymarketOdds {
            team1_odds: None,
            team2_odds: None,
            market_url: Some(market_url),
        });
    }
    
    let search_results: serde_json::Value = response.json().await?;
    
    // Try to find a market matching the teams
    if let Some(results) = search_results.get("results").and_then(|r| r.as_array()) {
        for result in results {
            if let Some(question) = result.get("question").and_then(|q| q.as_str()) {
                let question_lower = question.to_lowercase();
                if question_lower.contains(&team1.to_lowercase()) && 
                   question_lower.contains(&team2.to_lowercase()) {
                    // Found a matching market
                    if let Some(outcomes) = result.get("outcomes").and_then(|o| o.as_array()) {
                        let mut team1_odds = None;
                        let mut team2_odds = None;
                        
                        for outcome in outcomes {
                            if let (Some(title), Some(price_str)) = (
                                outcome.get("title").and_then(|t| t.as_str()),
                                outcome.get("price").and_then(|p| p.as_str()),
                            ) {
                                let price = price_str.parse::<f64>().ok();
                                let title_lower = title.to_lowercase();
                                
                                if title_lower.contains(&team1.to_lowercase()) {
                                    team1_odds = price;
                                } else if title_lower.contains(&team2.to_lowercase()) {
                                    team2_odds = price;
                                }
                            }
                        }
                        
                        let slug = result.get("slug").and_then(|s| s.as_str()).map(|s| s.to_string());
                        let market_url = slug.map(|s| format!("https://polymarket.com/{}", s))
                            .or_else(|| Some(format!("https://polymarket.com/sports/valorant/games/week/1/val-{}-{}-{}", 
                                team1_slug, team2_slug, date)));
                        
                        return Ok(PolymarketOdds {
                            team1_odds,
                            team2_odds,
                            market_url,
                        });
                    }
                }
            }
        }
    }
    
    // If no market found, return URL only
    let market_url = format!("https://polymarket.com/sports/valorant/games/week/1/val-{}-{}-{}", 
        team1_slug, team2_slug, date);
    Ok(PolymarketOdds {
        team1_odds: None,
        team2_odds: None,
        market_url: Some(market_url),
    })
}

// Tauri command to get live matches (called from React)
#[tauri::command]
async fn get_live_matches() -> Result<Vec<MatchSegment>, String> {
    fetch_live_matches().await.map_err(|e| e.to_string())
}

// Dummy Polymarket odds for UI preview (e.g. "Will Jesus Christ return before 2027?" — 4% Yes / 96% No)
fn get_dummy_polymarket_odds() -> PolymarketOdds {
    PolymarketOdds {
        team1_odds: Some(0.04),
        team2_odds: Some(0.96),
        market_url: Some("https://polymarket.com/event/will-jesus-christ-return-before-2027".to_string()),
    }
}

// Tauri command to get Polymarket odds for a match
#[tauri::command]
async fn get_polymarket_odds(team1: String, team2: String) -> Result<PolymarketOdds, String> {
    if USE_TEST_DATA {
        return Ok(get_dummy_polymarket_odds());
    }
    fetch_polymarket_odds(&team1, &team2).await.map_err(|e| e.to_string())
}

// Position the window near the menu bar (top-right)
fn position_window_near_menu_bar<R: Runtime>(window: &tauri::WebviewWindow<R>) {
    if let Ok(Some(monitor)) = window.primary_monitor() {
        let screen_size = monitor.size();
        let scale_factor = monitor.scale_factor();
        
        // Calculate position: top-right corner, below menu bar
        // Menu bar is typically ~25-30px tall on macOS
        let menu_bar_height = 30.0;
        let window_width = 420.0;
        let _window_height = 260.0;
        
        // Position at top-right, accounting for scale factor
        let x = (screen_size.width as f64 / scale_factor) - window_width - 10.0; // 10px margin from right edge
        let y = menu_bar_height + 5.0; // 5px below menu bar
        
        let _ = window.set_position(LogicalPosition::new(x, y));
    } else {
        // Fallback: center at top of screen
        let _ = window.center();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_live_matches, get_polymarket_odds])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Create tray icon
            let icon = app_handle.default_window_icon().cloned();
            let mut builder = TrayIconBuilder::new().tooltip("Valorant: Loading...");

            if let Some(icon) = icon {
                builder = builder.icon(icon);
            }

            // Create a minimal menu (just quit option) since we're using custom React window
            // The React window will show all the match information
            let minimal_menu = MenuBuilder::new(&app_handle)
                .item(&MenuItem::with_id(&app_handle, "quit", "Quit", true, None::<&str>)?)
                .build()?;
            builder = builder.menu(&minimal_menu);

            // Store tray reference (populated after build)
            let tray_ref: Arc<Mutex<Option<TrayIcon<_>>>> = Arc::new(Mutex::new(None));

            // Build the tray icon with event handlers
            // Note: macOS automatically keeps menus open while hovering over the tray icon
            // The Enter/Move/Leave events help track hover state for custom behavior
            let tray = builder
                .on_tray_icon_event(move |tray_icon, event| {
                        match event {
                            TrayIconEvent::Click { .. } => {
                                // On click, open/show the React popover window
                                let app = tray_icon.app_handle();
                                
                                if let Some(window) = app.get_webview_window("main") {
                                    let is_visible = window.is_visible().unwrap_or(false);
                                    if is_visible {
                                        // Hide window if already visible
                                        let _ = window.hide();
                                    } else {
                                        // Show and position window
                                        position_window_near_menu_bar(&window);
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                    }
                                } else {
                                    // Create window if it doesn't exist
                                    if let Some(window_config) = app.config().app.windows.iter()
                                        .find(|w| w.label == "main") {
                                        if let Ok(window) = tauri::WebviewWindowBuilder::from_config(app, window_config)
                                            .and_then(|b| b.build()) {
                                            // Position window near menu bar
                                            position_window_near_menu_bar(&window);
                                            let _ = window.show();
                                            let _ = window.set_focus();
                                        }
                                    }
                                }
                            }
                            TrayIconEvent::Enter { .. } => {
                                // Mouse entered tray icon - menu will stay open while hovering
                                // macOS handles this automatically - menu stays open as long as mouse is over tray or menu
                                eprintln!("Mouse entered tray icon - menu stays open while hovering");
                            }
                            TrayIconEvent::Move { .. } => {
                                // Mouse moving over tray icon - menu stays open
                                // macOS keeps menu open while hovering
                            }
                            TrayIconEvent::Leave { .. } => {
                                // Mouse left tray icon area
                                eprintln!("Mouse left tray icon");
                            }
                            _ => {}
                        }
                })
                .on_menu_event({
                    move |app, event| {
                        match event.id.as_ref() {
                            "quit" => {
                                app.exit(0);
                            }
                            _ => {}
                        }
                    }
                })
                .build(&app_handle)?;
            
            // Store tray reference for menu refresh handler
            *tray_ref.lock().unwrap() = Some(tray.clone());

            // Update tooltip with match info
            let tooltip_text = if USE_TEST_DATA {
                if let Some(first_match) = get_test_matches().first() {
                    format!("Valorant: {}", format_match_text(first_match))
                } else {
                    "Valorant: No matches".to_string()
                }
            } else {
                "Valorant: Loading...".to_string()
            };
            let _ = tray.set_tooltip(Some(tooltip_text.as_str()));

            // Set up periodic tooltip update every 30 seconds
            // The React window will handle its own refresh via the get_live_matches command
            let tray_for_tooltip = tray.clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    match fetch_live_matches().await {
                        Ok(matches) => {
                            if let Some(first_match) = matches.first() {
                                let tooltip_text = format!("Valorant: {}", format_match_text(first_match));
                                let _ = tray_for_tooltip.set_tooltip(Some(tooltip_text.as_str()));
                            }
                        }
                        Err(e) => {
                            eprintln!("Error fetching matches: {}", e);
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
