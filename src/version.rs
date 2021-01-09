// Check if the two semver are equivalent
// But only check the version numbers the user provide in the cli
// Ex : 8.0 => will only check major and minor on the other version
// Ex : 14 => will only check the major number
pub fn is_versions_equivalent(required_version: &str, comparing_version: &str) -> bool {
    // Check the precision of the version provided by the user
    let version_parts = required_version.split('.').count();

    // Check if the required version of the formulae is available
    let required_version = lenient_semver::parse(required_version).unwrap();
    let comparing_version = lenient_semver::parse(comparing_version).unwrap();

    return match version_parts {
        // Check only major
        1 => required_version.major == comparing_version.major,
        // Check major and minor
        2 => {
            required_version.major == comparing_version.major
                && required_version.minor == comparing_version.minor
        }
        // Check major, minor and patch
        3 => {
            required_version.major == comparing_version.major
                && required_version.minor == comparing_version.minor
                && required_version.patch == comparing_version.patch
        }
        // Unsupported case
        _ => panic!(
            "Cannot compare correctly versions, unsupported version pattern {}",
            required_version
        ),
    };
}
