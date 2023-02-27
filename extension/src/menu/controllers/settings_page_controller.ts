import { Controller } from "@hotwired/stimulus";
import * as settings from "../../settings";

export default class extends Controller {
  static targets = ["goodInput", "controversialInput", "badInput"];
  declare readonly goodInputTarget: HTMLInputElement;
  declare readonly controversialInputTarget: HTMLInputElement;
  declare readonly badInputTarget: HTMLInputElement;

  reset() {
    this.goodInputTarget.value = settings.DEFAULT_ICONS.good;
    this.controversialInputTarget.value = settings.DEFAULT_ICONS.controversial;
    this.badInputTarget.value = settings.DEFAULT_ICONS.bad;

		this.goodInputTarget.dispatchEvent(new Event('input'));
		this.controversialInputTarget.dispatchEvent(new Event('input'));
		this.badInputTarget.dispatchEvent(new Event('input'));
  }
}