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

#[cfg(test)]
mod publisher_entity_unit_tests {
    use domain_patterns::models::ValueObject;
    use crate::domain::book::publisher::Publisher;
    use crate::infrastructure::book::book_model::PublisherModel;

    #[test]
    fn when_trying_to_create_a_publisher_entity_with_invalid_name_then_an_error_is_returned() {
        // Arrange
        let publisher_model = PublisherModel {
            name: "".to_string(),
            address: "123 Main St.".to_string(),
        };

        // Act
        let result = Publisher::try_from(publisher_model);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().message, "PublisherModel.name is invalid: Length must be between 2 and 100 characters long");
    }

    #[test]
    fn when_trying_to_create_a_publisher_entity_with_invalid_address_then_an_error_is_returned() {
        // Arrange
        let publisher_model = PublisherModel {
            name: "Acme Publishing".to_string(),
            address: "".to_string(),
        };

        // Act
        let result = Publisher::try_from(publisher_model);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().message, "PublisherModel.address is invalid: Length must be between 2 and 100 characters long");
    }

    #[test]
    fn when_trying_to_create_a_publisher_entity_with_valid_name_and_address_then_no_error_is_returned() {
        // Arrange
        let publisher_model = PublisherModel {
            name: "Acme Publishing".to_string(),
            address: "123 Main St.".to_string(),
        };

        // Act
        let result = Publisher::try_from(publisher_model);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.as_ref().unwrap().name.value(), "Acme Publishing");
        assert_eq!(result.as_ref().unwrap().address.value(), "123 Main St.");
    }
}