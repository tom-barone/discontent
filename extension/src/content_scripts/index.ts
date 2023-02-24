import { identify } from "../search_engine";
import * as browser from "webextension-polyfill";
import { ScoresRequest, ScoresResponseMessage } from "../types";

/* The flow of the content script is quite simple:
 * 	 1. Check if the content script has already run
 * 	 2. Find out which search engine we're on (if any)
 * 	 3. Get all the search engine links
 * 	 4. Send the links to the background script and request their scores
 * 	 5. Add the scores to the links
 */
(() => {
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
  const search_engine_links = search_engine.getAllLinks();
	if (search_engine_links.length === 0) {
		return;
	}

  // Step 4
  browser.runtime
    .sendMessage({
      type: "ScoresRequest",
      data: new ScoresRequest(search_engine_links),
    })
    .then((message: ScoresResponseMessage) => {
      const scoresResponse = message.data;
      // Step 5
      search_engine_links.forEach((search_engine_link) => {
        switch (scoresResponse.get(search_engine_link.link.hostname)) {
          case "Good":
            search_engine_link.addSymbol("💚");
            break;
          case "Bad":
            search_engine_link.addSymbol("💢");
            break;
          case "Controversial":
            search_engine_link.addSymbol("🤨");
            break;
          case "NoScore":
          default:
          // Do nothing
        }
      });
    });
})();

declare global {
  // So typescript doesn't complain about tiny `hasRun` boolean
  interface Window {
    hasRun: boolean;
  }
}
