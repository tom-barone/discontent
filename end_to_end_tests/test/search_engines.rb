# frozen_string_literal: true

require_relative '../setup'

class TestSearchEngines < CapybaraTestCase
  def check_search(domain)
    visit("https://#{domain}=site%3Aen.wikipedia.org")
    assert_text(:all, /💚 .+/)
    visit("https://#{domain}=site%3Agithub.com")
    assert_text(:all, /🤨 .+/)
    visit("https://#{domain}=site%3Atwitter.com")
    assert_text(:all, /💢 .+/)
  end

  def select_driver(browser)
    Capybara.current_driver = browser
    sleep(5) # Give the browser some time to load
  end

  %i[chrome firefox].each do |browser|
    define_method("test_#{browser}_google_displays_icons_on_links") do
      select_driver(browser)
      # Google
      check_search('www.google.com/search?q')
      check_search('www.google.it/search?q')
      check_search('www.google.com.au/search?q')
    end
    define_method("test_#{browser}_bing_displays_icons_on_links") do
      select_driver(browser)
      # Bing
      check_search('www.bing.com/search?q')
    end
    define_method("test_#{browser}_duckduckgo_displays_icons_on_links") do
      select_driver(browser)
      # DuckDuckGo
      check_search('duckduckgo.com/?q')
    end
  end
end
