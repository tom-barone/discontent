import * as browser from "webextension-polyfill";
import { Controller } from "@hotwired/stimulus";
import { Settings } from "../../settings";

export default class extends Controller {
  static values = {
    vote: String,
  };
  declare readonly voteValue: "good" | "bad";
	declare settings: Settings;

  connect() {
		this.settings = new Settings(browser);
    this.load_icon();
  }

  load_icon() {
    this.settings.get_icon(this.voteValue).then((icon) => {
      this.element.innerHTML = `<div>${icon}</div>`;
      this.element.classList.remove("invisible");
    });
  }
}
