use uv_static::EnvVars;

#[test]
fn added_in_metadata_uses_released_version_or_next_release() {
    let invalid = EnvVars::metadata()
        .iter()
        .filter_map(|(name, _, added_in)| match *added_in {
            Some(added_in) if is_valid_added_in(added_in) => None,
            Some(added_in) => Some(format!("{name}: {added_in}")),
            None => Some(format!("{name}: missing")),
        })
        .collect::<Vec<_>>();

    assert!(
        invalid.is_empty(),
        "invalid env var added-in metadata: {}",
        invalid.join(", ")
    );
}

fn is_valid_added_in(added_in: &str) -> bool {
    added_in == "next release" || is_semantic_version(added_in)
}

fn is_semantic_version(version: &str) -> bool {
    let mut components = version.split('.');
    let Some(major) = components.next() else {
        return false;
    };
    let Some(minor) = components.next() else {
        return false;
    };
    let Some(patch) = components.next() else {
        return false;
    };

    if components.next().is_some() {
        return false;
    }

    [major, minor, patch].into_iter().all(|component| {
        !component.is_empty() && component.bytes().all(|byte| byte.is_ascii_digit())
    })
}
