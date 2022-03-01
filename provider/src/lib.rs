/// Provider Input type
pub struct Payload(pub u8);

/// Provider Output type
pub struct Outcome(pub bool);

/// Provider functionality
pub fn functionality(payload: Payload) -> Outcome {
    Outcome(payload.0 == 0)
}
