# frozen_string_literal: true

require_relative '../setup'

class TestExtensionPopup < CapybaraTestCase
  def setup
    extension_page = "chrome-extension://#{ENV['CHROME_EXTENSION_ID']}/menu/menu.html"
    visit(extension_page)
  end

  def test_the_popup_displays_correctly
    assert_text('Discontent')
    assert_no_text('Settings')
  end

  def test_can_show_the_settings_page
    assert_text('Discontent')
    click_on('Open settings')
    assert_text('Settings')
  end
end
