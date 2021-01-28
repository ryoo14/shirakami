use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;

pub fn shorten_url(org_url: &str) -> Result<String> {
    // build regexp
    let product_id = "[0-9a-zA-Z]{10}?";
    let dp_or_gp = String::from("(dp|gp/product)/");
    let pattern = dp_or_gp + product_id;
    let re_product_id = Regex::new(&pattern).unwrap();

    // match
    let base_amazon_url = String::from("https://www.amazon.co.jp/");
    match re_product_id.captures(org_url) {
        Some(c) => Ok(base_amazon_url + &c[0]),
        None => Err(anyhow!("no match your input url.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp_url() {
        let url = "https://www.amazon.co.jp/%E3%82%B9%E3%83%9E%E3%83%BC%E3%83%88%E3%82%A6%E3%82%A9%E3%83%83%E3%83%81-%E3%83%95%E3%83%AB%E3%82%BF%E3%83%83%E3%83%81%E3%82%B9%E3%82%AF%E3%83%AA%E3%83%BC%E3%83%B3-1-3%E3%82%A4%E3%83%B3%E3%83%81HD%E7%94%BB%E9%9D%A2-%E3%82%B9%E3%83%9E%E3%83%BC%E3%83%88%E3%83%96%E3%83%AC%E3%82%B9%E3%83%AC%E3%83%83%E3%83%88-%E3%82%B9%E3%83%88%E3%83%83%E3%83%97%E3%82%A6%E3%82%A9%E3%83%83%E3%83%81/dp/B08PPF6SHS?ref_=Oct_DLandingS_D_15a8fb7f_61&smid=A19WYSWOXJW1YG";
        assert_eq!(
            shorten_url(url).unwrap(),
            "https://www.amazon.co.jp/dp/B08PPF6SHS"
        );
    }
    #[test]
    fn test_gp_url() {
        let url = "https://www.amazon.co.jp/gp/product/B08PPF6SHS?pf_rd_r=3N5YDCWXTRAAV5TYZT9P&pf_rd_p=3d55ec74-6376-483a-a5a7-4e247166f80b";
        assert_eq!(
            shorten_url(url).unwrap(),
            "https://www.amazon.co.jp/gp/product/B08PPF6SHS"
        );
    }
    #[test]
    #[should_panic]
    fn test_not_include_dp_and_gp() {
        let url = "https://www.amazon.co.jp/not_include_dp_and_gp";
        shorten_url(url).unwrap();
    }
}
