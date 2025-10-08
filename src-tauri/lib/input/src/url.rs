use crate::{Input, Split};
use common::resources_dir;
use headless_chrome::{Browser, LaunchOptions};
use std::path::Path;

pub struct UrlInput;
impl Input for UrlInput {
    type Output = crate::Result<String>;

    fn read(url: impl AsRef<Path>) -> Self::Output {
        let browser = Browser::new(LaunchOptions {
            headless: true,
            // 禁用沙盒，该选项启用后会导致docker容器里的chrome连接超时,
            // 最终导致错误：ChromeLaunchError::NoAvailablePorts
            sandbox: false,
            //path: Some("resources/driver/chrome/chrome".into()),
            path: Some(resources_dir!("driver", "chrome", "chrome")),
            ..Default::default()
        })?;

        let tab = browser.new_tab()?;
        tab.navigate_to(url.as_ref().to_str().unwrap())?;
        tab.wait_until_navigated()?;
        tab.find_element("body")?.get_inner_text()?;
        let content = tab.find_element("body")?.get_inner_text()?;
        Ok(content)
    }
}

impl Split for UrlInput {}

impl UrlInput {
    /// 获取网页标题
    pub fn get_url_title(url: &str) -> crate::Result<String> {
        let browser = Browser::new(LaunchOptions {
            headless: true,
            // 禁用沙盒，该选项启用后会导致docker容器里的chrome连接超时,
            // 最终导致错误：ChromeLaunchError::NoAvailablePorts
            sandbox: false,
            //path: Some("resources/driver/chrome/chrome".into()),
            path: Some(resources_dir!("driver", "chrome", "chrome")),
            ..Default::default()
        })?;

        let tab = browser.new_tab()?;
        tab.navigate_to(url)?;
        tab.wait_until_navigated()?;
        Ok(tab.get_title()?)
    }
}
