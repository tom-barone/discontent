import { Controller } from "@hotwired/stimulus";
import * as settings from "../../settings";

export default class extends Controller {
  static values = {
    vote: String,
  };
  declare readonly voteValue: "good" | "bad";

  connect() {
    this.load_icon();
  }

  load_icon() {
    settings.get_icon(this.voteValue).then((icon) => {
      this.element.innerHTML = `<div>${icon}</div>`;
      this.element.classList.remove("invisible");
    });
  }
}
