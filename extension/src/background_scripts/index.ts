import * as browser from "webextension-polyfill";
import { fetchScores } from "../api";
import { Message } from "../types";

browser.runtime.onMessage.addListener(
  async (message: Message): Promise<Message> => {
    switch (message.type) {
      case "ScoresRequest":
        return fetchScores(message.data).then((scores) => ({
          type: "ScoresResponse",
          data: scores,
        }));
      default:
      // TODO proper error handling
    }
    return Promise.reject("Unknown message type");
  }
);
