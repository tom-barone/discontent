import { Link } from "./types";

export function identify(hostname: string): SearchEngine | null {
  if (hostname.includes("www.google.")) {
    return new Google();
  } else if (hostname.includes("www.bing.")) {
    return new Bing();
  } else if (hostname.includes("duckduckgo.com")) {
    return new DuckDuckGo();
  } else {
    return null;
  }
}

export function isValidHTTPURL(url: string): boolean {
  try {
    const url_obj = new URL(url);
    return url_obj.protocol === "http:" || url_obj.protocol === "https:";
  } catch (e) {
    return false;
  }
}

class SearchEngine {
  public async getAllLinks(): Promise<SearchEngineLink[]> {
    throw new Error("Not implemented");
  }
}

class Google extends SearchEngine {
  public async getAllLinks(): Promise<SearchEngineLink[]> {
    // Get all the anchor tags on the page
    const anchor_tags = document.getElementsByTagName("a");

    // Remove the google referral from the search results
    const search_links: SearchEngineLink[] = [];
    Array.from(anchor_tags).forEach((tag) => {
      // All google search results have an h3 tag below them
      const headerElement = tag.querySelector("h3");
      if (headerElement != null && isValidHTTPURL(tag.href)) {
        search_links.push(
          // Remove any google referral stuff from the url
          new SearchEngineLink(
            this.removeGoogleReferral(tag.href),
            headerElement
          )
        );
      }
    });
    return Promise.resolve(search_links);
  }

  private removeGoogleReferral(url: string): string {
    const url_obj = new URL(url);
    const params = new URLSearchParams(url_obj.search);
    return params.get("url") ?? url;
  }
}

class Bing extends SearchEngine {
  public async getAllLinks(): Promise<SearchEngineLink[]> {
    // Get all the anchor tags on the page
    const anchor_tags = document.getElementsByTagName("a");

    return Promise.allSettled(
      Array.from(anchor_tags).map(async (tag) => {
        // All bing results have no siblings and a parent h2 element
        if (
          tag.parentElement?.tagName !== "H2" ||
          tag.parentElement?.children.length !== 1 ||
          !isValidHTTPURL(tag.href)
        ) {
          return Promise.reject("Not a bing result");
        }
        // For SOME REASON bing wraps results in firefox with a referral url
        // BUT NOT for chrome! Why??
        // 	firefox: https://www.bing.com/ck/a?!&&p=...
        // 	chrome:  https://en.wikipedia.org/wiki/GitHub
        if (tag.href.includes("www.bing.com/ck")) {
          return this.fetchLinkFromBingReferral(tag.href).then(
            (link) => new SearchEngineLink(link, tag)
          );
        }
        return Promise.resolve(new SearchEngineLink(tag.href, tag));
      })
    ).then((results) => {
      return results.reduce((acc, result) => {
        if (result.status === "fulfilled") {
          acc.push(result.value);
        }
        return acc;
      }, [] as SearchEngineLink[]);
    });
  }

  private async fetchLinkFromBingReferral(
    referral_url: string
  ): Promise<string> {
    return await fetch(referral_url)
      .then((response) => response.text())
      .then((text) => {
        const search_result = text.match(/var u = "(.*)"/);
        if (search_result === null) {
          return Promise.reject("Could not find link");
        }
        if (search_result.length !== 2) {
          return Promise.reject("Regex returned unexepected results");
        }
        return search_result[1];
      });
  }
}

class DuckDuckGo extends SearchEngine {
  public async getAllLinks(): Promise<SearchEngineLink[]> {
    // Get all the anchor tags on the page
    const anchor_tags = document.getElementsByTagName("a");
    const search_links: SearchEngineLink[] = [];
    Array.from(anchor_tags).forEach((tag) => {
      // DuckDuckGo results are all under h2 tags
      if (
        tag.parentElement?.tagName === "H2" &&
        tag.parentElement?.children.length === 1 &&
        isValidHTTPURL(tag.href)
      ) {
        const headerElement = tag.children[0] as HTMLElement;
        if (headerElement != null) {
          search_links.push(
            // Remove any google referral stuff from the url
            new SearchEngineLink(tag.href, headerElement)
          );
        }
      }
    });
    return Promise.resolve(search_links);
  }
}

export class SearchEngineLink {
  private _link: Link;
  private _textElement: HTMLElement;

  constructor(url: string, textElement: HTMLElement) {
    this._link = { hostname: new URL(url).hostname };
    this._textElement = textElement;
  }

  get link(): Link {
    return this._link;
  }

  public addSymbol(symbol: string): void {
    if (this._textElement.innerText.startsWith(symbol)) {
      // Do nothing if there's already a symbol
      return;
    }
    this._textElement.innerText = `${symbol} ${this._textElement.innerText}`;
  }
}
