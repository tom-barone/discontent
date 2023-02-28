# frozen_string_literal: true

require 'capybara/minitest'
require 'selenium-webdriver'
require 'minitest/hooks/test'

class CapybaraTestCase < Minitest::Test
  include Capybara::DSL
  include Capybara::Minitest::Assertions
  include Minitest::Hooks

  chrome_options = Selenium::WebDriver::Chrome::Options.new

  chrome_options.add_argument(
    '--disable-extensions-except=../extension/dist/chrome'
  )
  chrome_options.add_argument('--load-extension=../extension/dist/chrome')
  chrome_options.add_argument('--headless=new')
  Capybara.register_driver :chrome do |app|
    Capybara::Selenium::Driver.new(app, browser: :chrome, options: chrome_options)
  end

  Capybara.default_driver = :chrome
  Capybara.default_max_wait_time = 20 # Seconds
  Capybara.enable_aria_label = true

  def teardown
    Capybara.reset_sessions!
    Capybara.use_default_driver
  end
end
