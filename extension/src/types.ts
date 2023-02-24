import { SearchEngineLink } from "./search_engine";

export interface ScoresRequestMessage {
  type: "ScoresRequest";
  data: ScoresRequest;
}

export interface ScoresResponseMessage {
  type: "ScoresResponse";
  data: ScoresResponse;
}

export type Message = ScoresRequestMessage | ScoresResponseMessage;

export enum Score {
  Good = "Good",
  Bad = "Bad",
  Controversial = "Controversial",
  NoScore = "NoScore",
}

export interface Link {
  hostname: string;
}

export interface LinkScore {
  link: Link;
  score: Score;
}

export class ScoresRequest {
  public json: string;

  constructor(search_engine_links: SearchEngineLink[]) {
    // Use a set so that duplicate links are removed
    const links = new Set(
      search_engine_links.map(
        (search_engine_link) => search_engine_link.link.hostname
      )
    );
    this.json = JSON.stringify({
      links: [...links].map((link) => ({ hostname: link })),
    });
  }
}

export type ScoresResponse = Map<string, Score>;
