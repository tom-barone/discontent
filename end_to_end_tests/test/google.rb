# frozen_string_literal: true

require_relative '../setup'

class TestGoogle < CapybaraTestCase
  # Right now doesn't work on Firefox
  %i[chrome].each do |browser|
    define_method("test_#{browser}_displays_icons_on_links") do
      Capybara.current_driver = browser
      visit('https://www.google.com/search?q=github+en.wikipedia+twitter')
      assert_text(:all, /💚 .+/)
      assert_text(:all, /🤨 .+/)
      assert_text(:all, /💢 .+/)
    end
  end
end
