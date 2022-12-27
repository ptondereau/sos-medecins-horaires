use std::env;

use sos_medecin_horaires::errors::Result;
use sos_medecin_horaires::sos::{get_horaires, GetHorairesResponse};
use sos_medecin_horaires::telegram::send_message;
use sos_medecin_horaires::AVAILABLES_SLOTS;

fn main() {
    env_logger::init();

    log::info!("Starting SOS Médecin Oise horaires watcher...");

    let _ = start().map_err(|e| {
        log::error!("{}", e);
    });
}

fn start() -> Result<()> {
    let token = env::var("BOT_TOKEN")?;
    let chat_id = env::var("CHAT_ID")?;

    let GetHorairesResponse { list_horaires, .. } = get_horaires()?;

    if let Some(horaires) = list_horaires {
        if !horaires.is_empty() {
            log::info!("Créneaux disponibles !");
            send_message(AVAILABLES_SLOTS.to_string(), token.as_str(), chat_id)?;
        } else {
            log::info!("Pas de créneaux");
        }
    }

    Ok(())
}
