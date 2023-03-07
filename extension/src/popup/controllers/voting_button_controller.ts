import * as browser from "webextension-polyfill";
import { Controller } from "@hotwired/stimulus";
import { Settings } from "../../settings";
import { submitVote } from "../../api";

const FADE_IN_AND_OUT_TIME = 200; // milliseconds
const FADE_OUT_CHECK_AFTER = 1; // seconds

export default class extends Controller<HTMLButtonElement> {
  // TODO: Add displaying of errors somehow
  static targets = ["icon", "spinner", "check"];
  static values = {
    vote: String,
  };
  declare readonly voteValue: "good" | "bad";
  declare readonly iconTarget: HTMLDivElement;
  declare readonly spinnerTarget: HTMLDivElement;
  declare readonly checkTarget: HTMLElement;
  declare settings: Settings;
  declare timer: NodeJS.Timeout;

  connect() {
    this.settings = new Settings(browser);
    this.load_icon();
  }

  load_icon() {
    this.settings.get_icon(this.voteValue).then((icon) => {
      this.iconTarget.innerText = icon;
      this.element.classList.remove("invisible");
    });
  }

  submit() {
    this.element.disabled = true;
    this._showSpinner();
    Promise.all([
      // Get the current user_id
      this.settings.get_user_id(),
      // Get the current tab's URL
      browser.tabs.query({ active: true, currentWindow: true }).then((tabs) => {
        if (tabs.length === 0 || tabs[0].url === undefined) {
          return Promise.reject(new Error("No active tab found"));
        }
        const hostname = new URL(tabs[0].url).hostname;
        if (hostname === "") {
          return Promise.reject(
            new Error(`No hostname found found for ${tabs[0].url}`)
          );
        }
        return hostname;
      }),
    ])
      .then(([user_id, hostname]) => {
        const vote_value = this.voteValue === "good" ? 1 : -1;
        return submitVote(vote_value, hostname, user_id);
      })
      .catch((error) => {
        console.log(error);
      })
      .finally(() => {
        this._showCheck();
        this.element.disabled = false;
      });
  }

  _showSpinner() {
    this.iconTarget.classList.add("d-none");
    this.checkTarget.classList.add("d-none");
    this._fadeIn(this.spinnerTarget);
  }

  _showCheck() {
    // The check will disappear after some time
    this.spinnerTarget.classList.add("d-none");
    this.iconTarget.classList.add("d-none");
    this._fadeIn(this.checkTarget);

    // Fade back in the icon
    this.timer && clearTimeout(this.timer);
    this.timer = setTimeout(() => {
      this.checkTarget.animate(
        [{ opacity: 1 }, { opacity: 0 }],
        FADE_IN_AND_OUT_TIME
      );
      setTimeout(() => {
        this.checkTarget.classList.add("d-none");
        this._fadeIn(this.iconTarget);
      }, FADE_IN_AND_OUT_TIME);
    }, FADE_OUT_CHECK_AFTER * 1000);
  }

  _fadeIn(element: HTMLElement) {
    element.animate([{ opacity: 0 }, { opacity: 1 }], FADE_IN_AND_OUT_TIME);
    element.classList.remove("d-none");
  }
}
