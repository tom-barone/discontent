// Get the type of the browser object from the polyfill
import { Browser } from "webextension-polyfill";
// Mock out the entire polyfill for testing
import "jest-webextension-mock";
declare const browser: Browser;

import { Settings, IconName } from "./settings";
import * as uuid from "uuid";

describe("Settings", () => {
  let settings: Settings;

  beforeEach(() => {
    settings = new Settings(browser);
  });

  test("Getting a non-existant user_id generates a new one", async () => {
    settings.get_user_id().then((newly_generated_id) => {
      expect(uuid.validate(newly_generated_id)).toBe(true);

      // More requests for the user_id should return the same value
      for (let i = 0; i < 3; i++) {
        expect(settings.get_user_id()).resolves.toEqual(newly_generated_id);
      }
    });
  });

  test("Getting an existing user_id", async () => {
    expect(
      settings.set_user_id("e2f50e34-203a-4fc3-9952-7029bfe65838")
    ).resolves.toEqual("e2f50e34-203a-4fc3-9952-7029bfe65838");

    for (let i = 0; i < 3; i++) {
      expect(settings.get_user_id()).resolves.toEqual(
        "e2f50e34-203a-4fc3-9952-7029bfe65838"
      );
    }
  });

  test("Setting an invalid user_id fails", async () => {
    const user_id = await settings.get_user_id();

    expect(settings.set_user_id("[;.[;")).rejects.toEqual(
      "User ID must be a valid UUID"
    );

    expect(settings.get_user_id()).resolves.toEqual(user_id);
  });

  test("Getting icons returns the default values", async () => {
    // Do it a few times to make sure
    for (let i = 0; i < 3; i++) {
      expect(settings.get_icon("good")).resolves.toEqual("💚");
      expect(settings.get_icon("controversial")).resolves.toEqual("🤨");
      expect(settings.get_icon("bad")).resolves.toEqual("💢");
    }
  });

  test("Getting icons returns the existing values", async () => {
    expect(settings.set_icon("good", "a")).resolves.toEqual("a");
    expect(settings.set_icon("controversial", "b")).resolves.toEqual("b");
    expect(settings.set_icon("bad", "c")).resolves.toEqual("c");

    // Do it a few times to make sure
    for (let i = 0; i < 3; i++) {
      expect(settings.get_icon("good")).resolves.toEqual("a");
      expect(settings.get_icon("controversial")).resolves.toEqual("b");
      expect(settings.get_icon("bad")).resolves.toEqual("c");
    }
  });

  test("Setting incorrect icons fails", async () => {
    expect(settings.set_icon("good", "a")).resolves.toEqual("a");
    expect(settings.set_icon("controversial", "b")).resolves.toEqual("b");
    expect(settings.set_icon("bad", "c")).resolves.toEqual("c");

    ["good", "controversial", "bad"].forEach((icon_name) => {
      expect(settings.set_icon(icon_name as IconName, "123")).rejects.toEqual(
        "Icon must be a single character"
      );
    });

    // Check our original values are still the same
    expect(settings.get_icon("good")).resolves.toEqual("a");
    expect(settings.get_icon("controversial")).resolves.toEqual("b");
    expect(settings.get_icon("bad")).resolves.toEqual("c");
  });
});
