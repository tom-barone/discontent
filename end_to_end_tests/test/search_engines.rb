# frozen_string_literal: true

require_relative '../setup'

class TestSearchEngines < CapybaraTestCase
  def check_search(domain)
    visit("https://#{domain}=site%3Aen.wikipedia.org")
    refresh
    assert_text(:all, /💚 .+/)
    visit("https://#{domain}=site%3Agithub.com")
    refresh
    assert_text(:all, /🤨 .+/)
    visit("https://#{domain}=site%3Atwitter.com")
    refresh
    assert_text(:all, /❌ .+/)
  end

  def prepare(browser)
    Capybara.current_driver = browser
    sleep(5) # Give the browser some time to load

    # Create a new window for the visits because of tab weirdness and the
    # extension auto opening the help page
    new_window = open_new_window
    switch_to_window new_window
    sleep(5)
  end

  BROWSERS_TO_TEST.each do |browser|
    define_method("test_#{browser}_google_displays_icons_on_links") do
      prepare(browser)
      # Google
      check_search('www.google.com/search?q')
      check_search('www.google.it/search?q')
      check_search('www.google.com.au/search?q')
    end
    define_method("test_#{browser}_bing_displays_icons_on_links") do
      prepare(browser)
      # Bing
      check_search('www.bing.com/search?q')
    end
    define_method("test_#{browser}_duckduckgo_displays_icons_on_links") do
      prepare(browser)
      # DuckDuckGo
      check_search('duckduckgo.com/?q')
      check_search('html.duckduckgo.com/html/?q')
    end
  end
end
