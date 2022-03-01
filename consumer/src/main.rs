/// Custom type
#[derive(PartialEq, Eq, Debug)]
struct Byte(u8);

/// Another custom type
#[derive(PartialEq, Eq, Debug)]
struct Boolean(bool);

/// ByteService will use an external dependency (Provider).
/// To mock the external calls, we declare the ByteService interface
/// and specify separately the implementation.
#[cfg_attr(test, mockall::automock)]
trait ByteService {
    fn is_zero(&self, byte: Byte) -> Boolean;
}

/// Concrete implementation of the ByteService, using the external dependency.
/// It handles converting to and from the external types.
/// Should the library change, only this Adapter will need updating.
struct ProviderAdapter;
impl ByteService for ProviderAdapter {
    fn is_zero(&self, byte: Byte) -> Boolean {
        let provider_payload = provider::Payload(byte.0);
        let provider_outcome = provider::functionality(provider_payload);
        Boolean(provider_outcome.0)
    }
}

/// Another service.
/// This does not use external dependencies, so we could use it statically
/// or dynamically (depending on our unit testing needs).
struct BooleanService;
impl BooleanService {
    fn is_true(&self, boolean: Boolean) -> bool {
        boolean.0
    }
}

/// Main state holder. It holds all the necessary services.
struct Application {
    byte_service: Box<dyn ByteService>,
    boolean_service: BooleanService,
}
impl Application {
    fn new(byte_service: Box<dyn ByteService>) -> Self {
        Self {
            byte_service,
            boolean_service: BooleanService,
        }
    }
}


/// Bin entrypoint.
fn main() {
    let app = Application::new( Box::new(ProviderAdapter));
    let is_zero = app.byte_service.is_zero(Byte(0));
    match app.boolean_service.is_true(is_zero) {
        true => println!("All good."),
        false => panic!("Whoops."),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unit tests for Application without mocking external dependencies.
    /// Since we don't control how expensive the external calls are,
    /// we rarely want to do this.
    #[test]
    fn without_mocks() {
        let app = Application::new( Box::new(ProviderAdapter));
        assert_eq!(app.byte_service.is_zero(Byte(0)), Boolean(true));
        assert_eq!(app.byte_service.is_zero(Byte(1)), Boolean(false));
    }

    /// Unit tests for Application mocking external dependencies.
    /// We are not really mocking the external calls themselves,
    /// rather the crate's adapter - which returns ad hoc values directly.
    #[test]
    fn with_mocks() {
        let mut mock = MockByteService::new();
        mock.expect_is_zero()
            .with(mockall::predicate::eq(Byte(0)))
            .times(1)
            .returning(|_| Boolean(false));
        mock.expect_is_zero()
            .with(mockall::predicate::eq(Byte(1)))
            .times(1)
            .returning(|_| Boolean(true));
        let app = Application::new( Box::new(mock));
        assert_eq!(app.byte_service.is_zero(Byte(0)), Boolean(false));
        assert_eq!(app.byte_service.is_zero(Byte(1)), Boolean(true));
    }
}
