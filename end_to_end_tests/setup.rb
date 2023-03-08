# frozen_string_literal: true

require 'capybara/minitest'
require 'date'
require 'selenium-webdriver'

class CapybaraTestCase < Minitest::Test
  include Capybara::DSL
  include Capybara::Minitest::Assertions

  # Chrome setup
  chrome_options = Selenium::WebDriver::Chrome::Options.new
  chrome_options.add_argument(
    '--disable-extensions-except=../extension/dist/chrome'
  )
  chrome_options.add_argument('--load-extension=../extension/dist/chrome')
  chrome_options.add_argument('--headless=new') if ENV['HEADLESS'] == 'true'
  chrome_options.add_option('goog:loggingPrefs', { browser: 'ALL' })
  Capybara.register_driver :chrome do |app|
    driver = Capybara::Selenium::Driver.new(app, browser: :chrome, options: chrome_options)
    driver.browser.manage.window.resize_to(1920, 1080)
    driver
  end

  # Firefox setup
  firefox_options = Selenium::WebDriver::Firefox::Options.new
  firefox_options.add_argument('-headless') if ENV['HEADLESS'] == 'true'
  Capybara.register_driver :firefox do |app|
    driver = Capybara::Selenium::Driver.new(app, browser: :firefox, options: firefox_options)
    driver.browser.manage.window.resize_to(1920, 1080)
    driver.browser.install_addon('../extension/dist/firefox', true)
    driver
  end

  Capybara.default_driver = :chrome
  Capybara.default_max_wait_time = 20 # Seconds
  Capybara.enable_aria_label = true

  def extension_popup_url
    case Capybara.current_driver
    when :chrome
      return "chrome-extension://#{ENV['CHROME_EXTENSION_ID']}/popup/popup.html"
    when :firefox
      prefs_file = "#{page.driver.browser.capabilities['moz:profile']}/prefs.js"
      # Extract the webextensions uuids
      uuid_regex = /user_pref\("extensions.webextensions.uuids", "(.+)"\)/
      uuid_match = uuid_regex.match(File.read(prefs_file))[1]
      # Remove all the backslashes
      uuid_match.gsub!('\\', '')
      # Parse the uuid_match as JSON
      uuids = JSON.parse(uuid_match)
      return "moz-extension://#{uuids[ENV['FIREFOX_EXTENSION_ID']]}/popup/popup.html"
    end
    throw 'Unknown driver'
  end

  def teardown
    # Take a screenshot and save the page html if the test failed
    timestamp = Time.now.strftime('%Y_%m_%d-%H_%M_%S')
    filename = "#{name}-#{timestamp}"
    unless passed?
      save_screenshot("ci/screenshots/#{filename}.png")
      File.write("ci/screenshots/#{filename}.html", page.html)
      if defined?(page.driver.browser.logs)
        File.write("ci/screenshots/#{filename}-console-logs.log",
                   page.driver.browser.logs.get(:browser))
      end
    end

    Capybara.reset_sessions!
    Capybara.use_default_driver
  end
end
