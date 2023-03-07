import * as browser from "webextension-polyfill";
import { Controller } from "@hotwired/stimulus";

export default class extends Controller<HTMLButtonElement> {
  open_help_page() {
    browser.tabs.create({
      url: browser.runtime.getURL("/help/help.html"),
    });
  }
}
