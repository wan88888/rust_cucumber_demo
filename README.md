# Rust Cucumber Web Automation Demo

这是一个基于Rust和Cucumber的Web自动化测试框架示例，使用Page Object模式实现页面对象管理。

## 技术栈

- Rust：主要编程语言
- Cucumber：BDD测试框架
- Selenium (thirtyfour)：Web自动化工具
- ChromeDriver：浏览器驱动

## 项目结构

```
rust-cucumber-demo/
├── Cargo.toml              # 项目依赖配置
├── src/
│   ├── lib.rs             # 库入口文件
│   └── page_objects/      # Page Object模式实现
│       ├── mod.rs         # 模块导出
│       └── login_page.rs  # 登录页面对象
├── features/              # Cucumber特性文件
│   └── login.feature     # 登录功能测试场景
└── tests/                 # 测试代码
    ├── cucumber.rs       # 测试运行器
    └── login_steps.rs    # 测试步骤定义
```

## 主要特性

1. Page Object模式
   - 封装页面操作
   - 提供清晰的页面接口
   - 易于维护和扩展

2. BDD测试
   - 使用Gherkin语法编写测试场景
   - 可读性强
   - 支持业务人员理解

3. 异步操作
   - 使用Rust async/await
   - 处理页面加载和元素等待

## 安装和配置

1. 安装Rust和Cargo
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 安装ChromeDriver
```bash
brew install chromedriver  # macOS
```

3. 克隆项目
```bash
git clone [项目地址]
cd rust-cucumber-demo
```

4. 安装依赖
```bash
cargo build
```

## 运行测试

1. 启动ChromeDriver
```bash
chromedriver --port=9515
```

2. 运行测试
```bash
cargo test --test cucumber
```

## 示例代码

### 特性文件 (login.feature)
```gherkin
Feature: Login functionality
    Scenario: Successful login with valid credentials
        Given I am on the login page
        When I enter username "test_user"
        And I enter password "password123"
        And I click the login button
        Then I should be logged in successfully
```

### Page Object (login_page.rs)
```rust
pub struct LoginPage {
    driver: WebDriver,
}

impl LoginPage {
    pub async fn enter_username(&self, username: &str) -> WebDriverResult<()> {
        let username_input = self.driver
            .find(By::Id("username"))
            .await?;
        username_input.send_keys(username).await?;
        Ok(())
    }
}
```

## 自定义和扩展

1. 添加新的Page Object
   - 在`src/page_objects/`下创建新的页面对象文件
   - 在`mod.rs`中导出新的页面对象

2. 添加新的测试场景
   - 在`features/`目录下创建新的特性文件
   - 在`tests/`目录下实现对应的步骤定义

3. 修改配置
   - 更新`Cargo.toml`添加新的依赖
   - 修改ChromeDriver配置
   - 自定义等待时间和重试策略

## 最佳实践

1. 页面对象设计
   - 每个页面一个独立的Page Object
   - 封装所有页面操作
   - 返回清晰的结果和错误

2. 测试场景编写
   - 遵循BDD原则
   - 保持场景简单明确
   - 使用清晰的步骤描述

3. 元素定位
   - 优先使用ID和CSS选择器
   - 避免使用XPath
   - 适当的等待策略

## 常见问题

1. ChromeDriver连接失败
   - 确保ChromeDriver已启动
   - 检查端口配置
   - 验证Chrome浏览器版本兼容性

2. 元素定位失败
   - 检查元素选择器
   - 增加等待时间
   - 确认页面是否完全加载

3. 测试不稳定
   - 添加适当的等待机制
   - 优化元素定位策略
   - 检查网络连接

## 贡献

欢迎提交Issue和Pull Request来改进这个框架。

## 许可证

MIT License
