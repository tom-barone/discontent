import { identify } from "./webpage";

export {};

declare global {
  interface Window {
    hasRun: boolean;
  }
}

(() => {
  // Check if the content script has already run
  if (window.hasRun) {
    return;
  }
  window.hasRun = true;

  // Find out which webpage we're on
  const webpage = identify(window.location.hostname);
  // ...if we don't know, do nothing
  if (webpage == null) {
    return;
  }

  const links = webpage.getAllLinksOnPage();

  links.forEach((link) => {
		//link.addSymbol("❌");
		link.addSymbol("💢");
		//link.addSymbol("💚");
    //link.addSymbol("🤨");
  });
})();
