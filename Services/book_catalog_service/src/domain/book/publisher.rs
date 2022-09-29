use crate::domain::core::errors::ValidationError;
use crate::domain::core::vvos::RblStringVvo;
use crate::infrastructure::book::book_model::PublisherModel;

#[derive(Debug)]
pub struct Publisher {
    pub name: RblStringVvo<2, 100>,
    pub address: RblStringVvo<2, 100>,
}

impl TryFrom<PublisherModel> for Publisher {
    type Error = ValidationError;

    fn try_from(value: PublisherModel) -> Result<Self, Self::Error> {
        let name = RblStringVvo::try_from(value.name)
            .map_err(|e| ValidationError {
            message: format!("PublisherModel.name is invalid: {}", e.message.as_str())
        })?;
        let address = RblStringVvo::try_from(value.address)
            .map_err(|e| ValidationError {
            message: format!("PublisherModel.address is invalid: {}", e.message.as_str())
        })?;

        Ok(Publisher {
            name,
            address,
        })
    }
}