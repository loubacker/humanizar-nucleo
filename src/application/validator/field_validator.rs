use crate::domain::exception::NucleoException;
use crate::domain::model::enums::ReasonCode;

pub(crate) fn required(
    value: Option<String>,
    reason: ReasonCode,
) -> Result<String, NucleoException> {
    let value = value.ok_or_else(|| NucleoException::new(reason))?;
    if value.trim().is_empty() {
        return Err(NucleoException::new(reason));
    }
    Ok(value)
}
