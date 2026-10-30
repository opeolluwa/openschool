use openschool_models::sea_orm_active_enums::ProductCurrency;
use crate::utils::currency::parse_currency;

const ONES: [&str; 20] = [
    "",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];

const TENS: [&str; 10] = [
    "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

const SCALES: [&str; 7] = [
    "",
    "Thousand",
    "Million",
    "Billion",
    "Trillion",
    "Quadrillion",
    "Quintillion",
];

fn group_to_words(n: u16) -> String {
    debug_assert!(n < 1000);
    let mut parts = Vec::new();

    let hundreds = n / 100;
    let remainder = n % 100;

    if hundreds > 0 {
        parts.push(format!("{} Hundred", ONES[hundreds as usize]));
    }

    if remainder >= 20 {
        let t = TENS[(remainder / 10) as usize];
        let o = remainder % 10;
        if o > 0 {
            parts.push(format!("{}-{}", t, ONES[o as usize]));
        } else {
            parts.push(t.to_string());
        }
    } else if remainder > 0 {
        parts.push(ONES[remainder as usize].to_string());
    }

    parts.join(" ")
}

fn number_to_words(n: u64) -> String {
    if n == 0 {
        return "Zero".to_string();
    }

    let mut remaining = n;
    let mut groups = Vec::new();
    let mut scale_idx = 0;

    while remaining > 0 {
        let group = (remaining % 1000) as u16;
        if group > 0 {
            let words = group_to_words(group);
            if SCALES[scale_idx].is_empty() {
                groups.push(words);
            } else {
                groups.push(format!("{} {}", words, SCALES[scale_idx]));
            }
        }
        remaining /= 1000;
        scale_idx += 1;
    }

    groups.reverse();
    groups.join(" ")
}

struct CurrencyWords {
    major: &'static str,
    major_plural: &'static str,
    minor: &'static str,
    minor_plural: &'static str,
}

fn currency_words(currency: ProductCurrency) -> CurrencyWords {
    match currency {
        ProductCurrency::Ngn => CurrencyWords {
            major: "Naira",
            major_plural: "Naira",
            minor: "Kobo",
            minor_plural: "Kobo",
        },
        ProductCurrency::Usd => CurrencyWords {
            major: "Dollar",
            major_plural: "Dollars",
            minor: "Cent",
            minor_plural: "Cents",
        },
        ProductCurrency::Gbp => CurrencyWords {
            major: "Pound",
            major_plural: "Pounds",
            minor: "Penny",
            minor_plural: "Pence",
        },
        ProductCurrency::Eur => CurrencyWords {
            major: "Euro",
            major_plural: "Euros",
            minor: "Cent",
            minor_plural: "Cents",
        },
        ProductCurrency::Ghs => CurrencyWords {
            major: "Cedi",
            major_plural: "Cedis",
            minor: "Pesewa",
            minor_plural: "Pesewas",
        },
        ProductCurrency::Kes => CurrencyWords {
            major: "Shilling",
            major_plural: "Shillings",
            minor: "Cent",
            minor_plural: "Cents",
        },
        ProductCurrency::Zar => CurrencyWords {
            major: "Rand",
            major_plural: "Rand",
            minor: "Cent",
            minor_plural: "Cents",
        },
        ProductCurrency::Cad => CurrencyWords {
            major: "Dollar",
            major_plural: "Dollars",
            minor: "Cent",
            minor_plural: "Cents",
        },
        ProductCurrency::Aud => CurrencyWords {
            major: "Dollar",
            major_plural: "Dollars",
            minor: "Cent",
            minor_plural: "Cents",
        },
        ProductCurrency::Jpy => CurrencyWords {
            major: "Yen",
            major_plural: "Yen",
            minor: "Sen",
            minor_plural: "Sen",
        },
        ProductCurrency::Cny => CurrencyWords {
            major: "Yuan",
            major_plural: "Yuan",
            minor: "Fen",
            minor_plural: "Fen",
        },
        ProductCurrency::Inr => CurrencyWords {
            major: "Rupee",
            major_plural: "Rupees",
            minor: "Paisa",
            minor_plural: "Paise",
        },
        ProductCurrency::Aed => CurrencyWords {
            major: "Dirham",
            major_plural: "Dirhams",
            minor: "Fils",
            minor_plural: "Fils",
        },
        ProductCurrency::Xof => CurrencyWords {
            major: "Franc",
            major_plural: "Francs",
            minor: "Centime",
            minor_plural: "Centimes",
        },
    }
}

pub fn amount_in_words(minor_units: i64, currency_code: &str) -> Option<String> {
    let currency = parse_currency(currency_code)?;
    let words = currency_words(currency);

    let abs = minor_units.unsigned_abs();
    let major = abs / 100;
    let minor = abs % 100;

    let major_words = number_to_words(major);
    let minor_words = number_to_words(minor);

    let major_unit = if major == 1 {
        words.major
    } else {
        words.major_plural
    };
    let minor_unit = if minor == 1 {
        words.minor
    } else {
        words.minor_plural
    };

    Some(format!(
        "{} {} and {} {} Only",
        major_words, major_unit, minor_words, minor_unit
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_amount() {
        assert_eq!(
            amount_in_words(0, "ngn").unwrap(),
            "Zero Naira and Zero Kobo Only"
        );
    }

    #[test]
    fn whole_major_units() {
        assert_eq!(
            amount_in_words(18_750_000, "ngn").unwrap(),
            "One Hundred Eighty-Seven Thousand Five Hundred Naira and Zero Kobo Only"
        );
    }

    #[test]
    fn minor_units_only() {
        assert_eq!(
            amount_in_words(50, "ngn").unwrap(),
            "Zero Naira and Fifty Kobo Only"
        );
    }

    #[test]
    fn mixed_major_and_minor() {
        assert_eq!(
            amount_in_words(125_050, "usd").unwrap(),
            "One Thousand Two Hundred Fifty Dollars and Fifty Cents Only"
        );
    }

    #[test]
    fn singular_major_unit() {
        assert_eq!(
            amount_in_words(100, "ngn").unwrap(),
            "One Naira and Zero Kobo Only"
        );
    }

    #[test]
    fn singular_minor_unit() {
        assert_eq!(
            amount_in_words(1, "ngn").unwrap(),
            "Zero Naira and One Kobo Only"
        );
    }

    #[test]
    fn gbp_singular_plural_distinction() {
        assert_eq!(
            amount_in_words(1, "gbp").unwrap(),
            "Zero Pounds and One Penny Only"
        );
        assert_eq!(
            amount_in_words(2, "gbp").unwrap(),
            "Zero Pounds and Two Pence Only"
        );
    }

    #[test]
    fn hyphenated_compound_tens() {
        assert_eq!(
            amount_in_words(2_100, "usd").unwrap(),
            "Twenty-One Dollars and Zero Cents Only"
        );
        assert_eq!(
            amount_in_words(9_999, "usd").unwrap(),
            "Ninety-Nine Dollars and Ninety-Nine Cents Only"
        );
    }

    #[test]
    fn large_number() {
        assert_eq!(
            amount_in_words(1_000_000_000_00, "ngn").unwrap(),
            "One Billion Naira and Zero Kobo Only"
        );
    }

    #[test]
    fn every_currency_produces_output() {
        for code in [
            "ngn", "usd", "gbp", "eur", "ghs", "kes", "zar", "cad", "aud", "jpy", "cny", "inr",
            "aed", "xof",
        ] {
            assert!(
                amount_in_words(12345, code).is_some(),
                "{code} should produce output"
            );
        }
    }

    #[test]
    fn unknown_currency_returns_none() {
        assert!(amount_in_words(100, "xyz").is_none());
        assert!(amount_in_words(100, "").is_none());
    }

    #[test]
    fn number_to_words_coverage() {
        assert_eq!(number_to_words(0), "Zero");
        assert_eq!(number_to_words(1), "One");
        assert_eq!(number_to_words(10), "Ten");
        assert_eq!(number_to_words(15), "Fifteen");
        assert_eq!(number_to_words(20), "Twenty");
        assert_eq!(number_to_words(99), "Ninety-Nine");
        assert_eq!(number_to_words(100), "One Hundred");
        assert_eq!(number_to_words(101), "One Hundred One");
        assert_eq!(number_to_words(999), "Nine Hundred Ninety-Nine");
        assert_eq!(number_to_words(1_000), "One Thousand");
        assert_eq!(number_to_words(1_001), "One Thousand One");
        assert_eq!(
            number_to_words(123_456),
            "One Hundred Twenty-Three Thousand Four Hundred Fifty-Six"
        );
        assert_eq!(number_to_words(1_000_000), "One Million");
        assert_eq!(number_to_words(1_000_000_000), "One Billion");
    }
}
