use crate::{DownloadId, TopicId};
use scraper::error::SelectorErrorKind;
use scraper::{Html, Selector};

const AUDIO_FORMAT_PRIORITY: [&str; 4] = ["FLAC", "MP3", "ALAC", "AAC"];
const AUDIO_BITRATE_PRIORITY: [&str; 3] = ["lossless", "320 kbps", "256 kbps"];

const CAPTCHA_IS_REQUIRED_TEXT: &str = "введите код подтверждения";
const INCORRECT_PASSWORD_TEXT: &str = "неверный пароль";
const SUCCESSFUL_LOGIN_TEXT: &str = "log-out-icon";

fn get_search_result_priority(result: &TopicData) -> usize {
    let format_priority = AUDIO_FORMAT_PRIORITY
        .iter()
        .enumerate()
        .find_map(|(i, format)| {
            if result.title.contains(format) {
                Some(i)
            } else {
                None
            }
        })
        .unwrap_or(10);
    let bitrate_priority = AUDIO_BITRATE_PRIORITY
        .iter()
        .enumerate()
        .find_map(|(i, bitrate)| {
            if result.title.contains(bitrate) {
                Some(i)
            } else {
                None
            }
        })
        .unwrap_or(10);
    let seeds_priority = match result.seeds_number {
        x if x == 0 => 10,
        x if x < 10 => 3,
        x if x < 20 => 2,
        x if x < 30 => 1,
        _ => 0,
    };

    format_priority * 5 + bitrate_priority * 10 + seeds_priority
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    SelectorError(#[from] SelectorErrorKind<'static>),
}

#[derive(Debug, PartialEq)]
pub struct TopicData {
    pub title: String,
    pub topic_id: TopicId,
    pub download_id: DownloadId,
    pub seeds_number: u64,
}

pub(crate) fn parse_search_results(raw_html: &str) -> Result<Vec<TopicData>, ParseError> {
    let html = Html::parse_document(raw_html);

    let table_row_selector = Selector::parse(r#"table.forumline tr"#)?;
    let table_entries = html.select(&table_row_selector);

    let href_selector = Selector::parse(r#"a[href]"#)?;
    let td_selector = Selector::parse(r#"td"#)?;
    let seeds_selector = Selector::parse(r#"b.seedmed"#)?;

    let mut results: Vec<_> = table_entries
        .skip(1)
        .filter(|el| el.children().filter(|el| el.value().is_element()).count() == 10)
        .filter_map(|el| {
            let columns = el.select(&td_selector).collect::<Vec<_>>();
            let link = columns[3].select(&href_selector).next()?;
            let category_str = columns[2]
                .select(&href_selector)
                .next()?
                .inner_html()
                .to_lowercase();

            if !category_str.contains("loss") {
                return None;
            }

            let title = link.inner_html().to_string();
            let topic_id = link
                .value()
                .attr("data-topic_id")?
                .to_string()
                .parse::<u64>()
                .ok()?
                .into();
            let download_id = columns[5]
                .select(&href_selector)
                .next()?
                .value()
                .attr("href")?
                .to_string()
                .replace("dl.php?t=", "")
                .parse::<u64>()
                .ok()?
                .into();
            let seeds_number = columns[6]
                .select(&seeds_selector)
                .next()?
                .inner_html()
                .to_string()
                .parse::<u64>()
                .ok()?
                .into();

            Some(TopicData {
                title,
                topic_id,
                download_id,
                seeds_number,
            })
        })
        .filter(|r| !r.title.contains("image+.cue"))
        .collect();

    // Sort search results by the search result priority
    results.sort_by(|a, b| get_search_result_priority(a).cmp(&get_search_result_priority(b)));

    Ok(results)
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Captcha verification is required.")]
    CaptchaVerificationIsRequired,
    #[error("Incorrect login or password.")]
    IncorrectPasswordText,
    #[error("Unknown authentication error")]
    UnknownAuthError,
}

pub(crate) fn parse_and_validate_auth_state(raw_html: &str) -> Result<(), AuthError> {
    if raw_html.contains(CAPTCHA_IS_REQUIRED_TEXT) {
        return Err(AuthError::CaptchaVerificationIsRequired);
    }

    if raw_html.contains(INCORRECT_PASSWORD_TEXT) {
        return Err(AuthError::IncorrectPasswordText);
    }

    if !raw_html.contains(SUCCESSFUL_LOGIN_TEXT) {
        return Err(AuthError::UnknownAuthError);
    }

    Ok(())
}
