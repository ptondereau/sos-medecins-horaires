use crate::errors::{Errors, Result, SOSMedecinsApiErrors};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetHorairesResponse {
    success: bool,
    pub list_horaires: Option<Vec<serde_json::Value>>,
    //code_status: Option<u64>,
    //message: Option<String>,
}

pub fn get_horaires() -> Result<GetHorairesResponse> {
    let date = chrono::Local::now().format("%d/%m/%Y+%H:%M:%S").to_string();
    log::debug!("Sent date: {}", &date);

    let response = ureq::post("https://beauvais.rdvasos.fr/SOS50/Horaires/GetHoraires")
        .set("Cookie", "ASP.NET_SessionId=makksbfyyvyzqg5h4xjhrhnv; __RequestVerificationToken_L1NPUzUw0=r4FpkjqYNYpL1_ovZ4bqyd1Ivdn_nwh83TSLVj5nHGDGiqe4fg8obPGxRCMmPL1PTIoaeI_FPmw6fwJitg35pN0GgkEWMRXhtGHW81wmp-E1")
    .send_form(&[
        ("service", "60"),
        ("site", "35"),
        ("action", "suiv"),
        ("date", date.as_str())
    ])?;

    let deser = response.into_json::<GetHorairesResponse>().unwrap();

    if !deser.success {
        return Err(Errors::SOSMedecinsApiError(
            SOSMedecinsApiErrors::NotSuccess,
        ));
    }

    Ok(deser)
}
