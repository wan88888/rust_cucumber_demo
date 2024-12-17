use thirtyfour::prelude::*;
use std::time::Duration;

#[derive(Debug)]
pub enum LoginError {
    ElementNotFound(String),
    ElementNotInteractable(String),
    WebDriverError(thirtyfour::error::WebDriverError),
    UnexpectedState(String),
}

impl From<thirtyfour::error::WebDriverError> for LoginError {
    fn from(error: thirtyfour::error::WebDriverError) -> Self {
        LoginError::WebDriverError(error)
    }
}

pub struct LoginPage {
    driver: WebDriver,
}

impl std::fmt::Debug for LoginPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginPage")
            .field("driver", &"<WebDriver>")
            .finish()
    }
}

impl LoginPage {
    pub fn new(driver: WebDriver) -> Self {
        Self { driver }
    }

    async fn wait_for_element(&self, by: By) -> Result<WebElement, LoginError> {
        self.driver
            .query(by)
            .wait(Duration::from_secs(10), Duration::from_millis(500))
            .first()
            .await
            .map_err(|e| LoginError::ElementNotFound(format!("Element not found: {:?}", e)))
    }

    pub async fn navigate(&self) -> Result<(), LoginError> {
        self.driver.goto("http://the-internet.herokuapp.com/login").await?;
        // 等待页面加载完成
        self.wait_for_element(By::Id("username")).await?;
        Ok(())
    }

    pub async fn enter_username(&self, username: &str) -> Result<(), LoginError> {
        let username_input = self.wait_for_element(By::Id("username")).await?;
        username_input.send_keys(username).await?;
        Ok(())
    }

    pub async fn enter_password(&self, password: &str) -> Result<(), LoginError> {
        let password_input = self.wait_for_element(By::Id("password")).await?;
        password_input.send_keys(password).await?;
        Ok(())
    }

    pub async fn click_login_button(&self) -> Result<(), LoginError> {
        let login_button = self.wait_for_element(By::Css("button[type='submit']")).await?;
        login_button.click().await?;
        Ok(())
    }

    pub async fn is_logged_in(&self) -> Result<bool, LoginError> {
        Ok(self.driver
            .query(By::Css(".flash.success"))
            .wait(Duration::from_secs(10), Duration::from_millis(500))
            .exists()
            .await?)
    }

    pub async fn get_error_message(&self) -> Result<String, LoginError> {
        let error_message = self.wait_for_element(By::Css(".flash.error")).await?;
        Ok(error_message.text().await?)
    }

    pub async fn is_in_secure_area(&self) -> Result<bool, LoginError> {
        Ok(self.driver
            .query(By::Css("h2"))
            .wait(Duration::from_secs(10), Duration::from_millis(500))
            .first()
            .await?
            .text()
            .await?
            .contains("Secure Area"))
    }

    pub async fn clear_username(&self) -> Result<(), LoginError> {
        let username_input = self.wait_for_element(By::Id("username")).await?;
        username_input.clear().await?;
        Ok(())
    }

    pub async fn clear_password(&self) -> Result<(), LoginError> {
        let password_input = self.wait_for_element(By::Id("password")).await?;
        password_input.clear().await?;
        Ok(())
    }

    pub async fn logout(&self) -> Result<(), LoginError> {
        if self.is_logged_in().await? {
            let logout_button = self.wait_for_element(By::Css(".button.secondary")).await?;
            logout_button.click().await?;
            
            // 等待登出完成
            self.wait_for_element(By::Id("username")).await?;
        }
        Ok(())
    }

    pub async fn quit(self) -> Result<(), LoginError> {
        self.driver.quit().await?;
        Ok(())
    }
}
