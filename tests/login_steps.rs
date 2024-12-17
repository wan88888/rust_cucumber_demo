use std::convert::Infallible;
use cucumber::{given, when, then, World};
use thirtyfour::prelude::*;
use tokio::sync::Mutex;
use std::sync::Arc;
use rust_cucumber_demo::page_objects::login_page::LoginPage;

#[derive(World)]
#[world(init = Self::new)]
pub struct LoginWorld {
    driver: Arc<Mutex<Option<WebDriver>>>,
    login_page: Arc<Mutex<Option<LoginPage>>>,
    error_message: Arc<Mutex<Option<String>>>,
}

// 手动实现 Debug
impl std::fmt::Debug for LoginWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginWorld")
            .field("driver", &"<WebDriver>")
            .field("login_page", &"<LoginPage>")
            .field("error_message", &*self.error_message.blocking_lock())
            .finish()
    }
}

impl LoginWorld {
    async fn setup_driver() -> WebDriverResult<WebDriver> {
        let caps = DesiredCapabilities::chrome();
        WebDriver::new("http://localhost:9515", caps).await
    }

    async fn cleanup(&mut self) {
        if let Some(login_page) = self.login_page.lock().await.take() {
            // 尝试登出（如果已登录）
            let _ = login_page.logout().await;
            
            // 清理输入字段
            let _ = login_page.clear_username().await;
            let _ = login_page.clear_password().await;
            
            // 关闭浏览器
            let _ = login_page.quit().await;
        }
        
        // 重置状态
        let mut driver_mutex = self.driver.lock().await;
        *driver_mutex = None;
        
        let mut error_mutex = self.error_message.lock().await;
        *error_mutex = None;
    }

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            driver: Arc::new(Mutex::new(None)),
            login_page: Arc::new(Mutex::new(None)),
            error_message: Arc::new(Mutex::new(None)),
        })
    }
}

// 在每个场景之后进行清理
#[given("I am on the login page")]
async fn on_login_page(world: &mut LoginWorld) -> Result<(), Infallible> {
    // 首先清理之前的资源
    world.cleanup().await;

    let driver = LoginWorld::setup_driver().await.unwrap();
    let login_page = LoginPage::new(driver.clone());
    login_page.navigate().await.unwrap();
    
    let mut driver_mutex = world.driver.lock().await;
    *driver_mutex = Some(driver);
    
    let mut page_mutex = world.login_page.lock().await;
    *page_mutex = Some(login_page);
    
    Ok(())
}

#[when(expr = "I enter username {string}")]
async fn enter_username(world: &mut LoginWorld, username: String) -> Result<(), Infallible> {
    let page_mutex = world.login_page.lock().await;
    if let Some(login_page) = &*page_mutex {
        login_page.enter_username(&username).await.unwrap();
    }
    Ok(())
}

#[when(expr = "I enter password {string}")]
async fn enter_password(world: &mut LoginWorld, password: String) -> Result<(), Infallible> {
    let page_mutex = world.login_page.lock().await;
    if let Some(login_page) = &*page_mutex {
        login_page.enter_password(&password).await.unwrap();
    }
    Ok(())
}

#[when("I click the login button")]
async fn click_login(world: &mut LoginWorld) -> Result<(), Infallible> {
    let page_mutex = world.login_page.lock().await;
    if let Some(login_page) = &*page_mutex {
        login_page.click_login_button().await.unwrap();
    }
    Ok(())
}

#[then("I should see an error message")]
async fn verify_error(world: &mut LoginWorld) -> Result<(), Infallible> {
    let page_mutex = world.login_page.lock().await;
    if let Some(login_page) = &*page_mutex {
        let message = login_page.get_error_message().await.unwrap();
        let mut error_message = world.error_message.lock().await;
        *error_message = Some(message);
    }
    Ok(())
}

#[then(expr = "the error message should contain {string}")]
async fn verify_error_message(world: &mut LoginWorld, expected: String) -> Result<(), Infallible> {
    let error_message = world.error_message.lock().await;
    if let Some(message) = &*error_message {
        assert!(message.contains(&expected), 
            "Expected error message to contain '{}', but got '{}'", 
            expected, message);
    } else {
        panic!("No error message was captured");
    }
    Ok(())
}

#[then("I should be logged in successfully")]
async fn verify_success(world: &mut LoginWorld) -> Result<(), Infallible> {
    let page_mutex = world.login_page.lock().await;
    if let Some(login_page) = &*page_mutex {
        assert!(login_page.is_logged_in().await.unwrap());
    }
    Ok(())
}

#[then("I should see the secure area")]
async fn verify_secure_area(world: &mut LoginWorld) -> Result<(), Infallible> {
    let page_mutex = world.login_page.lock().await;
    if let Some(login_page) = &*page_mutex {
        assert!(login_page.is_in_secure_area().await.unwrap());
    }
    Ok(())
}
