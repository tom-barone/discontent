import { Browser } from "webextension-polyfill";
import * as uuid from "uuid";

export type IconName = "good" | "controversial" | "bad";
export const DEFAULT_ICONS = {
  good: "💚",
  controversial: "🤨",
  bad: "❌",
};

export class Settings {
  private readonly _browser: Browser;

  constructor(browser: Browser) {
    this._browser = browser;
  }

  async get_icons() {
    return Promise.all([
      this.get_icon("good"),
      this.get_icon("controversial"),
      this.get_icon("bad"),
    ]).then((values) => {
      return {
        good: values[0],
        controversial: values[1],
        bad: values[2],
      };
    });
  }

  async get_icon(icon_name: IconName) {
    return this._browser.storage.local.get(icon_name).then(async (settings) => {
      if (is_valid_emoji(settings[icon_name])) {
        return settings[icon_name] as string;
      } else {
        // Save the default icon to storage if it's not valid
        return this.set_icon(icon_name, DEFAULT_ICONS[icon_name]);
      }
    });
  }

  async set_icon(icon_name: IconName, icon: string) {
    if (!is_valid_emoji(icon)) {
      return Promise.reject("Icon must be a single character");
    }
    return this._browser.storage.local
      .set({ [icon_name]: icon })
      .then(() => icon);
  }

  async get_user_id() {
    return this._browser.storage.local.get("user_id").then((settings) => {
      if (uuid.validate(settings["user_id"])) {
        return settings["user_id"];
      } else {
        // Save a newly generated user ID to storage if it's not valid
        return this.set_user_id(uuid.v4());
      }
    });
  }

  async set_user_id(user_id: string) {
    if (!uuid.validate(user_id)) {
      return Promise.reject("User ID must be a valid UUID");
    }
    return this._browser.storage.local
      .set({ ["user_id"]: user_id })
      .then(() => user_id);
  }
}

function is_valid_emoji(emoji: string | undefined): boolean {
  // https://stackoverflow.com/questions/54369513/how-to-count-the-correct-length-of-a-string-with-emojis-in-javascript
  return (
    emoji !== undefined && typeof emoji === "string" && [...emoji].length == 1
  );
}
