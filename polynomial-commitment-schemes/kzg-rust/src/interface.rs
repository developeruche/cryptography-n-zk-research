use crate::primitives::SRS;

/// This trait is used by it implementing struct to create a new SRS, taking in the string representation of the SRS or the fs-path to the SRS file.
pub trait FromStringToSRS {
    fn from_string_to_srs(&self, srs: String) -> SRS;
}

pub trait KZGInterface {
    // random_setup
    // setup_with_import
    // commit
    // compute witness polynomial
    // open withness polynomial
    // open
    // check
    // batch check
}
