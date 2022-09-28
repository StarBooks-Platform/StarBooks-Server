use crate::domain::core::errors::ValidationError;
use crate::domain::core::vvos::RblStringVVO;
use crate::infrastructure::book_model::PublisherModel;

#[derive(Debug)]
pub struct Publisher {
    pub name: RblStringVVO<2, 100>,
    pub address: RblStringVVO<2, 100>,
}

impl TryFrom<PublisherModel> for Publisher {
    type Error = ValidationError;

    fn try_from(value: PublisherModel) -> Result<Self, Self::Error> {
        let name = RblStringVVO::try_from(value.name)
            .map_err(|e| ValidationError {
            message: format!("PublisherModel.name is invalid: {}", e.message.as_str())
        })?;
        let address = RblStringVVO::try_from(value.address)
            .map_err(|e| ValidationError {
            message: format!("PublisherModel.address is invalid: {}", e.message.as_str())
        })?;

        Ok(Publisher {
            name,
            address,
        })
    }
}