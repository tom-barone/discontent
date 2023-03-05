import * as browser from "webextension-polyfill";
import { identify } from "../search_engine";
import { ScoresRequest, ScoresResponseMessage } from "../types";
import { Settings } from "../settings";

/* The flow of the content script is quite simple:
 * 	 1. Check if the content script has already run
 * 	 2. Find out which search engine we're on (if any)
 * 	 3. Get all the search engine links
 * 	 4. Send the links to the background script and request their scores
 * 	 5. Add the scores to the links
 */
window.onload = function () {
  // Step 1
  if (window.hasRun) {
    return;
  }
  window.hasRun = true;

  // Step 2
  const search_engine = identify(window.location.hostname);
  if (search_engine == null) {
    // Just do nothing if we're not on a supported search engine
    return;
  }

  // Step 3
  search_engine
    .getAllLinks()
    .then((search_engine_links) => {
      if (search_engine_links.length === 0) {
        return;
      }
      // Step 4
      const settings = new Settings(browser);
      return Promise.all([
        settings.get_icons(),
        browser.runtime.sendMessage({
          type: "ScoresRequest",
          data: new ScoresRequest(search_engine_links),
        }),
      ]).then(([icons, message]) => {
        const scoresResponse = (message as ScoresResponseMessage).data;
        // Step 5
        search_engine_links.forEach((search_engine_link) => {
          switch (scoresResponse[search_engine_link.link.hostname]) {
            case "Good":
              search_engine_link.addSymbol(icons.good);
              break;
            case "Controversial":
              search_engine_link.addSymbol(icons.controversial);
              break;
            case "Bad":
              search_engine_link.addSymbol(icons.bad);
              break;
            case "NoScore":
            default:
            // Do nothing
          }
        });
      });
    })
    .catch((error) => {
      // TODO: Handle the error somehow
      console.error(error);
    });
}
