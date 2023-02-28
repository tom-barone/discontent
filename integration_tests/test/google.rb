# frozen_string_literal: true

require_relative '../setup'

class TestGoogle < CapybaraTestCase
  def test_can_display_some_icons
    visit('https://www.google.com/search?q=difference%20between%20reddit%20and%20twitter')
    assert_text(/What.+/)
    assert_text(/💚.+/)
    # assert_text(/🤔.+/)
    # assert_text(/💚.+/)
    # assert_text(/💢.+/)
  end
end
