Feature: Login functionality
  As a user
  I want to be able to log in to the application
  So that I can access my account

  Background:
    Given I am on the login page

  Scenario: Failed login with invalid credentials
    When I enter username "test_user_invalid"
    And I enter password "wrong_password"
    And I click the login button
    Then I should see an error message
    And the error message should contain "Your username is invalid!"

  Scenario: Failed login with valid username but wrong password
    When I enter username "tomsmith"
    And I enter password "wrong_password"
    And I click the login button
    Then I should see an error message
    And the error message should contain "Your password is invalid!"

  Scenario: Failed login with empty credentials
    When I click the login button
    Then I should see an error message
    And the error message should contain "Your username is invalid!"

  Scenario: Failed login with empty password
    When I enter username "tomsmith"
    And I click the login button
    Then I should see an error message
    And the error message should contain "Your password is invalid!"

  Scenario: Successful login with valid credentials
    When I enter username "tomsmith"
    And I enter password "SuperSecretPassword!"
    And I click the login button
    Then I should be logged in successfully
    And I should see the secure area
