use crate::application::messaging::outbound::dto::{NucleoRejectedDto, NucleoRetrievedDto};
use crate::domain::model::enums::ReasonCode;
use crate::domain::model::messaging::NucleoRetrieveCommand;

pub struct NucleoCallbackMapper;

impl NucleoCallbackMapper {
    pub fn retrieved(command: &NucleoRetrieveCommand, nome: String) -> NucleoRetrievedDto {
        NucleoRetrievedDto {
            request_event_id: command.event_id,
            source_event_id: command.source_event_id,
            nucleo_id: command.nucleo_id,
            nome,
        }
    }

    pub fn rejected(command: &NucleoRetrieveCommand) -> NucleoRejectedDto {
        NucleoRejectedDto {
            request_event_id: command.event_id,
            source_event_id: command.source_event_id,
            nucleo_id: command.nucleo_id,
            reason_code: ReasonCode::NucleoNotFound.code().to_owned(),
            message: ReasonCode::NucleoNotFound.message().to_owned(),
        }
    }
}
