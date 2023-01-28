export function identify(hostname: string): Webpage | null {
  if (hostname.includes("www.google.")) {
    return new Google();
  } else {
    return null;
  }
}

export class Webpage {
  public getAllLinksOnPage(): Link[] {
    throw new Error("Not implemented");
  }
}

export class Google extends Webpage {
  public getAllLinksOnPage(): Link[] {
    // Get all the anchor tags on the page
    const anchor_tags = document.getElementsByTagName("a");

    // Remove the google referral from the search results
    const search_links: Link[] = [];
    Array.from(anchor_tags).forEach((tag) => {
      // All google search results have an h3 tag below them
      const headerElement = tag.querySelector("h3");
      if (headerElement != null) {
        search_links.push(
          // Remove any google referral stuff from the url
          new Link(this.removeGoogleReferral(tag.href), headerElement)
        );
      }
    });
    return search_links;
  }

  private removeGoogleReferral(url: string): string {
    const url_obj = new URL(url);
    const params = new URLSearchParams(url_obj.search);
    return params.get("url") ?? url;
  }
}

export class Link {
  private _hostname: string;
  private _textElement: HTMLElement;

  constructor(url: string, textElement: HTMLElement) {
    this._hostname = new URL(url).hostname;
    this._textElement = textElement;
  }

  get hostname(): string {
    return this._hostname;
  }

  public addSymbol(symbol: string): void {
    if (this._textElement.innerHTML.startsWith(symbol)) {
      // Do nothing if there's already a symbol
      return;
    }
    this._textElement.innerHTML = `${symbol} ${this._textElement.innerHTML}`;
  }
}
