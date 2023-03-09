# frozen_string_literal: true

require_relative '../setup'

GOOD_ICON_SETTING_SELECTOR = 'input[data-settings-page-target="goodInput"]'
CONTROVERSIAL_ICON_SETTING_SELECTOR = 'input[data-settings-page-target="controversialInput"]'
BAD_ICON_SETTING_SELECTOR = 'input[data-settings-page-target="badInput"]'
SPINNER_SELECTOR = '[data-icon-setting-target="spinner"]'
TICK_SELECTOR = '[data-icon-setting-target="check"]'
ERROR_SELECTOR = '[data-icon-setting-target="error"]'

class TestExtensionPopup < CapybaraTestCase
  def prepare(browser)
    Capybara.current_driver = browser
    sleep(5) # Give the browser time to open

    # Create a new window because of tab weirdness and the
    # extension auto opening the help page
    new_window = open_new_window
    switch_to_window new_window
    visit(extension_popup_url)
    sleep(15) # Give the popup time to register everything
  end

  def open_settings
    sleep(5) # enough for the open animation to finish
    click_on('Open settings')
    sleep(5) # enough for the open animation to finish
  end

  def close_settings
    sleep(5) # enough for the open animation to finish
    click_on('Close settings')
    sleep(5) # enough for the open animation to finish
  end

  # Run tests for multiple browsers
  BROWSERS_TO_TEST.each do |browser|
    define_method("test_#{browser}_the_popup_displays_correctly") do
      prepare(browser)
      assert_text('Discontent')
      assert_no_text('Settings')
      assert_no_text('Settings')
      assert_button('Upvote button')
      assert_button('Downvote button')
      assert_equal find_button('Upvote button').text.strip, '💚'
      assert_equal find_button('Downvote button').text.strip, '💢'
    end

    define_method("test_#{browser}_can_show_and_hide_the_settings_page") do
      prepare(browser)
      open_settings
      assert_text('Settings')
      assert_text('Good')
      assert_text('Spicy')
      assert_text('Bad')
      assert_button('Reset')
      assert_link('Icon list')
      good_icon_input = find(GOOD_ICON_SETTING_SELECTOR)
      controversial_icon_input = find(CONTROVERSIAL_ICON_SETTING_SELECTOR)
      bad_icon_input = find(BAD_ICON_SETTING_SELECTOR)
      assert_equal good_icon_input.value, '💚'
      assert_equal controversial_icon_input.value, '🤨'
      assert_equal bad_icon_input.value, '💢'

      close_settings
      assert_no_text('Settings')
      assert_no_text('Good')
      assert_no_text('Spicy')
      assert_no_text('Bad')
      assert_no_button('Reset')
      assert_no_link('Icon list')
    end

    define_method("test_#{browser}_changing_and_resetting_the_vote_settings") do
      prepare(browser)
      open_settings
      good_icon_input = find(GOOD_ICON_SETTING_SELECTOR)
      controversial_icon_input = find(CONTROVERSIAL_ICON_SETTING_SELECTOR)
      bad_icon_input = find(BAD_ICON_SETTING_SELECTOR)
      assert_equal good_icon_input.value, '💚'
      assert_equal controversial_icon_input.value, '🤨'
      assert_equal bad_icon_input.value, '💢'

      # Set new icons
      good_icon_input.set('g')
      controversial_icon_input.set('c')
      bad_icon_input.set('b')

      # Check that 3 checkmarks appear
      assert_selector(TICK_SELECTOR, count: 3)

      # After some time they should disappear
      sleep(3)
      assert_no_selector(TICK_SELECTOR)

      assert_equal good_icon_input.value, 'g'
      assert_equal controversial_icon_input.value, 'c'
      assert_equal bad_icon_input.value, 'b'

      # Check the vote buttons show the new icons
      close_settings
      assert_equal find_button('Upvote button').text.strip, 'g'
      assert_equal find_button('Downvote button').text.strip, 'b'

      # Open settings again and reset
      open_settings
      click_on('Reset')
      assert_selector(TICK_SELECTOR, count: 3)
      sleep(3)
      assert_no_selector(TICK_SELECTOR)

      assert_equal good_icon_input.value, '💚'
      assert_equal controversial_icon_input.value, '🤨'
      assert_equal bad_icon_input.value, '💢'
      close_settings
      assert_equal find_button('Upvote button').text.strip, '💚'
      assert_equal find_button('Downvote button').text.strip, '💢'
    end

    define_method("test_#{browser}_setting_the_vote_icons_to_invalid_values") do
      prepare(browser)
      open_settings
      good_icon_input = find(GOOD_ICON_SETTING_SELECTOR)
      controversial_icon_input = find(CONTROVERSIAL_ICON_SETTING_SELECTOR)
      bad_icon_input = find(BAD_ICON_SETTING_SELECTOR)

      # Set invalid
      good_icon_input.send_keys('gg')
      controversial_icon_input.send_keys(:backspace)
      bad_icon_input.send_keys('as')

      # Check that 3 error messages show
      assert_selector(ERROR_SELECTOR, count: 3)
      # Check the error messages are correct
      assert_selector('[title="Icon must be a single character"]', count: 3)

      # Check they don't disappear
      sleep(5)
      assert_selector(ERROR_SELECTOR, count: 3)
    end

    define_method("test_#{browser}_clicking_the_icon_list_link") do
      prepare(browser)
      open_settings
      new_window = window_opened_by { click_link 'Icon list' }
      within_window new_window do
        assert_text('Emoji List, v15.0')
      end
    end

    define_method("test_#{browser}_clicking_the_help_page") do
      prepare(browser)
      new_window = window_opened_by { click_on('Open help page') }
      within_window new_window do
        assert_text('Hello friend,')
      end
    end
  end

  def teardown
    # Run javascript to clear local storage
    # Need to deal with annoying browser / chrome name differences
    page.execute_script('let browser = window.browser || window.chrome; browser.storage.local.clear()')
    sleep(5)
    super
  end
end
