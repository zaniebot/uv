use std::borrow::Cow;
use std::str::FromStr;

use jiff::{Span, Timestamp, ToSpan, Unit, tz::TimeZone};

/// A timestamp that excludes files newer than it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExcludeNewerValue {
    /// The resolved timestamp.
    timestamp: Timestamp,
    /// The span used to derive the [`Timestamp`], if any.
    span: Option<ExcludeNewerSpan>,
}

impl PartialOrd for ExcludeNewerValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExcludeNewerValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl std::hash::Hash for ExcludeNewerValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.timestamp.hash(state);
    }
}

impl ExcludeNewerValue {
    /// Create a new [`ExcludeNewerValue`].
    pub fn new(timestamp: Timestamp, span: Option<ExcludeNewerSpan>) -> Self {
        Self { timestamp, span }
    }

    /// Return the [`Timestamp`] in milliseconds.
    pub fn timestamp_millis(&self) -> i64 {
        self.timestamp.as_millisecond()
    }

    /// Return the [`Timestamp`].
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Return the [`ExcludeNewerSpan`] used to construct the [`Timestamp`], if any.
    pub fn span(&self) -> Option<&ExcludeNewerSpan> {
        self.span.as_ref()
    }

    pub fn into_parts(self) -> (Timestamp, Option<ExcludeNewerSpan>) {
        (self.timestamp, self.span)
    }

    pub fn compare(&self, other: &Self) -> Option<ExcludeNewerValueChange> {
        match (&self.span, &other.span) {
            (None, Some(span)) => Some(ExcludeNewerValueChange::SpanAdded(*span)),
            (Some(_), None) => Some(ExcludeNewerValueChange::SpanRemoved),
            (Some(self_span), Some(other_span)) if self_span != other_span => Some(
                ExcludeNewerValueChange::SpanChanged(*self_span, *other_span),
            ),
            (Some(_), Some(span)) if self.timestamp != other.timestamp => {
                Some(ExcludeNewerValueChange::RelativeTimestampChanged(
                    self.timestamp,
                    other.timestamp,
                    *span,
                ))
            }
            (None, None) if self.timestamp != other.timestamp => Some(
                ExcludeNewerValueChange::AbsoluteTimestampChanged(self.timestamp, other.timestamp),
            ),
            (Some(_), Some(_)) | (None, None) => None,
        }
    }
}

impl From<Timestamp> for ExcludeNewerValue {
    fn from(timestamp: Timestamp) -> Self {
        Self {
            timestamp,
            span: None,
        }
    }
}

impl std::fmt::Display for ExcludeNewerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.timestamp.fmt(f)
    }
}

impl serde::Serialize for ExcludeNewerValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.timestamp.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ExcludeNewerValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Support both a simple string ("2024-03-11T00:00:00Z") and a table
        // ({ timestamp = "2024-03-11T00:00:00Z", span = "P2W" })
        #[derive(serde::Deserialize)]
        struct TableForm {
            timestamp: Timestamp,
            span: Option<ExcludeNewerSpan>,
        }

        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum Helper {
            String(String),
            Table(Box<TableForm>),
        }

        match Helper::deserialize(deserializer)? {
            Helper::String(s) => Self::from_str(&s).map_err(serde::de::Error::custom),
            Helper::Table(table) => Ok(Self::new(table.timestamp, table.span)),
        }
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for ExcludeNewerValue {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("ExcludeNewerValue")
    }

    fn json_schema(_generator: &mut schemars::generate::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "description": "Exclude distributions uploaded after the given timestamp.\n\nAccepts both RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`) and local dates in the same format (e.g., `2006-12-02`), as well as relative durations (e.g., `1 week`, `30 days`, `6 months`). Relative durations are resolved to a timestamp at lock time.",
        })
    }
}

/// Determine what format the user likely intended and return an appropriate error message.
fn format_exclude_newer_error(
    input: &str,
    date_err: &jiff::Error,
    span_err: &jiff::Error,
) -> String {
    let trimmed = input.trim();

    // Check for ISO 8601 duration (`[-+]?[Pp]`), e.g., "P2W", "+P1D", "-P30D"
    let after_sign = trimmed.trim_start_matches(['+', '-']);
    if after_sign.starts_with('P') || after_sign.starts_with('p') {
        return format!("`{input}` could not be parsed as an ISO 8601 duration: {span_err}");
    }

    // Check for friendly duration (`[-+]?\s*[0-9]+\s*[A-Za-z]`), e.g., "2 weeks", "-30 days",
    // "1hour"
    let after_sign_trimmed = after_sign.trim_start();
    let mut chars = after_sign_trimmed.chars().peekable();

    // Check if we start with a digit
    if chars.peek().is_some_and(char::is_ascii_digit) {
        // Skip digits
        while chars.peek().is_some_and(char::is_ascii_digit) {
            chars.next();
        }
        // Skip optional whitespace
        while chars.peek().is_some_and(|c| c.is_whitespace()) {
            chars.next();
        }
        // Check if next character is a letter (unit designator)
        if chars.peek().is_some_and(char::is_ascii_alphabetic) {
            return format!("`{input}` could not be parsed as a duration: {span_err}");
        }
    }

    // Check for date/timestamp (`[-+]?[0-9]{4}-`), e.g., "2024-01-01", "2024-01-01T00:00:00Z"
    let mut chars = after_sign.chars();
    let looks_like_date = chars.next().is_some_and(|c| c.is_ascii_digit())
        && chars.next().is_some_and(|c| c.is_ascii_digit())
        && chars.next().is_some_and(|c| c.is_ascii_digit())
        && chars.next().is_some_and(|c| c.is_ascii_digit())
        && chars.next().is_some_and(|c| c == '-');

    if looks_like_date {
        return format!("`{input}` could not be parsed as a valid date: {date_err}");
    }

    // If we can't tell, return a generic error message
    format!(
        "`{input}` could not be parsed as a valid exclude-newer value (expected a date like `2024-01-01`, a timestamp like `2024-01-01T00:00:00Z`, or a duration like `3 days` or `P3D`)"
    )
}

impl FromStr for ExcludeNewerValue {
    type Err = String;

    /// Parse an [`ExcludeNewerValue`] from a string.
    ///
    /// Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format
    /// (e.g., `2006-12-02`), "friendly" durations (e.g., `1 week`, `30 days`), and ISO 8601
    /// durations (e.g., `PT24H`, `P7D`, `P30D`).
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Try parsing as a timestamp first
        if let Ok(timestamp) = input.parse::<Timestamp>() {
            return Ok(Self::new(timestamp, None));
        }

        // Try parsing as a date
        // In Jiff, if an RFC 3339 timestamp could be parsed, then it must necessarily be the case
        // that a date can also be parsed. So we can collapse the error cases here. That is, if we
        // fail to parse a timestamp and a date, then it should be sufficient to just report the
        // error from parsing the date. If someone tried to write a timestamp but committed an error
        // in the non-date portion, the date parsing below will still report a holistic error that
        // will make sense to the user. (I added a snapshot test for that case.)
        let date_err = match input.parse::<jiff::civil::Date>() {
            Ok(date) => {
                let timestamp = date
                    .checked_add(1.day())
                    .and_then(|date| date.to_zoned(TimeZone::system()))
                    .map(|zdt| zdt.timestamp())
                    .map_err(|err| {
                        format!(
                            "`{input}` parsed to date `{date}`, but could not \
                         be converted to a timestamp: {err}",
                        )
                    })?;
                return Ok(Self::new(timestamp, None));
            }
            Err(err) => err,
        };

        // Try parsing as a span
        let span_err = match input.parse::<Span>() {
            Ok(span) => {
                // Allow overriding the current time in tests for deterministic snapshots
                let now = if let Ok(test_time) = std::env::var("UV_TEST_CURRENT_TIMESTAMP") {
                    test_time
                        .parse::<Timestamp>()
                        .expect("UV_TEST_CURRENT_TIMESTAMP must be a valid RFC 3339 timestamp")
                        .to_zoned(TimeZone::UTC)
                } else {
                    Timestamp::now().to_zoned(TimeZone::UTC)
                };

                // We do not allow years and months as units, as the amount of time they represent
                // is not fixed and can differ depending on the local time zone. We could allow this
                // via the CLI in the future, but shouldn't allow it via persistent configuration.
                if span.get_years() != 0 {
                    let years = span
                        .total((Unit::Year, &now))
                        .map(f64::ceil)
                        .unwrap_or(1.0)
                        .abs();
                    let days = years * 365.0;
                    return Err(format!(
                        "Duration `{input}` uses unit 'years' which is not allowed; use days instead, e.g., `{days:.0} days`.",
                    ));
                }
                if span.get_months() != 0 {
                    let months = span
                        .total((Unit::Month, &now))
                        .map(f64::ceil)
                        .unwrap_or(1.0)
                        .abs();
                    let days = months * 30.0;
                    return Err(format!(
                        "Duration `{input}` uses 'months' which is not allowed; use days instead, e.g., `{days:.0} days`."
                    ));
                }

                // We're using a UTC timezone so there are no transitions (e.g., DST) and days are
                // always 24 hours. This means that we can also allow weeks as a unit.
                //
                // Note we use `span.abs()` so `1 day ago` has the same effect as `1 day` instead
                // of resulting in a future date.
                let cutoff = now.checked_sub(span.abs()).map_err(|err| {
                    format!("Duration `{input}` is too large to subtract from current time: {err}")
                })?;

                return Ok(Self::new(cutoff.into(), Some(ExcludeNewerSpan(span))));
            }
            Err(err) => err,
        };

        // Return a targeted error message based on heuristics about what the user likely intended
        Err(format_exclude_newer_error(input, &date_err, &span_err))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ExcludeNewerSpan(Span);

impl std::fmt::Display for ExcludeNewerSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for ExcludeNewerSpan {
    fn eq(&self, other: &Self) -> bool {
        self.0.fieldwise() == other.0.fieldwise()
    }
}

impl Eq for ExcludeNewerSpan {}

impl serde::Serialize for ExcludeNewerSpan {
    /// Serialize to an ISO 8601 duration string.
    ///
    /// We use ISO 8601 format for serialization (rather than the "friendly" format).
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for ExcludeNewerSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <Cow<'_, str>>::deserialize(deserializer)?;
        let span: Span = s.parse().map_err(serde::de::Error::custom)?;
        Ok(Self(span))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExcludeNewerValueChange {
    /// A relative span changed to a new value
    SpanChanged(ExcludeNewerSpan, ExcludeNewerSpan),
    /// A relative span was added
    SpanAdded(ExcludeNewerSpan),
    /// A relative span was removed
    SpanRemoved,
    /// A relative span is present and the timestamp changed
    RelativeTimestampChanged(Timestamp, Timestamp, ExcludeNewerSpan),
    /// The timestamp changed and a relative span is not present
    AbsoluteTimestampChanged(Timestamp, Timestamp),
}

impl ExcludeNewerValueChange {
    pub fn is_relative_timestamp_change(&self) -> bool {
        matches!(self, Self::RelativeTimestampChanged(_, _, _))
    }
}

impl std::fmt::Display for ExcludeNewerValueChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpanChanged(old, new) => {
                write!(f, "change of exclude newer span from `{old}` to `{new}`")
            }
            Self::SpanAdded(span) => {
                write!(f, "addition of exclude newer span `{span}`")
            }
            Self::SpanRemoved => {
                write!(f, "removal of exclude newer span")
            }
            Self::RelativeTimestampChanged(old, new, span) => {
                write!(
                    f,
                    "change of calculated ({span}) exclude newer timestamp from `{old}` to `{new}`"
                )
            }
            Self::AbsoluteTimestampChanged(old, new) => {
                write!(
                    f,
                    "change of exclude newer timestamp from `{old}` to `{new}`"
                )
            }
        }
    }
}
