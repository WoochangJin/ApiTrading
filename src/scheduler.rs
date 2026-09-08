use chrono::{Local, TimeZone, Duration as ChronoDuration};
use tokio::time::{sleep, Duration};
use crate::{notification, state::AppState};

const TARGET_HOUR: u32 = 18;
const TARGET_MINUTE: u32 = 0;

pub async fn run(state: AppState) {
    loop {
        let wait = duration_until_next_target();
        tracing::info!("next run in {:?}", wait);
        sleep(wait).await;

        if let Err(e) = crate::services::run_daily_cycle(&state).await {
            tracing::error!("daily cycle failed: {e:?}");
            notification::send_discord(
                &state.discord_client, 
                &format!("failed daily cycle: {e:?}"),
            ).await;
        }
    }
}

fn duration_until_next_target() -> Duration {
    let now = Local::now();
    let today_target = now.date_naive()
        .and_hms_opt(TARGET_HOUR, TARGET_MINUTE, 0)
        .unwrap();
    let mut target = Local.from_local_datetime(&today_target).single().unwrap_or(now);

    if target <= now {
        target = target + ChronoDuration::days(1);
    }

    (target - now).to_std().unwrap_or(Duration::from_secs(60))
}